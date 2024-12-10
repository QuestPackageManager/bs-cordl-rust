#[cfg(feature = "GameServerPlayersTableView")]
#[repr(C)]
#[derive(Debug)]
pub struct GameServerPlayersTableView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _tableView: *mut crate::HMUI::TableView,
    pub _gameServerPlayerCellPrefab: *mut crate::GlobalNamespace::GameServerPlayerTableCell,
    pub _gameServerPlayerCellWithoutSongsPrefab: *mut crate::GlobalNamespace::GameServerPlayerTableCell,
    pub _gameServerPlayerCellWithoutModifiersPrefab: *mut crate::GlobalNamespace::GameServerPlayerTableCell,
    pub _tableHeaderSongGo: *mut crate::UnityEngine::GameObject,
    pub _tableHeaderModifiersGo: *mut crate::UnityEngine::GameObject,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _entitlementModel: *mut crate::GlobalNamespace::IEntitlementModel,
    pub selectSuggestedLevelEvent: *mut crate::System::Action_1<
        crate::GlobalNamespace::BeatmapKey,
    >,
    pub selectSuggestedGameplayModifiersEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::GameplayModifiers,
    >,
    pub kickPlayerEvent: *mut crate::System::Action_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _initialized: bool,
    pub _hasKickPermissions: bool,
    pub _allowSelection: bool,
    pub _showSongSelection: bool,
    pub _showModifierSelection: bool,
    pub _selectedPlayer: *mut crate::GlobalNamespace::IConnectedPlayer,
    pub _sortedConnectedPlayers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::IConnectedPlayer,
    >,
    pub _lobbyPlayersDataModel: *mut crate::GlobalNamespace::ILobbyPlayersDataModel,
}
#[cfg(feature = "GameServerPlayersTableView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameServerPlayersTableView =>
    ""."GameServerPlayersTableView"
);
#[cfg(feature = "GameServerPlayersTableView")]
impl std::ops::Deref for crate::GlobalNamespace::GameServerPlayersTableView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerPlayersTableView")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameServerPlayersTableView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerPlayersTableView")]
impl crate::GlobalNamespace::GameServerPlayersTableView {
    pub const kCellId: &'static str = "Cell";
    pub const kNoModifiersCellId: &'static str = "NoModifierCell";
    pub const kNoSongsCellId: &'static str = "NoSongCell";
    pub fn CellForIdx(
        &mut self,
        tableView: quest_hook::libil2cpp::Gc<crate::HMUI::TableView>,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::TableCell>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::TableCell> = __cordl_object
            .invoke("CellForIdx", (tableView, idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn CellSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("CellSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GameServerPlayerTableCell>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::GameServerPlayerTableCell,
        > = __cordl_object.invoke("GetCurrentPrefab", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleCellKickPlayer(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleCellKickPlayer", (idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleCellUseBeatmap(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleCellUseBeatmap", (idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleCellUseModifiers(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleCellUseModifiers", (idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn NumberOfCells(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("NumberOfCells", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetData(
        &mut self,
        sortedPlayers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::GlobalNamespace::IConnectedPlayer,
            >,
        >,
        lobbyPlayersDataModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ILobbyPlayersDataModel,
        >,
        hasKickPermissions: bool,
        allowSelection: bool,
        showSongSelection: bool,
        showModifierSelection: bool,
        clearSelection: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetData",
                (
                    sortedPlayers,
                    lobbyPlayersDataModel,
                    hasKickPermissions,
                    allowSelection,
                    showSongSelection,
                    showModifierSelection,
                    clearSelection,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetLobbyPlayerData(
        &mut self,
        idx: i32,
        player: quest_hook::libil2cpp::ByRefMut<
            *mut crate::GlobalNamespace::IConnectedPlayer,
        >,
        playerData: quest_hook::libil2cpp::ByRefMut<
            *mut crate::GlobalNamespace::ILobbyPlayerData,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetLobbyPlayerData", (idx, player, playerData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn add_kickPlayerEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_kickPlayerEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_selectSuggestedGameplayModifiersEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::GameplayModifiers>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_selectSuggestedGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_selectSuggestedLevelEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::BeatmapKey>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_selectSuggestedLevelEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentCellId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_currentCellId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_kickPlayerEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_kickPlayerEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_selectSuggestedGameplayModifiersEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::GameplayModifiers>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_selectSuggestedGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_selectSuggestedLevelEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::BeatmapKey>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_selectSuggestedLevelEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameServerPlayersTableView")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameServerPlayersTableView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
