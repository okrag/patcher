pub mod apply;
pub mod patch;

use crate::apply::apply;
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

use clap::{Parser, Subcommand};
use patch::patch;

fn run_command<'a, T: Into<String>, P: Into<PathBuf>>(cmd: T, cwd: P) {
    let cmd: String = cmd.into();
    let mut args = cmd.split_whitespace();
    let output = Command::new(args.next().unwrap())
        .current_dir(cwd.into())
        .args(args)
        .output()
        .unwrap();

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

fn copy_files(src: &PathBuf, dst: &PathBuf) {
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let src = entry.path();
        let dst = dst.join(src.file_name().unwrap());
        if src.is_dir() {
            fs::create_dir(&dst).unwrap();
            copy_files(&src, &dst);
        } else {
            fs::copy(&src, &dst).unwrap();
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// applys the patches
    Apply {
        /// patches path
        #[arg(short, long)]
        patches: PathBuf,

        /// path to the patched output
        #[arg(short, long)]
        patched: PathBuf,

        /// path to unpatched input
        #[arg(short, long)]
        base: PathBuf,
    },
    /// creates a patch from the latest commit
    Patch {
        /// patches path
        #[arg(short, long)]
        patches: PathBuf,

        /// path to the patched output
        #[arg(short, long)]
        patched: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Apply {
            patches,
            patched,
            base,
        } => apply(patches, patched, base),
        Commands::Patch { patches, patched } => patch(patches, patched),
    }
}
