#[cfg(feature = "UnityEngine+TextCore+Text+TextShaderUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct TextShaderUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextShaderUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::TextShaderUtilities
    => "UnityEngine.TextCore.Text"."TextShaderUtilities"
);
#[cfg(feature = "UnityEngine+TextCore+Text+TextShaderUtilities")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::TextShaderUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextShaderUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::TextShaderUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+TextShaderUtilities")]
impl crate::UnityEngine::TextCore::Text::TextShaderUtilities {}
#[cfg(feature = "UnityEngine+TextCore+Text+TextShaderUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::TextShaderUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
