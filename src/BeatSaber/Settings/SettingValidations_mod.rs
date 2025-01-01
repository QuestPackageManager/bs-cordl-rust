#[cfg(feature = "BeatSaber+Settings+SettingValidations")]
#[repr(C)]
#[derive(Debug)]
pub struct SettingValidations {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+Settings+SettingValidations")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::Settings::SettingValidations =>
    "BeatSaber.Settings"."SettingValidations"
);
#[cfg(feature = "BeatSaber+Settings+SettingValidations")]
impl std::ops::Deref for crate::BeatSaber::Settings::SettingValidations {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+SettingValidations")]
impl std::ops::DerefMut for crate::BeatSaber::Settings::SettingValidations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+Settings+SettingValidations")]
impl crate::BeatSaber::Settings::SettingValidations {}
#[cfg(feature = "BeatSaber+Settings+SettingValidations")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::Settings::SettingValidations {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
