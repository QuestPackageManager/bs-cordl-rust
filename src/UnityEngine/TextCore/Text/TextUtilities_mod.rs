#[cfg(feature = "UnityEngine+TextCore+Text+TextUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TextUtilities {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::TextUtilities =>
    "UnityEngine.TextCore.Text"."TextUtilities"
);
#[cfg(feature = "UnityEngine+TextCore+Text+TextUtilities")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::TextUtilities {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::TextUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextUtilities")]
impl crate::UnityEngine::TextCore::Text::TextUtilities {}
#[cfg(feature = "UnityEngine+TextCore+Text+TextUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::TextUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
