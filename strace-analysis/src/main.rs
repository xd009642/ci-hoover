use ron::ser::*;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, MAIN_SEPARATOR_STR};
use std::process::Command;

fn process_strace_output(output: &[u8]) -> Vec<String> {
    let string = String::from_utf8_lossy(output);
    let mut result = vec![];

    for line in string.lines() {
        if line.ends_with("= 0") && (line.contains("unlink") || line.contains("rmdir")) {
            let mut tmp = line.to_string();
            let mut keep = false;
            tmp.retain(|c| {
                if c == '(' {
                    keep = true;
                    false
                } else if c == ')' {
                    keep = false;
                    false
                } else {
                    keep
                }
            });
            for maybe_path in tmp.split(",").filter(|x| x.contains(MAIN_SEPARATOR_STR)) {
                let tmp = maybe_path
                    .trim()
                    .trim_start_matches("\"")
                    .trim_end_matches("\"")
                    .to_string();

                // Do a cheeky filter out of files in directory
                if line.contains("rmdir") {
                    result.retain(|x: &String| !x.starts_with(&tmp));
                }
                if !tmp.is_empty() {
                    result.push(tmp);
                }
            }
        }
    }
    result.dedup();
    result
}

fn remove_packages(packages: &[&str]) -> Vec<String> {
    let strace_out = Command::new("strace")
        .args(["-e", "trace=%file", "-f", "apt-get", "remove", "-y"])
        .args(packages)
        .arg("--fix-missing")
        .output()
        .expect("Failed to strace apt-get");

    process_strace_output(&strace_out.stderr)
}

fn cleanup() -> Vec<String> {
    let strace_out = Command::new("strace")
        .args(["-e", "trace=%file", "-f", "apt-get", "autoremove", "-y"])
        .output()
        .expect("Failed to strace apt-get");

    process_strace_output(&strace_out.stderr)
}

fn compress_deletions(inputs: Vec<String>) -> Vec<String> {
    let mut set = BTreeSet::new();
    set.insert("/var/cache".to_string());

    for file in inputs.iter() {

        let path = Path::new(&file);
        if set.iter().any(|x| path.starts_with(x)) {
            continue;
        }

        let mut parent = match path.parent() {
            Some(s) => s,
            None => continue,
        };
        if parent.exists() {
            let parent_is_empty = fs::read_dir(parent).map(|mut x| x.next().is_none()).unwrap_or(false);
            if parent_is_empty {
                println!("Removing parent: {}", parent.display());
                set.insert(parent.display().to_string());
            } else {
                println!("Removing file: {}", file);
                set.insert(file.clone());
            }
        } else {
            while let Some(new_parent) = parent.parent() {
                if new_parent.exists() {
                    println!("Reduced {} to {}", path.display(), parent.display());
                    set.insert(parent.display().to_string());
                    break;
                } else {
                    println!("Going from {} to {}", parent.display(), new_parent.display());
                    parent = new_parent
                }
            }
        }
    }
    set.into_iter().collect()
}

fn main() {
    // Credit should go to pr2502 for thinking of this idea on how to make my apt removals go
    // brrrrrr

    let mut result = vec![];
    result.append(&mut remove_packages(&["'^aspnetcore-.*'"]));
    result.append(&mut remove_packages(&["'^dotnet-.*'"]));
    result.append(&mut remove_packages(&["'^llvm-.*'"]));
    result.append(&mut remove_packages(&["'php.*'"]));
    result.append(&mut remove_packages(&[
        "azure-cli",
        "google-chrome-stable",
        "firefox",
        "powershell",
        "mono-devel",
        "libgl1-mesa-dri",
    ]));
    result.append(&mut remove_packages(&["google-cloud-sdk"]));
    result.append(&mut remove_packages(&["google-cloud-cli"]));

    result.append(&mut cleanup());

    result.sort();
    result.dedup();

    let result = compress_deletions(result);

    let data = to_string_pretty(&result, PrettyConfig::new()).expect("Unable to write RON");
    fs::write("res/delete_list.ron", data).expect("Failed to save file");
}
