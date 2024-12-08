#[cfg(feature = "UnityEngine+AndroidJNI")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidJNI {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+AndroidJNI")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AndroidJNI => "UnityEngine"
    ."AndroidJNI"
);
#[cfg(feature = "UnityEngine+AndroidJNI")]
impl std::ops::Deref for crate::UnityEngine::AndroidJNI {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJNI")]
impl std::ops::DerefMut for crate::UnityEngine::AndroidJNI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJNI")]
impl crate::UnityEngine::AndroidJNI {}
#[cfg(feature = "UnityEngine+AndroidJNI")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AndroidJNI {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
