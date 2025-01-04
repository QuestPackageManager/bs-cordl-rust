#[cfg(feature = "UnityEngine+SceneManagement+UnloadSceneOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnloadSceneOptions {
    #[default]
    None = 0i32,
    UnloadAllEmbeddedSceneObjects = 1i32,
}
#[cfg(feature = "UnityEngine+SceneManagement+UnloadSceneOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SceneManagement::UnloadSceneOptions
    => "UnityEngine.SceneManagement"."UnloadSceneOptions"
);
