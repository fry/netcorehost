#![allow(unused)]

use netcorehost::pdcstring::PdCString;
use std::{env, path::Path, process::Command, str::FromStr};

pub fn test_netcore_version() -> String {
   env::var("NETCOREHOST_TEST_NETCORE_VERSION").unwrap_or_else(|_| "net5.0".to_string())
}

pub fn test_runtime_config_path() -> PdCString {
    PdCString::from_str(&format!(
        "tests/Test/bin/Debug/{}/Test.runtimeconfig.json",
        test_netcore_version()
    ))
    .unwrap()
}

pub fn test_dll_path() -> PdCString {
    PdCString::from_str(&format!(
        "tests/Test/bin/Debug/{}/Test.dll",
        test_netcore_version()
    ))
    .unwrap()
}

pub fn setup() {
    build_test_project()
}

pub fn build_test_project() {
    if Path::new(&test_dll_path().to_os_string()).exists() {
        return;
    }

    Command::new("dotnet")
        .arg("build")
        .current_dir("tests/Test")
        .spawn()
        .expect("dotnet build failed")
        .wait()
        .expect("dotnet build failed");
}
