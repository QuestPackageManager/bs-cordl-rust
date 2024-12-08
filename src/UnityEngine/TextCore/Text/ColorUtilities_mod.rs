#[cfg(feature = "UnityEngine+TextCore+Text+ColorUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorUtilities {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+TextCore+Text+ColorUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::ColorUtilities =>
    "UnityEngine.TextCore.Text"."ColorUtilities"
);
#[cfg(feature = "UnityEngine+TextCore+Text+ColorUtilities")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::ColorUtilities {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+ColorUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::ColorUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+ColorUtilities")]
impl crate::UnityEngine::TextCore::Text::ColorUtilities {}
#[cfg(feature = "UnityEngine+TextCore+Text+ColorUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::ColorUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
