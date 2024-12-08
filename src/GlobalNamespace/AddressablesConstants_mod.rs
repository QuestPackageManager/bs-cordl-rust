#[cfg(feature = "AddressablesConstants")]
#[repr(C)]
#[derive(Debug)]
pub struct AddressablesConstants {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "AddressablesConstants")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AddressablesConstants => ""
    ."AddressablesConstants"
);
#[cfg(feature = "AddressablesConstants")]
impl std::ops::Deref for crate::GlobalNamespace::AddressablesConstants {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AddressablesConstants")]
impl std::ops::DerefMut for crate::GlobalNamespace::AddressablesConstants {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
#[cfg(feature = "AddressablesConstants")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AddressablesConstants {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
