use clap::{Clap,AppSettings};
use serde::Deserialize;

const DEFAULT_GAME_PATH : &str = "~/.local/share/Steam/steamapps/common/Beat Saber/";
const DEFAULT_DIRECTORY_FORMAT : &str = "{song_author} - {song_name} ({level_author})-{id}";

#[derive(Debug,Clap)]
#[clap(name = "beatsavercli", setting = AppSettings::ColoredHelp)]
pub struct BeatSaverCli {
    #[clap(flatten)]
    pub settings: Settings,
    pub url: String,
}

#[derive(Debug,clap::Args, Deserialize)]

pub struct Settings {
    #[clap(short, long)]
    pub game_path: Option<String>,
    #[clap(short, long)]
    pub directory_format: Option<String>
}

