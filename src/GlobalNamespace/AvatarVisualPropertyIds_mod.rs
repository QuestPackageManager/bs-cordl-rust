#[cfg(feature = "AvatarVisualPropertyIds")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarVisualPropertyIds {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "AvatarVisualPropertyIds")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AvatarVisualPropertyIds => ""
    ."AvatarVisualPropertyIds"
);
#[cfg(feature = "AvatarVisualPropertyIds")]
impl std::ops::Deref for crate::GlobalNamespace::AvatarVisualPropertyIds {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AvatarVisualPropertyIds")]
impl std::ops::DerefMut for crate::GlobalNamespace::AvatarVisualPropertyIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AvatarVisualPropertyIds")]
impl crate::GlobalNamespace::AvatarVisualPropertyIds {
    pub const kKeywordPointLightIsLocal: &'static str = "POINT_LIGHT_IS_LOCAL";
}
#[cfg(feature = "AvatarVisualPropertyIds")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AvatarVisualPropertyIds {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
