#[cfg(feature = "PlayerMissionStatsData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerMissionStatsData {
    __cordl_parent: crate::System::Object,
    pub _missionId: *mut crate::System::String,
    pub _cleared: bool,
}
#[cfg(feature = "PlayerMissionStatsData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PlayerMissionStatsData => ""."PlayerMissionStatsData"
);
#[cfg(feature = "PlayerMissionStatsData")]
impl std::ops::Deref for PlayerMissionStatsData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerMissionStatsData")]
impl std::ops::DerefMut for PlayerMissionStatsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerMissionStatsData")]
impl PlayerMissionStatsData {
    pub fn New(
        missionId: *mut crate::System::String,
        cleared: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (missionId, cleared))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        missionId: *mut crate::System::String,
        cleared: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (missionId, cleared))?;
        Ok(__cordl_ret)
    }
    pub fn get_cleared(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_cleared", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_missionId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_missionId", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_cleared(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cleared", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "PlayerMissionStatsData")]
impl quest_hook::libil2cpp::ObjectType for PlayerMissionStatsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
