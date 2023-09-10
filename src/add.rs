use std::fs;
use std::path::Path;

pub async fn run(path: &str, lang: &str) -> anyhow::Result<()> {
    let path = Path::new(path);
    let template_dir = Path::new("/home/john/projects/aoc-cli/src/templates").join(lang);

    assert!(
        template_dir.exists(),
        "Sorry! We have no template for that language"
    );
    println!("{:?}", template_dir);

    Ok(())
}
