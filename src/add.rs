use crate::temp::{cache_template, get_template};

use fs_extra::dir::CopyOptions;
use glob::glob;
use std::path::Path;
use std::{any, fs};

pub async fn run(path: &str, lang: &str) -> anyhow::Result<()> {
    cache_template().await?;
    let template = get_template(lang).await?;

    let path = Path::new(path).canonicalize()?;

    println!("Adding {} to folders in {}", lang, path.to_string_lossy());

    let folders = glob(&format!("{}/**/day*", path.to_string_lossy()))?
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    let already_exists = glob(&format!("{}/**/day*/{}", path.to_string_lossy(), lang))?
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    let folders_to_add = folders
        .iter()
        .filter(|x| !already_exists.contains(x))
        .collect::<Vec<_>>();

    let mut options = CopyOptions::new();
    options.skip_exist = true;

    for folder in &folders_to_add {
        fs_extra::dir::copy(
            template.to_str().unwrap(),
            folder.to_str().unwrap(),
            &options,
        )?;
    }

    println!(
        "Successfully added {} to {} folders",
        lang,
        &folders_to_add.len()
    );

    Ok(())
}
