#[cfg(feature = "UnityEngine+Android+AndroidApp")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidApp {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Android+AndroidApp")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Android::AndroidApp =>
    "UnityEngine.Android"."AndroidApp"
);
#[cfg(feature = "UnityEngine+Android+AndroidApp")]
impl std::ops::Deref for crate::UnityEngine::Android::AndroidApp {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidApp")]
impl std::ops::DerefMut for crate::UnityEngine::Android::AndroidApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+AndroidApp")]
impl crate::UnityEngine::Android::AndroidApp {}
#[cfg(feature = "UnityEngine+Android+AndroidApp")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Android::AndroidApp {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
