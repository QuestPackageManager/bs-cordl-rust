#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightmapperUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct LightmapperUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightmapperUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::GlobalIllumination::LightmapperUtils =>
    "UnityEngine.Experimental.GlobalIllumination"."LightmapperUtils"
);
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightmapperUtils")]
impl std::ops::Deref
for crate::UnityEngine::Experimental::GlobalIllumination::LightmapperUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightmapperUtils")]
impl std::ops::DerefMut
for crate::UnityEngine::Experimental::GlobalIllumination::LightmapperUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightmapperUtils")]
impl crate::UnityEngine::Experimental::GlobalIllumination::LightmapperUtils {}
#[cfg(feature = "UnityEngine+Experimental+GlobalIllumination+LightmapperUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Experimental::GlobalIllumination::LightmapperUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
