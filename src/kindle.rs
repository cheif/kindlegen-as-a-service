use std::process::Command;
use std::fs;
use std::str;
use std::env;
use std::path::Path;
use uuid::Uuid;
use infer;

#[cfg(not(test))]
use log::{debug, info};

#[cfg(test)]
use std::{println as debug, println as info};

pub fn convert(buf: Vec<u8>) -> Result<fs::File, String> {
    let ebook_convert = env::var("EBOOK_CONVERT").unwrap_or("ebook-convert".to_string());
    let kind = infer::get(&buf).ok_or_else(|| "Could not parse filetype")?;
    info!("mime-type: {}", kind.mime_type());
    let extension = match kind.mime_type() {
        "application/pdf" => Ok(kind.extension()),
        // Epub files are just zipfiles, so we can't really distinguish them from other zipfiles,
        // let's just hope for the best.
        "application/zip" => Ok("epub"),
        _ => Err(format!("Unsupported mime-type: {}", kind.mime_type()))
    }?;
    info!("extension: {}", extension);

    let uuid = Uuid::new_v4();
    let input_file = format!("/tmp/{}.{}", uuid, extension);
    let mobi_file = format!("/tmp/{}.mobi", uuid);
    debug!("Writing to disk ({})", &input_file);
    // Write the input file to disk, so that we can run kindlegen on it
    fs::write(&input_file, buf).map_err(|_| "Unable to write file")?;

    debug!("Running `{} {} {}`", ebook_convert, &input_file, &mobi_file);
    let output = Command::new(ebook_convert)
        .arg(input_file)
        .arg(&mobi_file)
        .output()
        .map_err(|err| format!("ebook-convert run failed: {}", err))?;

    debug!("ebook-convert output: {:?}", output);

    // Kindlegen sometimes exits with 1 but still manages to generate a file (when there are warnings)
    if !Path::new(&mobi_file).exists() {
        return Err(str::from_utf8(&output.stdout).unwrap().to_string())
    }

    debug!("Returning to user");
    return fs::File::open(mobi_file).map_err(|_| "Could not open resulting file".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_epub() {
        let buf = std::fs::read("testdata/hostregn.epub").unwrap();
        convert(buf).unwrap();
    }

    #[test]
    fn incorrect_language_code_epub() {
        let buf = std::fs::read("testdata/pottraning_pa_3_daga.epub").unwrap();
        convert(buf).unwrap();
    }

    #[test]
    fn warnings_epub() {
        let buf = std::fs::read("testdata/de_aderton.epub").unwrap();
        convert(buf).unwrap();
    }

    #[test]
    fn correct_pdf() {
        let buf = std::fs::read("testdata/Howtoskatea10k.pdf").unwrap();
        convert(buf).unwrap();
    }

}

