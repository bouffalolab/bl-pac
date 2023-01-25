use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use clap::{builder::PossibleValue, Parser, ValueEnum};

#[derive(Clone, Copy, Debug, Parser)]
enum Chip {
    BL616,
    BL808,
}

impl ValueEnum for Chip {
    fn value_variants<'a>() -> &'a [Self] {
        &[Chip::BL616, Chip::BL808]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        let (name, help) = match self {
            Chip::BL616 => ("bl616", "bouffalo bl616"),
            Chip::BL808 => ("bl808", "bouffalo bl808"),
        };
        Some(PossibleValue::new(name).help(help))
    }
}

impl ToString for Chip {
    fn to_string(&self) -> String {
        match self {
            Chip::BL616 => "bl616",
            Chip::BL808 => "bl808",
        }
        .to_string()
    }
}

#[derive(Debug, Parser)]
struct Args {
    /// Chips to process, leave blank to process all chips
    #[clap(value_enum)]
    chips: Vec<Chip>,
}

fn main() {
    let args = Args::parse();

    // get workspace for xtask, its parent being root workspace
    let xtask = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace = xtask
        .parent()
        .expect("find parent of xtask directory")
        .canonicalize()
        .expect("file system canonicalize for workspace path containing xtask");

    // process given chips only, or all chips if none provided in parameter
    let chips = if !args.chips.is_empty() {
        &args.chips
    } else {
        <Chip as ValueEnum>::value_variants()
    };

    for chip in chips {
        process_chip(chip, &workspace);
    }
}

fn process_chip(chip: &Chip, workspace: &Path) {
    let chip_path = workspace.join(chip.to_string());
    let svd_file = chip_path.join(format!("{}.svd", chip.to_string()));
    let svd_config = svd2rust::Config {
        target: svd2rust::Target::RISCV,
        output_dir: chip_path.clone(),
        ..Default::default()
    };

    let input = fs::read_to_string(svd_file).expect("read svd file");

    let generation =
        svd2rust::generate(&input, &svd_config).expect("generate svd crate for one chip");

    let base_dir = chip_path.join("src");
    form::create_directory_structure(&base_dir, &generation.lib_rs).expect("form directory");

    Command::new("cargo")
        .arg("fmt")
        .current_dir(&chip_path)
        .output()
        .expect("run cargo format");
}
