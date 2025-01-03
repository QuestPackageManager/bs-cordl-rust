#[cfg(feature = "UnityEngine+Rendering+OnDemandRendering")]
#[repr(C)]
#[derive(Debug)]
pub struct OnDemandRendering {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Rendering+OnDemandRendering")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::OnDemandRendering =>
    "UnityEngine.Rendering"."OnDemandRendering"
);
#[cfg(feature = "UnityEngine+Rendering+OnDemandRendering")]
impl std::ops::Deref for crate::UnityEngine::Rendering::OnDemandRendering {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OnDemandRendering")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::OnDemandRendering {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+OnDemandRendering")]
impl crate::UnityEngine::Rendering::OnDemandRendering {
    pub fn GetRenderFrameInterval(
        frameInterval: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRenderFrameInterval", (frameInterval))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_renderFrameInterval() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_renderFrameInterval", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+OnDemandRendering")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::OnDemandRendering {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
