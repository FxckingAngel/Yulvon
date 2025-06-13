use std::env;
use std::fs;
use std::path::Path;
use bindgen;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: yul-bindgen <header.h> <output.yul>");
        std::process::exit(1);
    }
    let header = &args[1];
    let output = &args[2];
    let bindings = bindgen::Builder::default()
        .header(header)
        .generate()
        .expect("Unable to generate bindings");
    let out_str = bindings.to_string();
    let yul_externs = c_bindings_to_yul_externs(&out_str);
    fs::write(output, yul_externs).expect("Failed to write output");
}

fn c_bindings_to_yul_externs(bindings: &str) -> String {
    let mut yul = String::new();
    for line in bindings.lines() {
        if line.starts_with("pub fn ") {
            let sig = line.trim_start_matches("pub fn ").replace(" -> ", " -> ");
            let sig = sig.replace("(", "(").replace(")", ")");
            let sig = sig.replace(",", ",");
            let sig = sig.replace(":", ":");
            let sig = sig.replace(";", "");
            yul.push_str(&format!("extern fn {}\n", sig));
        }
    }
    yul
}
