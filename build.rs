use std::process::Command;

fn main() {
    println!("cargo:warning=Building NIVA OS kernel");
    println!("cargo:warning=Target: x86_64-unknown-none");
    println!("cargo:warning=Edition: 2021");
    
    // Build with bootloader
    println!("cargo:warning=Building with x86_64 bootloader");
}
