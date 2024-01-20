use std::{fs, path::PathBuf};

use crate::run_command;

pub fn patch(patches: &PathBuf, patched: &PathBuf) {
    let last_id = fs::read_dir(patches)
        .unwrap()
        .map(|r| r.unwrap())
        .filter(|entry| entry.file_type().unwrap().is_file())
        .map(|entry| entry.file_name().to_str().unwrap().to_string())
        .filter(|name| name.ends_with(".patch"))
        .filter_map(|x| {
            x.split_once('-')
                .and_then(|(num, _)| num.parse::<usize>().ok())
        })
        .max()
        .unwrap_or(0);

    println!("creating patch {}", last_id + 1);

    run_command(
        format!(
            "git format-patch --start-number {id} -n HEAD^ -o ../{patches}",
            id = last_id + 1,
            patches = patches.to_str().unwrap()
        ),
        patched,
    );
}
