#[cfg(feature = "MultiplayerLevelCompletionResults")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLevelCompletionResults {
    __cordl_parent: crate::System::Object,
    pub _playerLevelEndState: crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState,
    pub _playerLevelEndReason: crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason,
    pub _levelCompletionResults: *mut LevelCompletionResults,
}
#[cfg(feature = "MultiplayerLevelCompletionResults")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerLevelCompletionResults => ""
    ."MultiplayerLevelCompletionResults"
);
#[cfg(feature = "MultiplayerLevelCompletionResults")]
impl std::ops::Deref for MultiplayerLevelCompletionResults {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLevelCompletionResults")]
impl std::ops::DerefMut for MultiplayerLevelCompletionResults {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLevelCompletionResults")]
impl MultiplayerLevelCompletionResults {
    #[cfg(feature = "MultiplayerLevelCompletionResults+MultiplayerPlayerLevelEndReason")]
    pub type MultiplayerPlayerLevelEndReason = crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason;
    #[cfg(feature = "MultiplayerLevelCompletionResults+MultiplayerPlayerLevelEndState")]
    pub type MultiplayerPlayerLevelEndState = crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState;
    pub fn CompareTo(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn CreateFromSerializedData(
        &mut self,
        reader: *mut crate::LiteNetLib::Utils::NetDataReader,
    ) -> quest_hook::libil2cpp::Result<*mut MultiplayerLevelCompletionResults> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MultiplayerLevelCompletionResults = __cordl_object
            .invoke("CreateFromSerializedData", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState_MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason_LevelCompletionResults1(
        playerLevelEndState: crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState,
        playerLevelEndReason: crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason,
        levelCompletionResults: *mut LevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (playerLevelEndState, playerLevelEndReason, levelCompletionResults),
            )?;
        Ok(__cordl_object)
    }
    pub fn Serialize(
        &mut self,
        writer: *mut crate::LiteNetLib::Utils::NetDataWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState_MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason_LevelCompletionResults1(
        &mut self,
        playerLevelEndState: crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState,
        playerLevelEndReason: crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason,
        levelCompletionResults: *mut LevelCompletionResults,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (playerLevelEndState, playerLevelEndReason, levelCompletionResults),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_failedOrGivenUp(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_failedOrGivenUp", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hasAnyResults(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasAnyResults", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_levelCompletionResults(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut LevelCompletionResults> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut LevelCompletionResults = __cordl_object
            .invoke("get_levelCompletionResults", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playerLevelEndReason(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason = __cordl_object
            .invoke("get_playerLevelEndReason", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_playerLevelEndState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState = __cordl_object
            .invoke("get_playerLevelEndState", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerLevelCompletionResults")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerLevelCompletionResults {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerLevelCompletionResults+MultiplayerPlayerLevelEndReason")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason {
    Cleared = 0i32,
    ConnectedAfterLevelEnded = 7i32,
    Failed = 1i32,
    GivenUp = 2i32,
    HostEndedLevel = 4i32,
    Quit = 3i32,
    StartupFailed = 6i32,
    WasInactive = 5i32,
}
#[cfg(feature = "MultiplayerLevelCompletionResults+MultiplayerPlayerLevelEndReason")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndReason
    => ""."MultiplayerLevelCompletionResults/MultiplayerPlayerLevelEndReason"
);
#[cfg(feature = "MultiplayerLevelCompletionResults+MultiplayerPlayerLevelEndState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState {
    NotFinished = 1i32,
    NotStarted = 2i32,
    SongFinished = 0i32,
}
#[cfg(feature = "MultiplayerLevelCompletionResults+MultiplayerPlayerLevelEndState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerLevelCompletionResults_MultiplayerPlayerLevelEndState
    => ""."MultiplayerLevelCompletionResults/MultiplayerPlayerLevelEndState"
);