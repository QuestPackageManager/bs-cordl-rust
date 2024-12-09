#[cfg(feature = "UnityEngine+RectTransformUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct RectTransformUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+RectTransformUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RectTransformUtility =>
    "UnityEngine"."RectTransformUtility"
);
#[cfg(feature = "UnityEngine+RectTransformUtility")]
impl std::ops::Deref for crate::UnityEngine::RectTransformUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+RectTransformUtility")]
impl std::ops::DerefMut for crate::UnityEngine::RectTransformUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+RectTransformUtility")]
impl crate::UnityEngine::RectTransformUtility {}
#[cfg(feature = "UnityEngine+RectTransformUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::RectTransformUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
