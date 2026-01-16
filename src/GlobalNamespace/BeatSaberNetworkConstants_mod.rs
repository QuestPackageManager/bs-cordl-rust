#[cfg(feature = "cordl_class_BeatSaberNetworkConstants")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatSaberNetworkConstants {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_BeatSaberNetworkConstants")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::BeatSaberNetworkConstants {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatSaberNetworkConstants";
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
#[cfg(feature = "BeatSaberNetworkConstants")]
impl std::ops::Deref for crate::GlobalNamespace::BeatSaberNetworkConstants {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaberNetworkConstants")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatSaberNetworkConstants {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaberNetworkConstants")]
impl crate::GlobalNamespace::BeatSaberNetworkConstants {
    pub const dedicatedServerState: &'static str = "dedicated_server";
    pub const finishedLevel: &'static str = "finished_level";
    pub const isActive: &'static str = "is_active";
    pub const wantsToPlayNextLevel: &'static str = "wants_to_play_next_level";
    pub const wasActiveAtLevelStart: &'static str = "was_active_at_level_start";
}
#[cfg(feature = "cordl_class_BeatSaberNetworkConstants")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatSaberNetworkConstants {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
