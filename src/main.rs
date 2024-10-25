use bytesize::ByteSize;
use rayon::prelude::*;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Instant;
use sysinfo::Disks;

const fn true_value() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(default = "true_value")]
    android: bool,
    #[serde(default = "true_value")]
    dot_net: bool,
    #[serde(default = "true_value")]
    haskell: bool,
    #[serde(default = "true_value")]
    large_packages: bool,
    #[serde(default = "true_value")]
    docker_images: bool,
    #[serde(default = "true_value")]
    tools_cache: bool,
    #[serde(default = "true_value")]
    swap_storage: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            android: true,
            dot_net: true,
            haskell: true,
            large_packages: true,
            docker_images: true,
            tools_cache: true,
            swap_storage: true,
        }
    }
}

impl Config {
    const HASKELL: [&'static str; 2] = ["/opt/ghc", "/usr/local/.ghcup"];
    const ANDROID: [&'static str; 1] = ["/usr/local/lib/android"];
    const DOT_NET: [&'static str; 1] = ["/usr/share/dotnet"];

    fn get_paths(&self) -> Vec<PathBuf> {
        let mut res = vec![];
        if self.android {
            res.extend(Self::ANDROID.iter().map(PathBuf::from));
        }
        if self.dot_net {
            res.extend(Self::DOT_NET.iter().map(PathBuf::from));
        }
        if self.haskell {
            res.extend(Self::HASKELL.iter().map(PathBuf::from));
        }
        if self.tools_cache {
            match env::var("AGENT_TOOLSDIRECTORY") {
                Ok(s) => res.push(PathBuf::from(s)),
                Err(_) => (),
            }
        }
        if self.swap_storage {
            let _ = Command::new("swapoff").arg("-a").output();
            res.push(PathBuf::from("/mnt/swapfile"));
        }
        res
    }
}

fn print_summary(disks: &Disks) {
    println!("Name\tTotal\tUsed\tAvailable");
    for disk in disks.list() {
        let total = disk.total_space();
        let available = disk.available_space();
        println!(
            "{:?}\t{}\t{}\t{}",
            disk.name(),
            ByteSize(total),
            ByteSize(total - available),
            ByteSize(available)
        );
    }
}

fn remove_path(x: impl AsRef<Path>) {
    if x.as_ref().is_file() {
        let _ = fs::remove_file(x);
    } else {
        let _ = fs::remove_dir_all(x);
    }
}

fn main() {
    let start = Instant::now();
    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    };
    let mut disks = Disks::new_with_refreshed_list();
    print_summary(&disks);

    let do_docker = config.docker_images;

    let docker_thread = thread::spawn(move || {
        if do_docker {
            let _ = Command::new("docker")
                .args(["image", "prune", "--all", "--force"])
                .output();
        }
    });

    let mut ready_list: Vec<PathBuf> = if config.large_packages {
        match ron::de::from_bytes(include_bytes!("../res/delete_list.ron").as_slice()) {
            Ok(v) => v,
            Err(e) => {
                println!("Delete list invalid: {}", e);
                vec![]
            }
        }
    } else {
        vec![]
    };

    println!("Deleting things!");

    ready_list.append(&mut config.get_paths());

    let _: Vec<_> = ready_list.par_iter().map(|x| remove_path(x)).collect();

    // Join docker thread
    let _ = docker_thread.join();

    // Going to assume a disk hasn't been added/removed
    disks.refresh();
    print_summary(&disks);

    println!("Finished in {} seconds", start.elapsed().as_secs_f32());
}
