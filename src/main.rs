extern crate amethyst;
#[macro_use]
extern crate log;

use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::application_root_dir,
};

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn get_gltf_file() -> io::Result<PathBuf> {
    match env::args().skip(1).next() {
        Some(fp) => fs::canonicalize(fp),
        None => Err(io::Error::new(
            io::ErrorKind::Other,
            "No file path specified.",
        )),
    }
}

struct Example;

impl SimpleState for Example {}

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(amethyst::LoggerConfig {
        level_filter: amethyst::LogLevelFilter::Info,
        ..Default::default()
    })
    .level_for("gfx_device_gl", amethyst::LogLevelFilter::Warn)
    .level_for("dream78", amethyst::LogLevelFilter::Debug)
    .start();

    let fp = get_gltf_file().expect("glTF file missing or invalid");

    debug!("Loading `{}`", fp.display());

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );

    let game_data =
        GameDataBuilder::default().with_bundle(RenderBundle::new(pipe, Some(config)))?;
    let mut game = Application::new("./", Example, game_data)?;

    game.run();

    Ok(())
}
