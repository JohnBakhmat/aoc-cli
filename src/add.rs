use git2::Repository;
use glob::glob;
use std::fs;
use std::path::Path;

pub async fn run(path: &str, lang: &str) -> anyhow::Result<()> {
    let folders = glob(&format!("{}/**/day*", path))?
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    let already_exists = glob(&format!("{}/**/day*/{}", path, lang))?
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    let folders_to_add = folders
        .iter()
        .filter(|x| !already_exists.contains(x))
        .collect::<Vec<_>>();

    for folder in folders_to_add {
        println!("Adding {} to {}", lang, folder.display());
    }

    Ok(())
}
