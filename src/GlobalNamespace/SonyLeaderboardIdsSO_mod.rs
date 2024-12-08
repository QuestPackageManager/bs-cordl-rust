#[cfg(feature = "SonyLeaderboardIdsSO+LeaderboardIdData")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyLeaderboardIdsSO_LeaderboardIdData {
    __cordl_parent: crate::System::Object,
    pub _difficultyBeatmapId: *mut crate::System::String,
    pub _sonyLeaderboardId: u32,
}
#[cfg(feature = "SonyLeaderboardIdsSO+LeaderboardIdData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData => ""
    ."SonyLeaderboardIdsSO/LeaderboardIdData"
);
#[cfg(feature = "SonyLeaderboardIdsSO+LeaderboardIdData")]
impl std::ops::Deref for crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLeaderboardIdsSO+LeaderboardIdData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLeaderboardIdsSO+LeaderboardIdData")]
impl crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData {
    pub fn New(
        sonyLeaderboardId: u32,
        difficultyBeatmapId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sonyLeaderboardId, difficultyBeatmapId))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        sonyLeaderboardId: u32,
        difficultyBeatmapId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sonyLeaderboardId, difficultyBeatmapId))?;
        Ok(__cordl_ret)
    }
    pub fn get_difficultyBeatmapId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_difficultyBeatmapId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sonyLeaderboardId(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_sonyLeaderboardId", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SonyLeaderboardIdsSO+LeaderboardIdData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SonyLeaderboardIdsSO")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyLeaderboardIdsSO {
    __cordl_parent: PersistentScriptableObject,
    pub _leaderboardIds: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData,
    >,
}
#[cfg(feature = "SonyLeaderboardIdsSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SonyLeaderboardIdsSO => ""."SonyLeaderboardIdsSO"
);
#[cfg(feature = "SonyLeaderboardIdsSO")]
impl std::ops::Deref for SonyLeaderboardIdsSO {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLeaderboardIdsSO")]
impl std::ops::DerefMut for SonyLeaderboardIdsSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLeaderboardIdsSO")]
impl SonyLeaderboardIdsSO {
    #[cfg(feature = "SonyLeaderboardIdsSO+LeaderboardIdData")]
    pub type LeaderboardIdData = crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData;
    pub fn ILeaderboardIdContainer_get_leaderboardIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<*mut ILeaderboardIdData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut ILeaderboardIdData,
        > = __cordl_object.invoke("ILeaderboardIdContainer.get_leaderboardIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_leaderboardIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData,
        > = __cordl_object.invoke("get_leaderboardIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_leaderboardIds(
        &mut self,
        value: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::SonyLeaderboardIdsSO_LeaderboardIdData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_leaderboardIds", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SonyLeaderboardIdsSO")]
impl quest_hook::libil2cpp::ObjectType for SonyLeaderboardIdsSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
