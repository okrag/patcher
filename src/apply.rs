use std::{fs, path::PathBuf};

use crate::{copy_files, run_command};

pub fn apply(patches: &PathBuf, patched: &PathBuf, base: &PathBuf) {
    let _ = fs::remove_dir_all(patched);
    fs::create_dir(patched).unwrap();

    copy_files(&base.into(), &patched.into());

    run_command("git init", patched);
    run_command("git add .", patched);
    run_command("git commit -m \"__init__\"", patched);

    let mut entries = fs::read_dir(patches)
        .unwrap()
        .map(|r| r.unwrap())
        .filter(|entry| entry.file_type().unwrap().is_file())
        .map(|entry| entry.file_name().to_str().unwrap().to_string())
        .filter(|name| name.ends_with(".patch"))
        .filter_map(|x| {
            x.split_once('-')
                .and_then(|(num, _)| num.parse::<usize>().ok())
                .map(|num| (num, x))
        })
        .collect::<Vec<_>>();

    entries.sort_by(|a, b| a.0.cmp(&b.0));

    for (_, entry) in entries {
        run_command(
            format!(
                "git apply ../{patches}/{entry}",
                patches = patches.to_str().unwrap()
            ),
            patched,
        );
    }

    run_command("git add .", patched);
    run_command("git commit -m \"__patches__\"", patched);
}
