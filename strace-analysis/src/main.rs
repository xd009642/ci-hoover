use std::process::Command;

fn process_strace_output(output: &[u8]) -> Vec<String> {
    let string = String::from_utf8_lossy(output);

    for line in string.lines() {
        if line.endswith("= 0") && (line.contains("unlink") || line.contains("rmdir")) {
            println!("{}", line);
        }
    }
    vec![]
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
    remove_packages(&["'^aspnetcore-.*'"]);
    remove_packages(&["'^dotnet-.*'"]);
    remove_packages(&["'^llvm-.*'"]);
    remove_packages(&["'php.*'"]);
    remove_packages(&[
        "azure-cli",
        "google-chrome-stable",
        "firefox",
        "powershell",
        "mono-devel",
        "libgl1-mesa-dri",
    ]);
    remove_packages(&["google-cloud-sdk"]);
    remove_packages(&["google-cloud-cli"]);

    cleanup();
}
