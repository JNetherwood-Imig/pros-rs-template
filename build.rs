fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let pros_bytes = reqwest::blocking::get(
        "https://github.com/purduesigbots/pros/releases/download/3.8.0/kernel@3.8.0.zip",
    )
    .unwrap()
    .bytes()
    .unwrap();
    let pros_bytes: Vec<_> = pros_bytes.into_iter().collect();

    std::fs::write(format!("{out_dir}/pros.zip"), pros_bytes).unwrap();

    zip::ZipArchive::new(std::fs::File::open(format!("{out_dir}/pros.zip")).unwrap())
        .unwrap()
        .extract(format!("{out_dir}"))
        .unwrap();

    println!("cargo:rustc-link-search=native={out_dir}/firmware");

    println!("cargo:rustc-link-lib=static=pros");
    println!("cargo:rustc-link-lib=static=c");
    println!("cargo:rustc-link-lib=static=m");
}