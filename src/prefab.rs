//! asset/prefab stuff

use amethyst::{
    assets::{AssetPrefab, Handle, Prefab, PrefabData, ProgressCounter},
    controls::ControlTagPrefab,
    core::transform::Transform,
    derive::PrefabData,
    ecs::Entity,
    gltf::{GltfSceneAsset, GltfSceneFormat},
    renderer::camera::CameraPrefab,
    renderer::light::LightPrefab,
    utils::tag::Tag,
};

// This needs to be in scope for the PrefabData derive to work.
// Isolating this import is why I put this struct in a module.
use amethyst::Error;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub(crate) struct Scene {
    pub(crate) handle: Option<Handle<Prefab<ScenePrefabData>>>,
    pub(crate) animation_index: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct AnimationMarker;

#[derive(Default, Deserialize, Serialize, PrefabData)]
#[serde(default)]
pub(crate) struct ScenePrefabData {
    transform: Option<Transform>,
    gltf: Option<AssetPrefab<GltfSceneAsset, GltfSceneFormat>>,
    camera: Option<CameraPrefab>,
    light: Option<LightPrefab>,
    tag: Option<Tag<AnimationMarker>>,
    fly_tag: Option<ControlTagPrefab>,
}
