#[cfg(feature = "UnityEngine+SceneManagement+LoadSceneParameters")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct LoadSceneParameters {
    pub m_LoadSceneMode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    pub m_LocalPhysicsMode: crate::UnityEngine::SceneManagement::LocalPhysicsMode,
}
#[cfg(feature = "UnityEngine+SceneManagement+LoadSceneParameters")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::SceneManagement::LoadSceneParameters => "UnityEngine.SceneManagement"
    ."LoadSceneParameters"
);
#[cfg(feature = "UnityEngine+SceneManagement+LoadSceneParameters")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::SceneManagement::LoadSceneParameters {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+SceneManagement+LoadSceneParameters")]
impl crate::UnityEngine::SceneManagement::LoadSceneParameters {
    pub fn _ctor(
        &mut self,
        mode: crate::UnityEngine::SceneManagement::LoadSceneMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (mode),
        )?;
        Ok(__cordl_ret.into())
    }
}
