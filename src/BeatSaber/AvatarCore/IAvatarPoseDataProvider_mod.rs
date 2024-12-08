#[cfg(feature = "BeatSaber+AvatarCore+IAvatarPoseDataProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IAvatarPoseDataProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarPoseDataProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::AvatarCore::IAvatarPoseDataProvider
    => "BeatSaber.AvatarCore"."IAvatarPoseDataProvider"
);
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarPoseDataProvider")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::IAvatarPoseDataProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarPoseDataProvider")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::IAvatarPoseDataProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarPoseDataProvider")]
impl crate::BeatSaber::AvatarCore::IAvatarPoseDataProvider {
    pub fn get_currentPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::AvatarCore::AvatarPoseData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::AvatarCore::AvatarPoseData = __cordl_object
            .invoke("get_currentPose", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_poseDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<crate::BeatSaber::AvatarCore::AvatarPoseData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_poseDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_poseDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<crate::BeatSaber::AvatarCore::AvatarPoseData>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_poseDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarPoseDataProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::IAvatarPoseDataProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
