use foundry_config::{
    ethers_solc::remappings::{RelativeRemapping, Remapping},
    Config,
};

pub fn clear_deps() -> anyhow::Result<()> {
    let mut config = foundry_config::load_config();
    config.remappings.clear();
    let stringed = config.to_string_pretty()?;
    std::fs::write(Config::FILE_NAME, stringed)?;

    Ok(())
}

pub fn inject_deps(mappings: Vec<Remapping>) -> anyhow::Result<()> {
    let mut config = foundry_config::load_config();

    mappings
        .into_iter()
        .for_each(|r| config.remappings.push(RelativeRemapping::new(r, ".")));

    let stringed = config.to_string_pretty()?;
    std::fs::write(Config::FILE_NAME, stringed)?;

    Ok(())
}
