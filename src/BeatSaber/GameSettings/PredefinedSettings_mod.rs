#[cfg(feature = "BeatSaber+GameSettings+PredefinedSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct PredefinedSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+GameSettings+PredefinedSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::GameSettings::PredefinedSettings =>
    "BeatSaber.GameSettings"."PredefinedSettings"
);
#[cfg(feature = "BeatSaber+GameSettings+PredefinedSettings")]
impl std::ops::Deref for crate::BeatSaber::GameSettings::PredefinedSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+PredefinedSettings")]
impl std::ops::DerefMut for crate::BeatSaber::GameSettings::PredefinedSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+PredefinedSettings")]
impl crate::BeatSaber::GameSettings::PredefinedSettings {
    pub const kControllersPositionOffsetLimit: f32 = 0.1f32;
    pub const kControllersRotationOffsetLimit: f32 = 180f32;
    pub const kDefaultPlayerHeight: f32 = 1.8f32;
    pub const kHeadPosToPlayerHeightOffset: f32 = 0.1f32;
    pub const kMaxRoomDistanceFromCenterPerAxis: f32 = 4f32;
}
#[cfg(feature = "BeatSaber+GameSettings+PredefinedSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::GameSettings::PredefinedSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
