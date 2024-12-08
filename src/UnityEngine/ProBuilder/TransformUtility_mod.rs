#[cfg(feature = "UnityEngine+ProBuilder+TransformUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct TransformUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+TransformUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::TransformUtility =>
    "UnityEngine.ProBuilder"."TransformUtility"
);
#[cfg(feature = "UnityEngine+ProBuilder+TransformUtility")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::TransformUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+TransformUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::TransformUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+TransformUtility")]
impl crate::UnityEngine::ProBuilder::TransformUtility {}
#[cfg(feature = "UnityEngine+ProBuilder+TransformUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::TransformUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
