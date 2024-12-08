#[cfg(feature = "LeaderboardIdsSO+LeaderboardIdData")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardIdsSO_LeaderboardIdData {
    __cordl_parent: crate::System::Object,
    pub _difficultyBeatmapId: *mut crate::System::String,
    pub _platformLeaderboardId: *mut crate::System::String,
}
#[cfg(feature = "LeaderboardIdsSO+LeaderboardIdData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData => ""
    ."LeaderboardIdsSO/LeaderboardIdData"
);
#[cfg(feature = "LeaderboardIdsSO+LeaderboardIdData")]
impl std::ops::Deref for crate::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardIdsSO+LeaderboardIdData")]
impl std::ops::DerefMut for crate::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardIdsSO+LeaderboardIdData")]
impl crate::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData {
    pub fn New(
        difficultyBeatmapId: *mut crate::System::String,
        platformLeaderboardId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (difficultyBeatmapId, platformLeaderboardId))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        difficultyBeatmapId: *mut crate::System::String,
        platformLeaderboardId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (difficultyBeatmapId, platformLeaderboardId))?;
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
    pub fn get_platformLeaderboardId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_platformLeaderboardId", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LeaderboardIdsSO+LeaderboardIdData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LeaderboardIdsSO")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardIdsSO {
    __cordl_parent: PersistentScriptableObject,
    pub _leaderboardIds: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData,
    >,
}
#[cfg(feature = "LeaderboardIdsSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LeaderboardIdsSO => ""."LeaderboardIdsSO"
);
#[cfg(feature = "LeaderboardIdsSO")]
impl std::ops::Deref for LeaderboardIdsSO {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardIdsSO")]
impl std::ops::DerefMut for LeaderboardIdsSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardIdsSO")]
impl LeaderboardIdsSO {
    #[cfg(feature = "LeaderboardIdsSO+LeaderboardIdData")]
    pub type LeaderboardIdData = crate::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData;
    pub fn Add(
        &mut self,
        difficultyBeatmapId: *mut crate::System::String,
        platformLeaderboardId: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (difficultyBeatmapId, platformLeaderboardId))?;
        Ok(__cordl_ret)
    }
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
            *mut crate::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData,
        > = __cordl_object.invoke("get_leaderboardIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_leaderboardIds(
        &mut self,
        value: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::LeaderboardIdsSO_LeaderboardIdData,
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
#[cfg(feature = "LeaderboardIdsSO")]
impl quest_hook::libil2cpp::ObjectType for LeaderboardIdsSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
