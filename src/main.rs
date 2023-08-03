use regex::Regex;
use std::fs;

fn main() -> std::io::Result<()> {
    let month = String::from("2023-07");
    let regex = Regex::new(r"(?m)^(\d+)\] ([\-\+]?\d+[,\.]?\d*) (.*)$").unwrap();
    let src_path = format!("./data/{}.md", month);
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
        let money = match mat.get(2) {
            Some(m) => m.as_str(),
            None => {
                continue;
            }
        };
        let description = match mat.get(3) {
            Some(desc) => desc.as_str().trim(),
            None => {
                continue;
            }
        };
        let csv_line = format!("{};{};{}\n", date, money, description);
        csv.push_str(&csv_line);
    }
    let destinaton_path = format!("./parsed/{}.csv", month);
    fs::create_dir("./parsed/")?;
    fs::write(destinaton_path, csv)?;
    Ok(())
}
