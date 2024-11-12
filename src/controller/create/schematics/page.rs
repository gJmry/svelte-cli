use std::fs::{create_dir_all, File};
use std::io::{Result, Write};
use std::path::Path;
use crate::controller::utils::path_utils;

pub fn main(name: String){
    if let Err(e) = make_page_file(name) {
        eprintln!("Error: {}", e);
    }
}

fn make_page_file(name: String) -> Result<()>{
    let root_path = path_utils::get_root_path(path_utils::get_current_directory());

    let filename = format!("{}/src/routes/{}/+page.svelte", root_path.display(), name);
    let path = Path::new(&filename);

    if Path::exists(path) {
        println!("Page file already exists");
        std::process::exit(0);
    }

    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    let content = page_file_configuration(name);
    file.write_all(content.as_bytes())?;

    let display_path = filename.replace("\\", "/");
    println!(
        "Successfully generated: {}",
        display_path
    );

    Ok(())
}

fn page_file_configuration(name: String) -> String {
    format!(
        r#"
<script></script>
<style></style>
<main>
    {} page
</main>
"#,
        name
    )
}