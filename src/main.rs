extern crate rpg_engine;

use rpg_engine::*;
use std::fs;
use std::io::Write;
use walkdir::WalkDir;

fn main() -> RPGResult<()> {
    let mut item_tome = rpg_engine::Tome::<ItemSpec, ItemInstance>::new("tome/items");

    for entry in WalkDir::new("tome/items") {
        let entry = entry.unwrap();
        if (entry.file_type().is_file()
            && !entry
                .file_name()
                .to_str()
                .map(|s| s.starts_with("."))
                .unwrap_or(false))
        {
            let full_path_name: &str =
                &entry.path().to_str().expect("couldn't read path as string");
            let path_name = &full_path_name[11..full_path_name.len() - 5];

            print!("check {}", &full_path_name);
            std::io::stdout().flush();

            match item_tome.get_instance(path_name) {
                Ok(_) => println!(" (ok)"),
                Err(e) => {
                    println!("\n{}", &e);
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}
