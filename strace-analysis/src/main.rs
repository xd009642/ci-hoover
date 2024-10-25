use ron::ser::*;
use std::fs;
use std::path::MAIN_SEPARATOR_STR;
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
    println!("Got: {:?}", result);
    result
}

fn remove_packages(packages: &[&str]) -> Vec<String> {
    println!("Removing: {:?}", packages);
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

fn main() {
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

    let data = to_string_pretty(&result, PrettyConfig::new()).expect("Unable to write RON");
    fs::write("res/delete_list.ron", data).expect("Failed to save file");
}
