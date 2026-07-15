#![allow(non_snake_case)]
use std::{
    path::{Path, PathBuf},
    fs::File,
    io::BufReader,
};

use serde::Deserialize;

pub fn parse_mcx_json(config_file: &Path) -> McxScene{
    let file = File::open(config_file)
        .expect("Cannot find config file given.");
    
    let reader = BufReader::new(file);

    serde_json::from_reader(reader)
        .expect("JSON config file is in an invalid format")
}

#[derive(Deserialize, Debug)]
pub struct McxScene {
    pub Session: Session,
    pub Forward: Forward,
    pub Optode: Optode,
    pub Domain: Domain,
}

#[derive(Deserialize, Debug)]
pub struct Session {
    pub ID: String,
    pub DoMismatch: f64,
    pub OutputType: String,
    pub RNGSeed: f64,
    pub Photons: f64
}

#[derive(Deserialize, Debug)]
pub struct Forward {
    pub T0: f64,
    pub T1: f64,
    pub Dt: f64,
}

#[derive(Deserialize, Debug)]
pub struct Optode {
    pub Source: Source
}

#[derive(Deserialize, Debug)]
pub struct Source {
    pub Pos: [f64; 3],
    pub Dir: [f64; 3],
    pub Type: String
}

#[derive(Deserialize, Debug)]
pub struct Domain {
    pub OriginType: f64,
    pub Media: Vec<Media>,
    pub Dims: [f64; 3]
}

#[derive(Deserialize, Debug)]
pub struct Media {
    pub mua: f64,
    pub mus: f64,
    pub g: f64,
    pub n: f64,
}