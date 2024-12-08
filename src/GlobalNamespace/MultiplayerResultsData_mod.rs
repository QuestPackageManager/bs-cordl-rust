#[cfg(feature = "MultiplayerResultsData")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerResultsData {
    __cordl_parent: crate::System::Object,
    pub _gameId: *mut crate::System::String,
    pub _localPlayerResultData: *mut MultiplayerPlayerResultsData,
    pub _otherPlayersData: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut MultiplayerPlayerResultsData,
    >,
    pub _allPlayersSortedData: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut MultiplayerPlayerResultsData,
    >,
}
#[cfg(feature = "MultiplayerResultsData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerResultsData => ""."MultiplayerResultsData"
);
#[cfg(feature = "MultiplayerResultsData")]
impl std::ops::Deref for MultiplayerResultsData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerResultsData")]
impl std::ops::DerefMut for MultiplayerResultsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerResultsData")]
impl MultiplayerResultsData {
    pub fn get_gameId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_gameId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_localPlayerResultData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut MultiplayerPlayerResultsData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut MultiplayerPlayerResultsData = __cordl_object
            .invoke("get_localPlayerResultData", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        gameId: *mut crate::System::String,
        localPlayerResultData: *mut MultiplayerLevelCompletionResults,
        otherPlayersResultData: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut MultiplayerLevelCompletionResults,
        >,
        badgesProvider: *mut MultiplayerBadgesProvider,
        multiplayerSessionManager: *mut IMultiplayerSessionManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    gameId,
                    localPlayerResultData,
                    otherPlayersResultData,
                    badgesProvider,
                    multiplayerSessionManager,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_allPlayersSortedData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut MultiplayerPlayerResultsData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut MultiplayerPlayerResultsData,
        > = __cordl_object.invoke("get_allPlayersSortedData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_otherPlayersData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut MultiplayerPlayerResultsData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut MultiplayerPlayerResultsData,
        > = __cordl_object.invoke("get_otherPlayersData", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        gameId: *mut crate::System::String,
        localPlayerResultData: *mut MultiplayerLevelCompletionResults,
        otherPlayersResultData: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut MultiplayerLevelCompletionResults,
        >,
        badgesProvider: *mut MultiplayerBadgesProvider,
        multiplayerSessionManager: *mut IMultiplayerSessionManager,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    gameId,
                    localPlayerResultData,
                    otherPlayersResultData,
                    badgesProvider,
                    multiplayerSessionManager,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MultiplayerResultsData")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerResultsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
