use anyhow::anyhow;
use ethers_solc::remappings::{RelativeRemapping, Remapping};

pub fn inject_deps(mappings: Vec<Remapping>) -> anyhow::Result<()> {
    let data = std::fs::read_to_string("foundry.toml")
        .map_err(|_| anyhow!("failed to find foundry.toml"))?;

    let mut foundry_config: toml::Table =
        toml::from_str(&data).map_err(|_| anyhow::anyhow!("failed to read foundry.toml"))?;

    let err = || anyhow!("failed to parse foundry.toml");

    let profile_default = foundry_config
        .get_mut("profile")
        .ok_or_else(err)?
        .get_mut("default")
        .ok_or_else(err)?;

    if let Some(remappings) = profile_default.get_mut("remappings") {
        let remappings = remappings.as_array_mut().ok_or_else(err)?;

        mappings
            .into_iter()
            .map(|r| RelativeRemapping::new(r, "."))
            .for_each(|r| {
                let valued = toml::Value::String(r.to_string());
                if !remappings.contains(&valued) {
                    remappings.push(valued);
                }
            });
    } else {
        return Err(anyhow!("please add remappings = [] under profile.default"));
    }

    std::fs::write("foundry.toml", toml::to_string_pretty(&foundry_config)?)?;

    Ok(())
}
