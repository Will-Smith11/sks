pub fn fetch_url(url: &str, dir: &str) -> anyhow::Result<()> {
    let res = std::process::Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(dir)
        .spawn()?
        .wait()?;
    println!("fetched result: {res}");

    Ok(())
}
