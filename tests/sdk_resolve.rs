use netcorehost::{nethost, pdcstr, pdcstring::PdCString};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[path = "common.rs"]
mod common;

#[test]
#[cfg(all(feature = "netcore3_0", feature = "sdk-resolver"))]
fn resolve_sdk() {
    let hostfxr = nethost::load_hostfxr().unwrap();
    let dotnet_path = which::which("dotnet").unwrap();
    let sdk = hostfxr
        .resolve_sdk(&PdCString::from_os_str(&dotnet_path).unwrap(), pdcstr!("."), true)
        .unwrap();

    let actual_sdks = get_sdks();
    assert!(actual_sdks.contains(&sdk.into_path()));
}

#[test]
#[cfg(all(feature = "netcore3_0", feature = "sdk-resolver"))]
fn list_sdks() {
    let hostfxr = nethost::load_hostfxr().unwrap();
    let dotnet_path = which::which("dotnet").unwrap();
    let mut sdks = hostfxr.get_available_sdks(&PdCString::from_os_str(&dotnet_path).unwrap());
    let mut actual_sdks = get_sdks();
    sdks.sort();
    actual_sdks.sort();
    assert_eq!(actual_sdks, sdks);
}

#[test]
#[cfg(all(feature = "netcore2_1"))]
fn get_native_search_directories() {
    common::setup();

    let hostfxr = nethost::load_hostfxr().unwrap();
    hostfxr
        .get_native_search_directories(pdcstr!(".\\tests\\Test\\bin\\Debug\\net6.0\\Test.dll"))
        .unwrap();
}

fn get_sdks() -> Vec<PathBuf> {
    let sdks_output = Command::new("dotnet").arg("--list-sdks").output().unwrap();
    assert!(sdks_output.status.success());

    String::from_utf8_lossy(&sdks_output.stdout)
        .lines()
        .map(|line| {
            let (version, path) = line.split_once(" ").unwrap();
            Path::new(&path[1..(path.len() - 1)]).join(version)
        })
        .collect::<Vec<_>>()
}
