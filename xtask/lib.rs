use std::{
    collections::VecDeque,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use clap::ValueEnum;
use strum::{Display, EnumIter, IntoEnumIterator};

use self::cargo::CargoArgsBuilder;



mod cargo;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter, ValueEnum)]
#[strum(serialize_all = "kebab-case")]
pub enum Platform {
    Pi5,
    Pi4,
    Pi3,
    PiZero,
}

impl Platform {
    pub fn target(&self) -> &str {
        use Platform::*;

        match self {
            Pi5 => "aarch64-unknown-linux-gnu", // use 64-bit on Pi5
            Pi4 | Pi3 => "armv7-unknown-linux-gnueabihf",
            PiZero => "arm-unknown-linux-gnueabihf",
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter, ValueEnum)]
#[strum(serialize_all = "kebab-case")]
pub enum Package {
    EspHal,
    EspHalProcmacros,
    EspHalSmartled,
    EspLpHal,
    EspRiscvRt,
    Examples,
}


#[derive(Debug, Clone, Copy, Display, ValueEnum)]
#[strum(serialize_all = "lowercase")]
pub enum Version {
    Major,
    Minor,
    Patch,
}

/// Build the specified package, using the given toolchain/target/features if
/// provided.
pub fn build_package(
    package_path: &Path,
    features: Vec<String>,
    toolchain: Option<String>,
    target: Option<String>,
) -> Result<()> {
    log::info!("Building package '{}'", package_path.display());
    if !features.is_empty() {
        log::info!("  Features: {}", features.join(","));
    }
    if let Some(ref target) = target {
        log::info!("  Target:   {}", target);
    }

    let mut builder = CargoArgsBuilder::default()
        .subcommand("build")
        .arg("-Zbuild-std=core")
        .arg("--release");

    if let Some(toolchain) = toolchain {
        builder = builder.toolchain(toolchain);
    }

    if let Some(target) = target {
        builder = builder.target(target);
    }

    if !features.is_empty() {
        builder = builder.features(&features);
    }

    let args = builder.build();
    log::debug!("{args:#?}");

    cargo::run(&args, package_path)?;

    Ok(())
}

/// Make the path "Windows"-safe
pub fn windows_safe_path(path: &Path) -> PathBuf {
    PathBuf::from(path.to_str().unwrap().to_string().replace("\\\\?\\", ""))
}