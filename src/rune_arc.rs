use std::{env, path::{Path, PathBuf}, ptr};

use broadsword::runtime;
use broadsword::scanner::threaded::scan;
use broadsword::scanner::Pattern;
use std::slice;
use practice_tool_tasks::codegen::{self, aob_direct, aob_indirect_twice};


fn patches_paths() -> impl Iterator<Item = PathBuf> {
  let base_path = PathBuf::from(
      env::var("ER_PATCHES_PATH").unwrap_or_else(|_| "C:/SteamLibrary/steamapps/common/ELDEN RING/Game".to_string()),
  );
  base_path
      .read_dir()
      .expect("Couldn't scan patches directory")
      .map(Result::unwrap)
      .map(|dir| dir.path().join("Game").join("eldenring.exe"))
}

fn base_addresses_rs_path() -> PathBuf {

  let path = Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| "D:\\Elden Ring Tools\\er-auto-arc\\lib".to_string())
      )
      .ancestors()
      .nth(1)
      .unwrap()
      .to_path_buf()
      .join("lib")
      .join("libeldenring")
      .join("src")
      .join("codegen")
      .join("base_addresses.rs");

    if !path.exists() {
        panic!("base_addresses.rs not found. Ensure CARGO_MANIFEST_DIR is set with a path ending at \\lib and containing a libeldenring folder");
    }

    return path;
}

const GAME_DATA_MAN_AOB: &str = "48 8B 05 ?? ?? ?? ?? 48 85 C0 74 05 48 8B 40 58 C3 C3";
const CS_REGULATION_MANAGER_AOB: &str = "48 8B 0D ?? ?? ?? ?? 48 85 C9 74 0B 4C 8B C0 48 8B D7";
pub fn hook() {
  let game_data_man = aob_indirect_twice(
    "GameDataMan",
    &[GAME_DATA_MAN_AOB],
    3,
    7,
    true,
  );
  let cs_regulation_manager = aob_indirect_twice(
    "CSRegulationManager",
    &[CS_REGULATION_MANAGER_AOB],
    3,
    7,
    true,
  );
  codegen::codegen_base_addresses(base_addresses_rs_path(), patches_paths(), &[game_data_man, cs_regulation_manager]);
}
