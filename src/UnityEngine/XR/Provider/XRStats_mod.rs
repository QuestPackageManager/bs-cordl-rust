#[cfg(feature = "UnityEngine+XR+Provider+XRStats")]
#[repr(C)]
#[derive(Debug)]
pub struct XRStats {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+XR+Provider+XRStats")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::Provider::XRStats =>
    "UnityEngine.XR.Provider"."XRStats"
);
#[cfg(feature = "UnityEngine+XR+Provider+XRStats")]
impl std::ops::Deref for crate::UnityEngine::XR::Provider::XRStats {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+Provider+XRStats")]
impl std::ops::DerefMut for crate::UnityEngine::XR::Provider::XRStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+XR+Provider+XRStats")]
impl crate::UnityEngine::XR::Provider::XRStats {}
#[cfg(feature = "UnityEngine+XR+Provider+XRStats")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::XR::Provider::XRStats {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
