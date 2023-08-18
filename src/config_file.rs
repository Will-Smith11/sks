use anyhow::anyhow;
use dirs::home_dir;
use foundry_config::ethers_solc::remappings::Remapping;
use serde::{Deserialize, Serialize};
use toml::Table;

use crate::git_commands;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    name: String,
    deps: Table,
}

impl ConfigFile {
    pub fn new() -> anyhow::Result<Self> {
        let path = "./sks.toml";

        toml::from_str(&std::fs::read_to_string(path)?)
            .map_err(|_| anyhow::anyhow!("failed to load toml"))
    }

    pub fn clean(&self) -> anyhow::Result<()> {
        let mut path = home_dir().ok_or(anyhow::anyhow!("couldn't find homedir"))?;
        path.push(".sks");
        path.push(&self.name);
        std::fs::remove_dir_all(path)?;

        Ok(())
    }

    /// The format is: `solc context:prefix=target ./MyContract.sol`
    pub fn pull_deps(&self) -> anyhow::Result<Vec<Remapping>> {
        let mut res = Vec::new();

        let mut path = home_dir().ok_or(anyhow::anyhow!("couldn't find homedir"))?;
        path.push(".sks");
        path.push(&self.name);

        std::fs::create_dir_all(&path)?;

        for (name, git) in self.deps.iter() {
            let git_url = git.as_str().ok_or(anyhow::anyhow!(format!(
                "didn't get a git url for dep: {}",
                name
            )))?;
            path.push(name);
            git_commands::fetch_url(
                git_url,
                path.to_str()
                    .ok_or(anyhow!("failed to convert path to str"))?,
            )?;

            res.push(Remapping {
                name: name.clone(),
                path: path
                    .to_str()
                    .ok_or(anyhow!("failed to convert path to str"))?
                    .to_owned(),
                context: None,
            });

            path.pop();
        }

        Ok(res)
    }
}
