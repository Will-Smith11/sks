use clap::{Parser, Subcommand};
use sks::{config_file::ConfigFile, foundry_injection::inject_deps};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Opts {
    #[clap(subcommand)]
    pub sub: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    Install,
    Clean,
}
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.sub {
        Subcommands::Clean => {
            let config = ConfigFile::new()?;
            config.clean()?;
            Ok(())
        }
        Subcommands::Install => {
            let config = ConfigFile::new()?;
            let _ = config.clean();
            let deps = config.pull_deps()?;
            inject_deps(deps)?;
            Ok(())
        }
    }
}
