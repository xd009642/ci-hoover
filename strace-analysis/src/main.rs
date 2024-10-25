use std::fs;
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
            // Do a cheeky filter out of files in directory
            if line.contains("rmdir") {
                result.retain(|x| !x.starts_with(&tmp));
            }
            result.push(tmp);
        }
    }
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

    let data = ron::to_string(&result).expect("Unable to write RON");
    fs::write("../res/delete_list.ron", data).expect("Failed to save file");
}
