#[cfg(feature = "UnityEngine+ProBuilder+MaterialUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct MaterialUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+MaterialUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::MaterialUtility =>
    "UnityEngine.ProBuilder"."MaterialUtility"
);
#[cfg(feature = "UnityEngine+ProBuilder+MaterialUtility")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MaterialUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MaterialUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::MaterialUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MaterialUtility")]
impl crate::UnityEngine::ProBuilder::MaterialUtility {}
#[cfg(feature = "UnityEngine+ProBuilder+MaterialUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MaterialUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
