#[cfg(feature = "BeatSaber+Settings+SettingStrings")]
#[repr(C)]
#[derive(Debug)]
pub struct SettingStrings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+Settings+SettingStrings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::SettingStrings =>
    "BeatSaber.Settings"."SettingStrings"
);
#[cfg(feature = "BeatSaber+Settings+SettingStrings")]
impl std::ops::Deref for crate::BeatSaber::Settings::SettingStrings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+SettingStrings")]
impl std::ops::DerefMut for crate::BeatSaber::Settings::SettingStrings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+SettingStrings")]
impl crate::BeatSaber::Settings::SettingStrings {}
#[cfg(feature = "BeatSaber+Settings+SettingStrings")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::Settings::SettingStrings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
