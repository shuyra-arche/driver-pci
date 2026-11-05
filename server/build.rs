fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("none") {
        return;
    }
    let ld = std::env::var("DEP_EXECUTION_LINKER_SCRIPT")
        .expect("execution did not publish DEP_EXECUTION_LINKER_SCRIPT");
    println!("cargo::rustc-link-arg=-T{ld}");
    println!("cargo::rerun-if-changed={ld}");
}
