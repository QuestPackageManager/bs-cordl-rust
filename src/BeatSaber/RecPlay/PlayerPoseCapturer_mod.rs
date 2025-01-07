#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseCapturer")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerPoseCapturer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub frames: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::BeatSaber::RecPlay::PlayerPoseFrame,
        >,
    >,
}
#[cfg(feature = "BeatSaber+RecPlay+PlayerPoseCapturer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::RecPlay::PlayerPoseCapturer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.RecPlay";
    const CLASS_NAME: &'static str = "PlayerPoseCapturer";
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
