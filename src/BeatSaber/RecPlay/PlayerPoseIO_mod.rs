#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseIO")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerPoseIO {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseIO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::RecPlay::PlayerPoseIO =>
    "BeatSaber.RecPlay"."PlayerPoseIO"
);
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseIO")]
impl std::ops::Deref for crate::BeatSaber::RecPlay::PlayerPoseIO {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseIO")]
impl std::ops::DerefMut for crate::BeatSaber::RecPlay::PlayerPoseIO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseIO")]
impl crate::BeatSaber::RecPlay::PlayerPoseIO {}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseIO")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::RecPlay::PlayerPoseIO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
