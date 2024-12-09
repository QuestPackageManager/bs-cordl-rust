#[cfg(feature = "MultiplayerResultsData")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerResultsData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _gameId: *mut quest_hook::libil2cpp::Il2CppString,
    pub _localPlayerResultData: *mut crate::GlobalNamespace::MultiplayerPlayerResultsData,
    pub _otherPlayersData: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut crate::GlobalNamespace::MultiplayerPlayerResultsData,
    >,
    pub _allPlayersSortedData: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut crate::GlobalNamespace::MultiplayerPlayerResultsData,
    >,
}
#[cfg(feature = "MultiplayerResultsData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerResultsData => ""
    ."MultiplayerResultsData"
);
#[cfg(feature = "MultiplayerResultsData")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerResultsData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerResultsData")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerResultsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerResultsData")]
impl crate::GlobalNamespace::MultiplayerResultsData {
    pub fn New(
        gameId: *mut quest_hook::libil2cpp::Il2CppString,
        localPlayerResultData: *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        otherPlayersResultData: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
        badgesProvider: *mut crate::GlobalNamespace::MultiplayerBadgesProvider,
        multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
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
    pub fn _ctor(
        &mut self,
        gameId: *mut quest_hook::libil2cpp::Il2CppString,
        localPlayerResultData: *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        otherPlayersResultData: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
        badgesProvider: *mut crate::GlobalNamespace::MultiplayerBadgesProvider,
        multiplayerSessionManager: *mut crate::GlobalNamespace::IMultiplayerSessionManager,
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
            *mut crate::GlobalNamespace::MultiplayerPlayerResultsData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::MultiplayerPlayerResultsData,
        > = __cordl_object.invoke("get_allPlayersSortedData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_gameId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_gameId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_localPlayerResultData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::MultiplayerPlayerResultsData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::MultiplayerPlayerResultsData = __cordl_object
            .invoke("get_localPlayerResultData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_otherPlayersData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::MultiplayerPlayerResultsData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::GlobalNamespace::MultiplayerPlayerResultsData,
        > = __cordl_object.invoke("get_otherPlayersData", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "MultiplayerResultsData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerResultsData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
