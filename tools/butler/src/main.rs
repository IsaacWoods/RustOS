/*
 * TODO: this is going to be Pebble's all-in-one building, testing, hosting big-mess-of-stuff application. You use
 * it on a host to create a Pebble distribution and pave it onto a target, either real (e.g. over a network) or a
 * VM on the host.
 *
 * - Read a config file (e.g. Pebble.toml) to specify options
 * - Build a Pebble distribution
 *      - Build a custom Rust toolchain
 *      - Compile a firmware if needed
 *      - Compile all the things - graph of Steps
 *      - Create an image and put all the bits in
 *      - Meanwhile, put a nice tree in stdout to show the build process
 * - Launch QEMU and pave the image onto it
 * - (in the future) listen to the monitor over serial and format the packets nicely
 *
 * Subcommands:
 *    - `update_submodules` - goes through each submodule, looks at git status, pulls it if clean, presents list at
 *    end (color coded!!!) for status of each one - ([DIRTY], [UP TO DATE], [UPDATED], [REMOTE MISSING!!])
 *    - `rust` - used to manage a custom Pebble rust toolchain
 */

mod build;

use build::{BuildStep, RunCargo, Target};
use std::{path::PathBuf, string::ToString};

/// A Project is something that you can instruct Butler to build or run. This might be a Pebble distribution, or
/// something else (e.g. a target-side test that doesn't use the Pebble kernel).
pub struct Project {
    name: String,
    build_steps: Vec<build::BuildFuture>,
}

impl Project {
    pub fn new(name: String) -> Project {
        Project { name, build_steps: Vec::new() }
    }

    pub fn add_build_step<T>(&mut self, step: T)
    where
        T: BuildStep + 'static,
    {
        self.build_steps.push(Box::pin(step.build()));
    }

    pub async fn build(&mut self) {
        for step in self.build_steps.drain(..) {
            match step.await {
                Ok(_) => (),
                Err(err) => panic!("Build of project {} failed: {:?}", self.name, err),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let matches = clap::App::new("Butler")
        .version("0.1.0")
        .author("Isaac Woods")
        .about("Host-side program for managing Pebble builds")
        .subcommand(clap::SubCommand::with_name("build").about("Builds a Pebble distribution"))
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("build") {
        println!("Build requested");
    }

    let mut pebble = Project::new("Pebble".to_string());
    pebble.add_build_step(RunCargo {
        manifest_path: PathBuf::from("kernel/efiloader/Cargo.toml"),
        target: Target::Triple("x86_64-unknown-uefi".to_string()),
        release: false,
        std_components: vec!["core".to_string()],
    });
    pebble.add_build_step(RunCargo {
        manifest_path: PathBuf::from("kernel/kernel_x86_64/Cargo.toml"),
        target: Target::Custom {
            triple: "x86_64-kernel".to_string(),
            spec: PathBuf::from("kernel/kernel_x86_64/x86_64-kernel.json"),
        },
        release: false,
        std_components: vec!["core".to_string(), "alloc".to_string()],
    });
    pebble.build().await;

    println!("Success");
}