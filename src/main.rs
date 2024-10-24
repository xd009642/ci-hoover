use bytesize::ByteSize;
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use sysinfo::Disks;

pub struct Config {
    android: bool,
    dot_net: bool,
    haskell: bool,
    large_packages: bool,
    docker_images: bool,
    tools_cache: bool,
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

fn main() {
    let start = Instant::now();
    let config = Config::default();
    let mut disks = Disks::new_with_refreshed_list();
    print_summary(&disks);

    println!("Deleting things!");

    let _: Vec<_> = config
        .get_paths()
        .par_iter()
        .map(|x| fs::remove_dir_all(x))
        .collect();

    // Going to assume a disk hasn't been added/removed
    disks.refresh();
    print_summary(&disks);

    println!("Finished in {} seconds", start.elapsed().as_secs_f32());
}
