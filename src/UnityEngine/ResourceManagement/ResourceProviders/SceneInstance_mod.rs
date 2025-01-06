#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneInstance")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SceneInstance {
    pub m_Scene: crate::UnityEngine::SceneManagement::Scene,
    pub m_Operation: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneInstance")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance =>
    "UnityEngine.ResourceManagement.ResourceProviders"."SceneInstance"
);
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneInstance")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+ResourceProviders+SceneInstance")]
impl crate::UnityEngine::ResourceManagement::ResourceProviders::SceneInstance {
    pub fn Activate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Activate",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ActivateAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncOperation> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ActivateAsync",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Scene(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SceneManagement::Scene> {
        let __cordl_ret: crate::UnityEngine::SceneManagement::Scene = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Scene",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Scene(
        &mut self,
        value: crate::UnityEngine::SceneManagement::Scene,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Scene",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
