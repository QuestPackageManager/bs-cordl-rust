#[cfg(feature = "BeatmapEventDataLightsExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEventDataLightsExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapEventDataLightsExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapEventDataLightsExtensions => ""
    ."BeatmapEventDataLightsExtensions"
);
#[cfg(feature = "BeatmapEventDataLightsExtensions")]
impl std::ops::Deref for BeatmapEventDataLightsExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataLightsExtensions")]
impl std::ops::DerefMut for BeatmapEventDataLightsExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataLightsExtensions")]
impl BeatmapEventDataLightsExtensions {
    #[cfg(feature = "BeatmapEventDataLightsExtensions+LightSwitchEventEffectDataValues")]
    pub type LightSwitchEventEffectDataValues = crate::GlobalNamespace::BeatmapEventDataLightsExtensions_LightSwitchEventEffectDataValues;
}
#[cfg(feature = "BeatmapEventDataLightsExtensions")]
impl quest_hook::libil2cpp::ObjectType for BeatmapEventDataLightsExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapEventDataLightsExtensions+LightSwitchEventEffectDataValues")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEventDataLightsExtensions_LightSwitchEventEffectDataValues {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapEventDataLightsExtensions+LightSwitchEventEffectDataValues")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapEventDataLightsExtensions_LightSwitchEventEffectDataValues
    => ""."BeatmapEventDataLightsExtensions/LightSwitchEventEffectDataValues"
);
#[cfg(feature = "BeatmapEventDataLightsExtensions+LightSwitchEventEffectDataValues")]
impl std::ops::Deref
for crate::GlobalNamespace::BeatmapEventDataLightsExtensions_LightSwitchEventEffectDataValues {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataLightsExtensions+LightSwitchEventEffectDataValues")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapEventDataLightsExtensions_LightSwitchEventEffectDataValues {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataLightsExtensions+LightSwitchEventEffectDataValues")]
impl crate::GlobalNamespace::BeatmapEventDataLightsExtensions_LightSwitchEventEffectDataValues {
    pub const kFadeA: i32 = 4i32;
    pub const kFadeB: i32 = 8i32;
    pub const kFadeW: i32 = 12i32;
    pub const kFlashAndFadeToBlack: i32 = -1i32;
    pub const kFlashAndFadeToBlackA: i32 = 3i32;
    pub const kFlashAndFadeToBlackB: i32 = 7i32;
    pub const kFlashAndFadeToBlackW: i32 = 11i32;
    pub const kHighlightA: i32 = 2i32;
    pub const kHighlightB: i32 = 6i32;
    pub const kHighlightW: i32 = 10i32;
    pub const kOff: i32 = 0i32;
    pub const kOnA: i32 = 1i32;
    pub const kOnB: i32 = 5i32;
    pub const kOnW: i32 = 9i32;
}
#[cfg(feature = "BeatmapEventDataLightsExtensions+LightSwitchEventEffectDataValues")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapEventDataLightsExtensions_LightSwitchEventEffectDataValues {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
