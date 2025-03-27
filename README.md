# Elden Ring Convenient Deaths

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
  - [Contributing](#contributing)

---

## For Users

### Features

- Keep runes on death.
- Keep rune arc enabled on death.
- Make death fadeout quicker.

### Installation

1. Download the latest release from the [Releases](https://github.com/chozandrias76/eldenring-convenient-deaths/releases) page.
2. Extract the downloaded archive to your Elden Ring installation directory.
3. Ensure the `er_convenient_deaths` file is placed in the correct folder.

### Usage

1. Launch Elden Ring.
2. The tool will automatically activate upon death.
3. To configure options, edit the `config.json` file located in the same directory as the `.dll`.

---

## For Developers

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
   git clone https://github.com/your-repo/eldenring-convenient-deaths.git ./eldenring-convenient-deaths
   cd eldenring-convenient-deaths
   ```

4. Build the project
  ```sh
  cargo build
  ```

5. Check ./target/debug folder for `er_convenient_deaths.dll`
