use std::process::Command;
use std::fs;
use std::str;
use std::path::Path;
use uuid::Uuid;

pub fn convert(buf: Vec<u8>) -> Result<fs::File, String> {
    let uuid = Uuid::new_v4();
    let epub_file = format!("/tmp/{}.epub", uuid);
    let mobi_file = format!("/tmp/{}.mobi", uuid);
    println!("... Writing to disk ({})", &epub_file);
    // Write the epub to disk, so that we can run kindlegen on it
    fs::write(&epub_file, buf).map_err(|_| "Unable to write file")?;

    println!("... Running kindlegen ({})", &epub_file);
    let output = Command::new("kindlegen")
        .arg(epub_file)
        .output()
        .map_err(|err| format!("Kindlegen run failed: {}", err))?;

    // Kindlegen sometimes exits with 1 but still manages to generate a file (when there are warnings)
    if !Path::new(&mobi_file).exists() {
        return Err(str::from_utf8(&output.stdout).unwrap().to_string())
    }

    println!("Returning to user");
    return fs::File::open(mobi_file).map_err(|_| "Could not open resulting file".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_epub() {
        let buf = std::fs::read("testdata/hostregn.epub").unwrap();
        let res = convert(buf);
        assert!(res.is_ok());
    }

    #[test]
    fn incorrect_language_code_epub() {
        let buf = std::fs::read("testdata/pottraning_pa_3_daga.epub").unwrap();
        let res = convert(buf);
        assert!(res.is_err());
    }

    #[test]
    fn warnings_epub() {
        let buf = std::fs::read("testdata/de_aderton.epub").unwrap();
        let res = convert(buf);
        assert!(res.is_ok());
    }
}

