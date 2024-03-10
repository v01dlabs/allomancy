use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use clap::{Args, Parser};
use strum::IntoEnumIterator;
use xtask::{Platform, Package, Version};
use xshell::{cmd, Shell};

// ----------------------------------------------------------------------------
// Command-line Interface

#[derive(Debug, Parser)]
enum Cli {
    /// Build documentation for the specified chip.
    //BuildDocumentation(BuildDocumentationArgs),
    /// Build all examples for the specified chip.
    //BuildExamples(BuildExamplesArgs),
    /// Build the specified package with the given options.
    //BuildPackage(BuildPackageArgs),
    /// Run the given example for the specified chip.
    //RunExample(RunExampleArgs),
    /// Bump the version of the specified package(s)
    //BumpVersion(BumpVersionArgs),
    /// Run hardware tests for the specified package and chip.
    HardwareUpload(HardwareUploadArgs),
}

#[derive(Debug, Args)]
struct BuildDocumentationArgs {
    /// Package to build documentation for.
    #[arg(value_enum)]
    package: Package,
    /// Which chip to build the documentation for.
    #[arg(value_enum)]
    platform: Platform,
    /// Open the documentation in the default browser once built.
    #[arg(long)]
    open: bool,
    /// Directory in which to place the built documentation.
    #[arg(long)]
    output_path: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct BuildExamplesArgs {
    /// Package to build examples for.
    #[arg(value_enum)]
    package: Package,
    /// Which board to build the examples for.
    #[arg(value_enum)]
    platform: Platform,
}

#[derive(Debug, Args)]
struct HardwareUploadArgs {
    /// Which board to test on.
    #[arg(value_enum)]
    platform: Platform,
    /// Host to ssh into.
    host: String,
}

#[derive(Debug, Args)]
struct BuildPackageArgs {
    /// Package to build.
    #[arg(value_enum)]
    package: Package,
    /// Target to build for.
    #[arg(long)]
    target: Option<String>,
    /// Features to build with.
    #[arg(long, value_delimiter = ',')]
    features: Vec<String>,
    /// Toolchain to build with.
    #[arg(long)]
    toolchain: Option<String>,
}

#[derive(Debug, Args)]
struct RunExampleArgs {
    /// Package to run example from.
    #[arg(value_enum)]
    package: Package,
    /// Which chip to run the examples for.
    #[arg(value_enum)]
    platform: Platform,
    /// Which example to run
    example: String,
}

#[derive(Debug, Args)]
struct BumpVersionArgs {
    /// How much to bump the version by.
    #[arg(value_enum)]
    amount: Version,
    /// Package(s) to target.
    #[arg(value_enum, default_values_t = Package::iter())]
    packages: Vec<Package>,
}

// ----------------------------------------------------------------------------
// Application

fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter_module("xtask", log::LevelFilter::Info)
        .init();

    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace = workspace.parent().unwrap().canonicalize()?;

    match Cli::parse() {
        //Cli::BuildDocumentation(args) => build_documentation(&workspace, args),
        //Cli::BuildExamples(args) => build_examples(&workspace, args),
        //Cli::BuildPackage(args) => build_package(&workspace, args),
        //Cli::RunExample(args) => run_example(&workspace, args),
        //Cli::BumpVersion(args) => bump_version(&workspace, args),
        Cli::HardwareUpload(args) => hardware_upload(&workspace, args),
    }
}

// ----------------------------------------------------------------------------
// Subcommands


// https://stackoverflow.com/a/65192210
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;

        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}


fn hardware_upload(workspace: &Path, args: HardwareUploadArgs) -> Result<()> {
    let package_path = xtask::windows_safe_path(&workspace.join(args.package.to_string()));
    let target = args.platform.target();
    xtask::build_package(&package_path, vec![], None, target)?;
    cmd!("scp -B {package_path}/target/{target}/release/{args.package} {args.host}:").run()?;
    Ok(())
}