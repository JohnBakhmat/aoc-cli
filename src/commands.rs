use chrono::Datelike;

use crate::YearInput;
use std::fs;
use std::path::Path;

pub async fn init_command(path: &str, year: YearInput) -> anyhow::Result<()> {
    let path = Path::new(path);
    if !path.exists() {
        fs::create_dir(path)?;
    }
    assert!(path.is_dir(), "Path must be a directory");
    let is_empty = path.read_dir()?.next().is_none();

    let current_year = chrono::Utc::now().year() as u16;
    match year {
        YearInput::All => {
            assert!(is_empty, "Directory must be empty");
        }
        YearInput::Current => {
            let year_path = path.join(current_year.to_string());
            assert!(
                !year_path.exists(),
                "Directory for current year already exists"
            );
        }
        YearInput::Specific(y) => {
            let year_path = path.join(y.to_string());

            assert!(
                y >= 2015 && y <= current_year,
                "Year must be between 2015 and the current year"
            );

            assert!(
                !year_path.exists(),
                "Directory for year {} already exists",
                y
            );
        }
    }

    let year_range = match year {
        YearInput::All => 2015..=current_year,
        YearInput::Current => current_year..=current_year,
        YearInput::Specific(y) => y..=y,
    };

    println!("Creating directory structure...");

    for year in year_range {
        let year_path = path.join(year.to_string());
        fs::create_dir(&year_path)?;
        for day in 1..=25 {
            let day_path = year_path.join(format!("day{:02}", day));
            fs::create_dir(&day_path)?;
            let input_path = day_path.join("input.1.txt");
            fs::File::create(&input_path)?;
            let input_path = day_path.join("input.2.txt");
            fs::File::create(&input_path)?;
        }
    }

    println!("Successfully created directory structure!");
    println!("Next steps:");
    println!("1. cd {}", path.display());
    println!("2. aoc-cli add deno");
    println!("Or do whatever you want with the directory structure! Im not a cop!");

    Ok(())
}
