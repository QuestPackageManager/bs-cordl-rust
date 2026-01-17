#[cfg(feature = "cordl_class_PredefinedSettings")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct PredefinedSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_PredefinedSettings")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::PredefinedSettings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PredefinedSettings";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "PredefinedSettings")]
impl std::ops::Deref for crate::GlobalNamespace::PredefinedSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PredefinedSettings")]
impl std::ops::DerefMut for crate::GlobalNamespace::PredefinedSettings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PredefinedSettings")]
impl crate::GlobalNamespace::PredefinedSettings {
    pub const kControllersPositionOffsetLimit: f32 = 0.1f32;
    pub const kControllersRotationOffsetLimit: f32 = 180f32;
    pub const kDefaultPlayerHeight: f32 = 1.8f32;
    pub const kHeadPosToPlayerHeightOffset: f32 = 0.1f32;
    pub const kMaxRoomDistanceFromCenterPerAxis: f32 = 4f32;
    pub const kQuest120HzFramerate: i32 = 120i32;
}
#[cfg(feature = "cordl_class_PredefinedSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PredefinedSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
