#[cfg(feature = "UnityEngine+AndroidJNISafe")]
#[repr(C)]
#[derive(Debug)]
pub struct AndroidJNISafe {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+AndroidJNISafe")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AndroidJNISafe => "UnityEngine"
    ."AndroidJNISafe"
);
#[cfg(feature = "UnityEngine+AndroidJNISafe")]
impl std::ops::Deref for crate::UnityEngine::AndroidJNISafe {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJNISafe")]
impl std::ops::DerefMut for crate::UnityEngine::AndroidJNISafe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AndroidJNISafe")]
impl crate::UnityEngine::AndroidJNISafe {}
#[cfg(feature = "UnityEngine+AndroidJNISafe")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AndroidJNISafe {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}