use std::{env, path::Path, process::Command};

fn main()
{
    let out_dir = env::var("OUT_DIR").unwrap();
    let tailwindcss_path = "tailwindcss"; // Path to the Tailwind CSS binary
    let input_css = "public/input.css";
    let output_css = "public/style.css";

    // Ensure the output directory exists
    let _ = std::fs::create_dir_all(Path::new(&out_dir).join("public"));

    // Run the Tailwind CSS CLI
    let status = Command::new(tailwindcss_path)
        .arg("-i")
        .arg(input_css)
        .arg("-o")
        .arg(output_css)
        .status()
        .expect("failed to execute process");

    if !status.success() {
        panic!("Tailwind CSS build failed");
    }
}
