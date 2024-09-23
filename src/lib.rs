use drm::control::connector::Interface;
use drm::control::Device;

const CARD_PATH: &str = "/dev/dri/card0";

mod card;

pub enum IsFree {
    Free,
    NotFree,
    Unknown
}

use IsFree::{Free, NotFree, Unknown};

pub fn get_os() -> &'static str {
    std::env::consts::OS
}

pub fn is_os_free() -> IsFree {
    match get_os() {
        "linux" => Free,
        "macos" => NotFree,
        "ios" => NotFree,
        "freebsd" => Free,
        "dragonfly" => Free,
        "netbsd" => Free,
        "openbsd" => Free,
        "solaris" => NotFree,
        "android" => Free,
        "windows" => NotFree,
        _ => Unknown
    }
}

pub fn get_architecture() -> &'static str {
    std::env::consts::ARCH
}

pub fn is_architecture_free(architecture: &str) -> IsFree {
    match architecture {
        "x86" => NotFree,
        "x86_64" => NotFree,
        "arm" => NotFree,
        "aarch64" => Free,
        "loongarch64" => Free,
        "m68k" => Free,
        "csky" => Free,
        "mips" => NotFree,
        "mips64" => NotFree,
        "powerpc" => NotFree,
        "powerpc64" => NotFree,
        "riscv64" => Free,
        "s390x" => NotFree,
        "sparc64" => Free,
        _ => Unknown,
    }
}

pub fn get_connectors() -> Result<Vec<Interface>, String> {
    let card = card::Card::open(CARD_PATH).map_err(|_| String::from("Couldn't open ") + CARD_PATH)?;
    card.resource_handles().map_err(|_| "Couldn't get connectors")?.connectors;

    unimplemented!()
}

pub fn is_connector_free(connector: &Interface) -> IsFree {
    unimplemented!()
}
