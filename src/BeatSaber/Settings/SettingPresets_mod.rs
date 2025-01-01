#[cfg(feature = "BeatSaber+Settings+SettingPresets")]
#[repr(C)]
#[derive(Debug)]
pub struct SettingPresets {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+Settings+SettingPresets")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::SettingPresets =>
    "BeatSaber.Settings"."SettingPresets"
);
#[cfg(feature = "BeatSaber+Settings+SettingPresets")]
impl std::ops::Deref for crate::BeatSaber::Settings::SettingPresets {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+SettingPresets")]
impl std::ops::DerefMut for crate::BeatSaber::Settings::SettingPresets {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+SettingPresets")]
impl crate::BeatSaber::Settings::SettingPresets {}
#[cfg(feature = "BeatSaber+Settings+SettingPresets")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::Settings::SettingPresets {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
