#[cfg(feature = "MultiplayerResultsData")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerResultsData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _gameId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _localPlayerResultData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MultiplayerPlayerResultsData,
    >,
    pub _otherPlayersData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IReadOnlyList_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MultiplayerPlayerResultsData,
            >,
        >,
    >,
    pub _allPlayersSortedData: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IReadOnlyList_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::MultiplayerPlayerResultsData,
            >,
        >,
    >,
}
#[cfg(feature = "MultiplayerResultsData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerResultsData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerResultsData";
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
        gameId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localPlayerResultData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
        otherPlayersResultData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerLevelCompletionResults,
                >,
            >,
        >,
        badgesProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerBadgesProvider,
        >,
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        gameId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        localPlayerResultData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerLevelCompletionResults,
        >,
        otherPlayersResultData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerLevelCompletionResults,
                >,
            >,
        >,
        badgesProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerBadgesProvider,
        >,
        multiplayerSessionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn get_allPlayersSortedData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerPlayerResultsData,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerPlayerResultsData,
                >,
            >,
        > = __cordl_object.invoke("get_allPlayersSortedData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gameId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_gameId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_localPlayerResultData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerPlayerResultsData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::MultiplayerPlayerResultsData,
        > = __cordl_object.invoke("get_localPlayerResultData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_otherPlayersData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerPlayerResultsData,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::MultiplayerPlayerResultsData,
                >,
            >,
        > = __cordl_object.invoke("get_otherPlayersData", ())?;
        Ok(__cordl_ret.into())
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
