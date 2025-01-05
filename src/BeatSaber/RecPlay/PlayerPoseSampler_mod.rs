#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseSampler")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerPoseSampler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub offsets: crate::BeatSaber::RecPlay::PoseOffsets,
    pub frames: crate::BeatSaber::RecPlay::PlayerPoseFrames,
    pub _headNearestFrame: i32,
    pub _leftHandNearestFrame: i32,
    pub _rightHandNearestFrame: i32,
}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseSampler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::RecPlay::PlayerPoseSampler =>
    "BeatSaber.RecPlay"."PlayerPoseSampler"
);
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseSampler")]
impl std::ops::Deref for crate::BeatSaber::RecPlay::PlayerPoseSampler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseSampler")]
impl std::ops::DerefMut for crate::BeatSaber::RecPlay::PlayerPoseSampler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseSampler")]
impl crate::BeatSaber::RecPlay::PlayerPoseSampler {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Sample(
        &mut self,
        _cordl_time: f32,
        player: quest_hook::libil2cpp::ByRefMut<crate::BeatSaber::RecPlay::PlayerPose>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Sample", (_cordl_time, player))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseSampler")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::RecPlay::PlayerPoseSampler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
