#[cfg(feature = "UnityEngine+QualitySettings")]
#[repr(C)]
#[derive(Debug)]
pub struct QualitySettings {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+QualitySettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::QualitySettings => "UnityEngine"
    ."QualitySettings"
);
#[cfg(feature = "UnityEngine+QualitySettings")]
impl std::ops::Deref for crate::UnityEngine::QualitySettings {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+QualitySettings")]
impl std::ops::DerefMut for crate::UnityEngine::QualitySettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+QualitySettings")]
impl crate::UnityEngine::QualitySettings {}
#[cfg(feature = "UnityEngine+QualitySettings")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::QualitySettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
