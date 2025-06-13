use clap::{Parser, Subcommand};
use std::fs;
use std::process::Command;

#[derive(Parser)]
#[command(name = "yulpm", about = "Yulvon Package Manager")]
pub struct YulpmCli {
    #[command(subcommand)]
    pub command: YulpmCommand,
}

#[derive(Subcommand)]
pub enum YulpmCommand {
    /// Initialize a new Yulvon package
    Init {
        #[arg(value_name = "NAME")] name: String,
    },
    /// Build the current package
    Build,
    /// Install a dependency
    Add {
        #[arg(value_name = "PACKAGE")] package: String,
    },
}

fn main() {
    let cli = YulpmCli::parse();
    match &cli.command {
        YulpmCommand::Init { name } => {
            let manifest = format!(
                "[package]\nname = \"{}\"\nversion = \"0.1.0\"\n\n[dependencies]\n",
                name
            );
            fs::write("Yulvon.toml", manifest).unwrap();
            fs::create_dir_all("src").unwrap();
            fs::write("src/main.yul", "fn main() {}\n").unwrap();
            println!("Initialized Yulvon package '{}'.", name);
        }
        YulpmCommand::Build => {
            // Call yulc to build main.yul
            let status = Command::new("cargo")
                .args(["run", "--bin", "yulc", "--", "build", "src/main.yul"])
                .status()
                .unwrap();
            if status.success() {
                println!("Build succeeded.");
            } else {
                println!("Build failed.");
            }
        }
        YulpmCommand::Add { package } => {
            // For now, just append to manifest (real registry integration can be added)
            let mut manifest = fs::read_to_string("Yulvon.toml").unwrap();
            manifest.push_str(&format!("{} = \"*\"\n", package));
            fs::write("Yulvon.toml", manifest).unwrap();
            println!("Added dependency '{}'.", package);
        }
    }
}
