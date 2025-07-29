#[cfg(feature = "cordl_class_AddressablesConstants")]
#[repr(C)]
#[derive(Debug)]
pub struct AddressablesConstants {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_AddressablesConstants")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::AddressablesConstants {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "AddressablesConstants";
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
#[cfg(feature = "cordl_class_AddressablesConstants")]
impl std::ops::Deref for crate::GlobalNamespace::AddressablesConstants {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_AddressablesConstants")]
impl std::ops::DerefMut for crate::GlobalNamespace::AddressablesConstants {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AddressablesConstants")]
impl crate::GlobalNamespace::AddressablesConstants {
    pub const kAllBeatmapCharacteristicsCollectionKey: &'static str = "AllBeatmapCharacteristicsCollection";
    pub const kAvatarAdapterInstallerLabel: &'static str = "AvatarAdapterInstaller";
    pub const kColorSchemeLabel: &'static str = "ColorScheme";
    pub const kEnvironmentInfoLabel: &'static str = "EnvironmentInfo";
    pub const kEnvironmentTracksDefinitionLabel: &'static str = "EnvironmentTracksDefinition";
    pub const kGameCreditsLabel: &'static str = "GameCredits";
    pub const kMusicPackCreditsLabel: &'static str = "MusicPackCredits";
    pub const kPackDefinitionLabelKey: &'static str = "PackDefinition";
    pub const kResultEnvironmentKey: &'static str = "ResultEnvironment";
    pub const kTextMeshProFontKey: &'static str = "TextMeshProFont";
}
#[cfg(feature = "cordl_class_AddressablesConstants")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AddressablesConstants {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
