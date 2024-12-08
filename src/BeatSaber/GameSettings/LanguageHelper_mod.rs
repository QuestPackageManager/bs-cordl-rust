#[cfg(feature = "BeatSaber+GameSettings+LanguageHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct LanguageHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatSaber+GameSettings+LanguageHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::GameSettings::LanguageHelper =>
    "BeatSaber.GameSettings"."LanguageHelper"
);
#[cfg(feature = "BeatSaber+GameSettings+LanguageHelper")]
impl std::ops::Deref for crate::BeatSaber::GameSettings::LanguageHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+LanguageHelper")]
impl std::ops::DerefMut for crate::BeatSaber::GameSettings::LanguageHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+LanguageHelper")]
impl crate::BeatSaber::GameSettings::LanguageHelper {}
#[cfg(feature = "BeatSaber+GameSettings+LanguageHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::GameSettings::LanguageHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
