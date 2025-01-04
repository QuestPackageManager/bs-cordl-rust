#[cfg(feature = "UnityEngine+SceneManagement+LocalPhysicsMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LocalPhysicsMode {
    #[default]
    None = 0i32,
    Physics2D = 1i32,
    Physics3D = 2i32,
}
#[cfg(feature = "UnityEngine+SceneManagement+LocalPhysicsMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SceneManagement::LocalPhysicsMode
    => "UnityEngine.SceneManagement"."LocalPhysicsMode"
);
