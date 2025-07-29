#[cfg(feature = "cordl_class_AvatarVisualPropertyIds")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarVisualPropertyIds {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_AvatarVisualPropertyIds")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::AvatarVisualPropertyIds {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "AvatarVisualPropertyIds";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "AvatarVisualPropertyIds")]
impl std::ops::Deref for crate::GlobalNamespace::AvatarVisualPropertyIds {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AvatarVisualPropertyIds")]
impl std::ops::DerefMut for crate::GlobalNamespace::AvatarVisualPropertyIds {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AvatarVisualPropertyIds")]
impl crate::GlobalNamespace::AvatarVisualPropertyIds {
    pub const kKeywordPointLightIsLocal: &'static str = "POINT_LIGHT_IS_LOCAL";
}
#[cfg(feature = "cordl_class_AvatarVisualPropertyIds")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AvatarVisualPropertyIds {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
