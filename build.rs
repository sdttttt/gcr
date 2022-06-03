use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize)]
struct CargoToml {
	package: Package,
}

#[derive(Deserialize)]
struct Package {
	version: String,
}

fn main() {
	println!("Thank you for installing and using GRC! [by @sdttttt]");
	println!("cargo:rerun-if-changed=Cargo.toml");

	let cargo_toml_str = std::fs::read_to_string(Path::new("Cargo.toml")).unwrap();
	let cargo_toml = toml::from_str::<CargoToml>(&cargo_toml_str).unwrap();

	let version_file_rs = Path::new("src/version.rs");

	let version_file_str = format!("pub const VERSION: &str = \"{}\";", cargo_toml.package.version);
	std::fs::write(version_file_rs, version_file_str).unwrap();
}
