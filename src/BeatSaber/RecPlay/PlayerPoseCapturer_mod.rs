#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseCapturer")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerPoseCapturer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub frames: *mut crate::System::Collections::Generic::List_1<
        crate::BeatSaber::RecPlay::PlayerPoseFrame,
    >,
}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseCapturer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::RecPlay::PlayerPoseCapturer =>
    "BeatSaber.RecPlay"."PlayerPoseCapturer"
);
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseCapturer")]
impl std::ops::Deref for crate::BeatSaber::RecPlay::PlayerPoseCapturer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseCapturer")]
impl std::ops::DerefMut for crate::BeatSaber::RecPlay::PlayerPoseCapturer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseCapturer")]
impl crate::BeatSaber::RecPlay::PlayerPoseCapturer {
    pub fn Capture(
        &mut self,
        _cordl_time: f32,
        pose: crate::BeatSaber::RecPlay::PlayerPose,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Capture", (_cordl_time, pose))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePlayerPoseFrames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::RecPlay::PlayerPoseFrames> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::RecPlay::PlayerPoseFrames = __cordl_object
            .invoke("CreatePlayerPoseFrames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (capacity))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (capacity))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseCapturer")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::RecPlay::PlayerPoseCapturer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
