use beatsaver::apis::default_api::maps_id_id_get;
use std::{convert::TryInto, fs::create_dir_all, path::{Path, PathBuf}};
use bytes::Bytes;
use clap::Clap;
use std::io::Cursor;

use dynfmt::{Format,SimpleCurlyFormat, FormatArgs};

use zip::ZipArchive;

mod cli;
use cli::BeatSaverCli;

#[tokio::main]
async fn main() {

    let opts = BeatSaverCli::parse();
    // Create a new client
    let client = BeatSaver::new();

    let map = opts.url.strip_prefix("beatsaver://").expect("Invalid URL");
    let map = client.map(&map.try_into().unwrap()).await.unwrap();


    let map_download: Bytes = client.download((&map).into()).await.unwrap();
    // save map
    let mut archive = ZipArchive::new(Cursor::new(map_download.as_ref())).unwrap();
    let directory_format = SimpleCurlyFormat.format(opts.settings.directory_format.as_ref().unwrap(), &[
        ("song_name", map.metadata.song_name.as_str()),
        ("song_author", map.metadata.song_author.as_str()),
        ("song_sub_name", map.metadata.song_sub_name.as_str()),
        ("level_author", map.metadata.level_author.as_str())
    ]).unwrap();
    let mut path = PathBuf::from(opts.settings.game_path.as_ref().unwrap());
    // TODO: ensure no ../ or / in the beginning
    path.push(directory_format.as_ref());
    create_dir_all(&path).expect("Could not create dir");
    archive.extract(&path).unwrap();

}
