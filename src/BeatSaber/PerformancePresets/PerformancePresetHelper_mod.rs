#[cfg(feature = "BeatSaber+PerformancePresets+PerformancePresetHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct PerformancePresetHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatSaber+PerformancePresets+PerformancePresetHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::PerformancePresets::PerformancePresetHelper =>
    "BeatSaber.PerformancePresets"."PerformancePresetHelper"
);
#[cfg(feature = "BeatSaber+PerformancePresets+PerformancePresetHelper")]
impl std::ops::Deref for crate::BeatSaber::PerformancePresets::PerformancePresetHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+PerformancePresets+PerformancePresetHelper")]
impl std::ops::DerefMut
for crate::BeatSaber::PerformancePresets::PerformancePresetHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+PerformancePresets+PerformancePresetHelper")]
impl crate::BeatSaber::PerformancePresets::PerformancePresetHelper {
    pub const kAddressableGroupName: &'static str = "PerformancePresets";
    pub const kCustomPresetKey: &'static str = "Custom";
    pub const kDefaultPresetKey: &'static str = "PlatformDefaultPerformancePreset";
    pub const kPS4HWDeviceCategoryName: &'static str = "Orbis";
    pub const kPS4ProHWDeviceCategoryName: &'static str = "Neo";
    pub const kPS5HWDeviceCategoryName: &'static str = "Prospero";
    pub const kPerformancePresetKey: &'static str = "PerformancePreset";
    pub const kQuest2HWDeviceCategoryName: &'static str = "Miramar";
    pub const kQuest3HWDeviceCategoryName: &'static str = "Stinson";
    pub const kQuestHWDeviceCategoryName: &'static str = "Monterey";
    pub const kQuestProHWDeviceCategoryName: &'static str = "Arcata";
    pub const kStandaloneHWDeviceCategoryName: &'static str = "Standalone";
    #[cfg(
        feature = "BeatSaber+PerformancePresets+PerformancePresetHelper+_IsPerformancePresetAvailableAsync_d__12"
    )]
    pub type _IsPerformancePresetAvailableAsync_d__12 = crate::BeatSaber::PerformancePresets::PerformancePresetHelper__IsPerformancePresetAvailableAsync_d__12;
    #[cfg(feature = "BeatSaber+PerformancePresets+PerformancePresetHelper+__c")]
    pub type __c = crate::BeatSaber::PerformancePresets::PerformancePresetHelper___c;
    #[cfg(
        feature = "BeatSaber+PerformancePresets+PerformancePresetHelper+_GetDefaultPerformancePresetAsync_d__14"
    )]
    pub type _GetDefaultPerformancePresetAsync_d__14 = crate::BeatSaber::PerformancePresets::PerformancePresetHelper__GetDefaultPerformancePresetAsync_d__14;
    #[cfg(
        feature = "BeatSaber+PerformancePresets+PerformancePresetHelper+_GetPerformancePresetAsync_d__13"
    )]
    pub type _GetPerformancePresetAsync_d__13 = crate::BeatSaber::PerformancePresets::PerformancePresetHelper__GetPerformancePresetAsync_d__13;
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatSaber+PerformancePresets+PerformancePresetHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::PerformancePresets::PerformancePresetHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
