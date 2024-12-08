#[cfg(feature = "ILeaderboardIdContainer")]
#[repr(C)]
#[derive(Debug)]
pub struct ILeaderboardIdContainer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ILeaderboardIdContainer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ILeaderboardIdContainer => ""."ILeaderboardIdContainer"
);
#[cfg(feature = "ILeaderboardIdContainer")]
impl std::ops::Deref for ILeaderboardIdContainer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ILeaderboardIdContainer")]
impl std::ops::DerefMut for ILeaderboardIdContainer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ILeaderboardIdContainer")]
impl ILeaderboardIdContainer {
    pub fn get_leaderboardIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<*mut ILeaderboardIdData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut ILeaderboardIdData,
        > = __cordl_object.invoke("get_leaderboardIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "ILeaderboardIdContainer")]
impl quest_hook::libil2cpp::ObjectType for ILeaderboardIdContainer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
