extern crate dirs;
use git2::{self, Repository};
use std::fs;

const REPO_URL: &'static str = "http://github.com/johnbakhmat/aoc-templates.git";

pub async fn cache_template() -> anyhow::Result<()> {
    let home_dir = dirs::home_dir().unwrap();
    let template_dir = home_dir.join(".aoc/templates/");

    if !template_dir.exists() {
        fs::create_dir_all(&template_dir)?;
        Repository::clone(REPO_URL, &template_dir)?;
    }

    let repo = Repository::open(&template_dir)?;
    crate::pull::run(&repo, "origin", "main")?;

    Ok(())
}
pub async fn get_template(lang: &str) -> anyhow::Result<std::path::PathBuf> {
    let template_dir = dirs::home_dir()
        .unwrap()
        .join(format!(".aoc/templates/{}/", lang));
    assert!(template_dir.exists(), "Oops, template not found");
    Ok(template_dir)
}
