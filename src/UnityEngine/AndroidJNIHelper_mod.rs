#[cfg(feature = "UnityEngine+AndroidJNIHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidJNIHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+AndroidJNIHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AndroidJNIHelper => "UnityEngine"
    ."AndroidJNIHelper"
);
#[cfg(feature = "UnityEngine+AndroidJNIHelper")]
impl std::ops::Deref for crate::UnityEngine::AndroidJNIHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJNIHelper")]
impl std::ops::DerefMut for crate::UnityEngine::AndroidJNIHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJNIHelper")]
impl crate::UnityEngine::AndroidJNIHelper {}
#[cfg(feature = "UnityEngine+AndroidJNIHelper")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AndroidJNIHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
