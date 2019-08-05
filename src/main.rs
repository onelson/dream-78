use amethyst::{
    animation::{AnimationBundle, VertexSkinningBundle},
    assets::PrefabLoaderSystem,
    controls::FlyControlBundle,
    core::transform::{Transform, TransformBundle},
    gltf::GltfSceneLoaderSystem,
    input::StringBindings,
    prelude::*,
    renderer::{
        plugins::{RenderPbr3D, RenderSkybox, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::{application_root_dir, auto_fov::AutoFovSystem},
};

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

#[allow(dead_code)]
fn get_gltf_file() -> io::Result<PathBuf> {
    match env::args().nth(1) {
        Some(fp) => fs::canonicalize(fp),
        None => Err(io::Error::new(
            io::ErrorKind::Other,
            "No file path specified.",
        )),
    }
}
mod prefab;
mod state;
use prefab::ScenePrefabData;
use state::Example;

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default())
        //    .level_for("gfx_device_gl", amethyst::LogLevelFilter::Warn)
        .level_for("dream78", amethyst::LogLevelFilter::Debug)
        .start();

    let app_root: PathBuf = application_root_dir()?;
    let resources_directory = app_root.join("assets/");
    let display_conf = app_root.join("resources/display_config.ron");

    let game_data = GameDataBuilder::default()
        .with(AutoFovSystem::default(), "auto_fov", &[])
        .with(
            PrefabLoaderSystem::<ScenePrefabData>::default(),
            "scene_loader",
            &[],
        )
        .with(
            GltfSceneLoaderSystem::default(),
            "gltf_loader",
            &["scene_loader"], // This is important so that entity instantiation is performed in a single frame.
        )
        .with_bundle(
            AnimationBundle::<usize, Transform>::new("animation_control", "sampler_interpolation")
                .with_dep(&["gltf_loader"]),
        )?
        .with_bundle(
            FlyControlBundle::<StringBindings>::new(None, None, None)
                .with_sensitivity(0.1, 0.1)
                .with_speed(5.),
        )?
        .with_bundle(TransformBundle::new().with_dep(&[
            "animation_control",
            "sampler_interpolation",
            "fly_movement",
        ]))?
        .with_bundle(VertexSkinningBundle::new().with_dep(&[
            "transform_system",
            "animation_control",
            "sampler_interpolation",
        ]))?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_conf))
                .with_plugin(RenderPbr3D::default().with_skinning())
                .with_plugin(RenderSkybox::default()),
        )?;

    let mut game = Application::build(resources_directory, Example::default())?.build(game_data)?;
    game.run();
    Ok(())
}
