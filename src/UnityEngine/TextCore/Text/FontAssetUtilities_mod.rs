#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct FontAssetUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetUtilities")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::FontAssetUtilities
    => "UnityEngine.TextCore.Text"."FontAssetUtilities"
);
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetUtilities")]
impl std::ops::Deref for crate::UnityEngine::TextCore::Text::FontAssetUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetUtilities")]
impl std::ops::DerefMut for crate::UnityEngine::TextCore::Text::FontAssetUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetUtilities")]
impl crate::UnityEngine::TextCore::Text::FontAssetUtilities {}
#[cfg(feature = "UnityEngine+TextCore+Text+FontAssetUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::TextCore::Text::FontAssetUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
