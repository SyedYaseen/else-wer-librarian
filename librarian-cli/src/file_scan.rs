use anyhow::anyhow;
use lofty::{
    self,
    config::{ParseOptions, ParsingMode},
    file::AudioFile,
    probe::Probe,
};
use std::{fs::read, path::Path};
use tokio::task;
use walkdir::WalkDir;

use crate::models::CreateFileMetadata;
pub async fn file_scan(path: String) -> anyhow::Result<()> {
    // let audio_formats = vec!["mp3", "m4b", "aac"];
    // let last_book = String::new();
    // for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
    //     if entry.path().is_file() {
    //         if let Some(ext) = entry.path().extension() {
    //             let ext = ext.to_str().unwrap_or_default();
    //             if audio_formats.contains(&ext) {
    //                 println!(
    //                     "{} | {} | {}",
    //                     ext,
    //                     entry.path().parent().unwrap().display(),
    //                     entry.path().file_name().unwrap_or_default().display()
    //                 );
    //                 continue;
    //             }
    //         }
    //     }
    // }
    println!("this work");
    let mets = extract_metadata(
        "/media/yaseen/Data/AudioBooks/JeffersonMays-TheExpanse/James S. A. Corey - The Expanse (complete series)/narrated by Jefferson Mays/0.2 - The Churn (novella)/Corey, James S. A. - The Expanse 0.2 - The Churn (novella) - 00 Opening Credits.mp3",
    ).await?;
    // println!("{:#?}", mets);
    Ok(())
}

pub async fn extract_metadata(path: &str) -> anyhow::Result<CreateFileMetadata> {
    let path_owned = path.trim().to_owned();

    let file_name = Path::new(&path_owned)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();

    let mut metadata =
        CreateFileMetadata::new(path_owned.clone(), None, file_name, None, None, None, None);

    let probe = Probe::open(&path_owned);

    if let Ok(probe) = probe {
        let probe = probe.options(ParseOptions::new().parsing_mode(ParsingMode::Relaxed));

        if let Ok(probe) = probe.guess_file_type() {
            let tagged_file = probe.read()?;
            println!(tagged_file.primary_tag());

            let properties = tagged_file.properties();
            println!("{:#?}", properties);
            metadata.duration = Some(properties.duration().as_millis() as i64);
            metadata.bitrate = properties.audio_bitrate().map(|b| b as i64);
        }
    }

    Ok(metadata)
}
