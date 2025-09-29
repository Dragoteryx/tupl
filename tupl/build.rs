use rustc_version::version_meta;

pub fn main() {
	println!("cargo::rustc-check-cfg=cfg(nightly)");
	if version_meta().unwrap().channel == rustc_version::Channel::Nightly {
		println!("cargo:rustc-cfg=nightly");
	}
}
