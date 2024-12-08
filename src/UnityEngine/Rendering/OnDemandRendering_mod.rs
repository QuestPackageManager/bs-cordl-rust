#[cfg(feature = "UnityEngine+Rendering+OnDemandRendering")]
#[repr(C)]
#[derive(Debug)]
pub struct OnDemandRendering {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Rendering+OnDemandRendering")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::OnDemandRendering =>
    "UnityEngine.Rendering"."OnDemandRendering"
);
#[cfg(feature = "UnityEngine+Rendering+OnDemandRendering")]
impl std::ops::Deref for crate::UnityEngine::Rendering::OnDemandRendering {
    type Target = crate::System::Object;
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
impl crate::UnityEngine::Rendering::OnDemandRendering {}
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
