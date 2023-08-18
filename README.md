# sks (Submodules Kill Solidity)
Lets face it.. Foundry submodules are terrible.
This is a simple cli that acts as a basic package manager to avoid this
issue

## Installation
Clone the repo then run our install script
```bash
git clone https://github.com/Will-Smith11/sks.git
cd sks
sudo chmod +x sks_install
./sks_install
```

# Usage
Simply create a `sks.toml` file in the root directory
the file layout is as such
```toml
name = "EpicTest"

[deps]
uniswap_v2_core = "https://github.com/Uniswap/v2-core.git"
```
to install simply run `sks install`
to clean out the directory run `sks clean`
the name of the deps are what the remapping names will be in foundry when you import them



