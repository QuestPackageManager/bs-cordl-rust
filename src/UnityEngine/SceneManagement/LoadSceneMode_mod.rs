#[cfg(feature = "UnityEngine+SceneManagement+LoadSceneMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LoadSceneMode {
    #[default]
    Additive = 1i32,
    Single = 0i32,
}
#[cfg(feature = "UnityEngine+SceneManagement+LoadSceneMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SceneManagement::LoadSceneMode =>
    "UnityEngine.SceneManagement"."LoadSceneMode"
);
