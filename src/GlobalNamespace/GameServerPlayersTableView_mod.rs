#[cfg(feature = "GameServerPlayersTableView")]
#[repr(C)]
#[derive(Debug)]
pub struct GameServerPlayersTableView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _tableView: *mut crate::HMUI::TableView,
    pub _gameServerPlayerCellPrefab: *mut GameServerPlayerTableCell,
    pub _gameServerPlayerCellWithoutSongsPrefab: *mut GameServerPlayerTableCell,
    pub _gameServerPlayerCellWithoutModifiersPrefab: *mut GameServerPlayerTableCell,
    pub _tableHeaderSongGo: *mut crate::UnityEngine::GameObject,
    pub _tableHeaderModifiersGo: *mut crate::UnityEngine::GameObject,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _entitlementModel: *mut IEntitlementModel,
    pub selectSuggestedLevelEvent: *mut crate::System::Action_1<BeatmapKey>,
    pub selectSuggestedGameplayModifiersEvent: *mut crate::System::Action_1<
        *mut GameplayModifiers,
    >,
    pub kickPlayerEvent: *mut crate::System::Action_1<*mut crate::System::String>,
    pub _initialized: bool,
    pub _hasKickPermissions: bool,
    pub _allowSelection: bool,
    pub _showSongSelection: bool,
    pub _showModifierSelection: bool,
    pub _selectedPlayer: *mut IConnectedPlayer,
    pub _sortedConnectedPlayers: *mut crate::System::Collections::Generic::List_1<
        *mut IConnectedPlayer,
    >,
    pub _lobbyPlayersDataModel: *mut ILobbyPlayersDataModel,
}
#[cfg(feature = "GameServerPlayersTableView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for GameServerPlayersTableView => ""
    ."GameServerPlayersTableView"
);
#[cfg(feature = "GameServerPlayersTableView")]
impl std::ops::Deref for GameServerPlayersTableView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerPlayersTableView")]
impl std::ops::DerefMut for GameServerPlayersTableView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameServerPlayersTableView")]
impl GameServerPlayersTableView {
    pub const kCellId: &'static str = "Cell";
    pub const kNoModifiersCellId: &'static str = "NoModifierCell";
    pub const kNoSongsCellId: &'static str = "NoSongCell";
    pub fn CellForIdx(
        &mut self,
        tableView: *mut crate::HMUI::TableView,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::TableCell> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::TableCell = __cordl_object
            .invoke("CellForIdx", (tableView, idx))?;
        Ok(__cordl_ret)
    }
    pub fn CellSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("CellSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCurrentPrefab(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut GameServerPlayerTableCell> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut GameServerPlayerTableCell = __cordl_object
            .invoke("GetCurrentPrefab", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn NumberOfCells(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("NumberOfCells", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetData(
        &mut self,
        sortedPlayers: *mut crate::System::Collections::Generic::List_1<
            *mut IConnectedPlayer,
        >,
        lobbyPlayersDataModel: *mut ILobbyPlayersDataModel,
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
        Ok(__cordl_ret)
    }
    pub fn TryGetLobbyPlayerData(
        &mut self,
        idx: i32,
        player: quest_hook::libil2cpp::ByRefMut<*mut IConnectedPlayer>,
        playerData: quest_hook::libil2cpp::ByRefMut<*mut ILobbyPlayerData>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetLobbyPlayerData", (idx, player, playerData))?;
        Ok(__cordl_ret)
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
    pub fn add_kickPlayerEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_kickPlayerEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_selectSuggestedGameplayModifiersEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut GameplayModifiers>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_selectSuggestedGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_selectSuggestedLevelEvent(
        &mut self,
        value: *mut crate::System::Action_1<BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_selectSuggestedLevelEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_currentCellId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_currentCellId", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_kickPlayerEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_kickPlayerEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_selectSuggestedGameplayModifiersEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut GameplayModifiers>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_selectSuggestedGameplayModifiersEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_selectSuggestedLevelEvent(
        &mut self,
        value: *mut crate::System::Action_1<BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_selectSuggestedLevelEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "GameServerPlayersTableView")]
impl quest_hook::libil2cpp::ObjectType for GameServerPlayersTableView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
