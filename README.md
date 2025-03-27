# Elden Ring Convenient Deaths
<p align="center" style="margin-bottom: 5rem">
  <a href="./readme.md">
    <img alt="Elden Ring Splash Screen Skipper Logo" src="./logo.svg" alt="ER Skip Startup Cutscenes Logo" width="250">
    
  </a>
</p>
A tool to enhance the gameplay experience of Elden Ring by providing convenient death-related features.
Intended for offline user only.

---

## Table of Contents

- [For Users](#for-users)
  - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
- [For Developers](#for-developers)
  - [Building from Source](#building-from-source)
- [Credits](#credits)

---

## For Users

### Features

- Keep runes on death.
- Keep rune arc enabled on death.
- Make death fadeout quicker.

### Installation

1. Download the latest release from the [Releases](https://github.com/chozandrias76/er-convenient-deaths/releases) page.
2. Extract the downloaded archive to your dll mods folder within the Elden Ring installation directory.
3. Use [LazyLoader](https://www.nexusmods.com/darksouls3/mods/677) or some other Fromsoft compatable DLL injector to inject the DLL when you start up the game.

### Usage

1. Launch Elden Ring.
2. The tool will automatically activate upon death.
3. To configure options, edit the `er_convenient_deaths.toml` file located in the same directory as the `eldenring.exe`.
4. ⚠️❗The configuration file created with the DLL disables every feature by default.❗⚠️ 
You must update the corresponding values to `true` to enable. For example:
```toml
keep_runes_on_death = true
keep_rune_arc_on_death = true
quicker_deaths = true
```

---

## For Developers

This program heavily relies on offsets identified in reverse engineering Elden Ring v1.16 and their instructions.

### Building from Source

1. Install Rust:
   - Follow the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).
2. Install the Rust nightly toolchain:
```sh
rustup install nightly
rustup default nightly
```
3. Clone the project
```sh
git clone https://github.com/chozandrias76/er-convenient-deaths.git ./eldenring-convenient-deaths
cd eldenring-convenient-deaths
```
4. Build the project
```sh
cargo build
```
5. Check `./target/debug` folder for `er_convenient_deaths.dll`

## Credits

<p align="left" style="display: flex; justify-content: space-between">
    <a href="https://github.com/vswarte" ><img src="https://avatars.githubusercontent.com/u/6827387?v=4" alt="Description" width="100"></br>Chainfailure</br></a>
    <a href="https://github.com/FeeeeK" ><img src="https://avatars.githubusercontent.com/u/26704473?v=4" alt="Description" width="100"></br>Aъ</br></a>
    <a href="https://github.com/ImAxel0" ><img src="https://avatars.githubusercontent.com/u/124681710?v=4" alt="Description" width="100"></br>Axel0</br></a>
    <a href="https://github.com/techiew" ><img src="https://avatars.githubusercontent.com/u/22145177?v=4" alt="Description" width="100"></br>Techiew</br></a>
</p>
