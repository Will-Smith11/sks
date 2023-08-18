use foundry_config::{
    ethers_solc::remappings::{RelativeRemapping, Remapping},
    Config,
};

pub fn inject_deps(mappings: Vec<Remapping>) -> anyhow::Result<()> {
    let mut config = foundry_config::load_config();

    mappings
        .into_iter()
        .map(|r| RelativeRemapping::new(r, "."))
        .for_each(|r| {
            if !config.remappings.contains(&r) {
                config.remappings.push(r);
            }
        });

    let stringed = config.to_string_pretty()?;
    std::fs::write(Config::FILE_NAME, stringed)?;

    Ok(())
}
