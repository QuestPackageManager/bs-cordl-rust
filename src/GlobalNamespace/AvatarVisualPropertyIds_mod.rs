#[cfg(feature = "AvatarVisualPropertyIds")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarVisualPropertyIds {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "AvatarVisualPropertyIds")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for AvatarVisualPropertyIds => ""."AvatarVisualPropertyIds"
);
#[cfg(feature = "AvatarVisualPropertyIds")]
impl std::ops::Deref for AvatarVisualPropertyIds {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AvatarVisualPropertyIds")]
impl std::ops::DerefMut for AvatarVisualPropertyIds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AvatarVisualPropertyIds")]
impl AvatarVisualPropertyIds {
    pub const kKeywordPointLightIsLocal: &'static str = "POINT_LIGHT_IS_LOCAL";
}
#[cfg(feature = "AvatarVisualPropertyIds")]
impl quest_hook::libil2cpp::ObjectType for AvatarVisualPropertyIds {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
