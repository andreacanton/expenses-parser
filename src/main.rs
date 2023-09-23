use regex::Regex;
use std::{env, fs, io::Result, path::PathBuf};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let folder_path = args.get(1).expect("Path argument is required");

    let watch_folder = PathBuf::from(folder_path);
    process_folder(&watch_folder)
}

fn process_folder(folder_path: &PathBuf) -> Result<()> {
    let destination_folder = format!("{}/../parsed/", folder_path.display());
    fs::create_dir_all(&destination_folder)?;
    let paths = fs::read_dir(folder_path).unwrap();
    for result_dir_entry in paths {
        let dir_entry = result_dir_entry.unwrap();
        let file_name = dir_entry.file_name().into_string().unwrap();
        let mut split = file_name.split(".");
        let month = split.next().unwrap();
        let csv = process_month(month, &dir_entry.path())?;
        let dest_path = PathBuf::from(format!("{}{}.csv", &destination_folder, month));
        fs::write(dest_path.as_path(), csv)?;
        println!("parsed {} into csv", dir_entry.path().display());
    }
    Ok(())
}

fn process_month(month: &str, src_path: &PathBuf) -> Result<String> {
    let regex = Regex::new(r"(?m)^(\d+)\] ([\-\+]?)(\d+[,\.]?\d*) (.*)$").unwrap();
    let content = fs::read_to_string(src_path)?;
    let mut csv = String::new();
    let result = regex.captures_iter(&content);
    for mat in result {
        let date = match mat.get(1) {
            Some(d) => format!("{}-{:0>2}", month, d.as_str()),
            None => {
                continue;
            }
        };
        let is_in = match mat.get(2) {
            Some(d) => {
                if d.as_str() == "+" {
                    true
                } else {
                    false
                }
            }
            None => true,
        };
        let money = match mat.get(3) {
            Some(m) => m.as_str(),
            None => {
                continue;
            }
        };
        let description = match mat.get(4) {
            Some(desc) => desc.as_str().trim(),
            None => {
                continue;
            }
        };
        let money_in = if is_in { money } else { "" };
        let money_out = if is_in { "" } else { money };
        let csv_line = format!("{};{};{};{}\n", date, money_in, money_out, description);
        csv.push_str(&csv_line);
    }
    Ok(csv)
}
