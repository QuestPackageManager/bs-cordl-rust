#[cfg(feature = "UnityEngine+ImageConversion")]
#[repr(C)]
#[derive(Debug)]
pub struct ImageConversion {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ImageConversion")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ImageConversion => "UnityEngine"
    ."ImageConversion"
);
#[cfg(feature = "UnityEngine+ImageConversion")]
impl std::ops::Deref for crate::UnityEngine::ImageConversion {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ImageConversion")]
impl std::ops::DerefMut for crate::UnityEngine::ImageConversion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ImageConversion")]
impl crate::UnityEngine::ImageConversion {}
#[cfg(feature = "UnityEngine+ImageConversion")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ImageConversion {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
