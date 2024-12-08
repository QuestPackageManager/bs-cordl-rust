#[cfg(feature = "GameServersListTableView")]
#[repr(C)]
#[derive(Debug)]
pub struct GameServersListTableView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _tableView: *mut TableViewWithDetailCell,
    pub _gameServerListCellPrefab: *mut GameServerListTableCell,
    pub _gameServerDetailCellPrefab: *mut GameServerListDetailTableCell,
    pub _container: *mut crate::Zenject::DiContainer,
    pub joinButtonPressedEvent: *mut crate::System::Action_1<*mut INetworkPlayer>,
    pub _isInitialized: bool,
    pub _gamesList: *mut quest_hook::libil2cpp::Il2CppArray<*mut INetworkPlayer>,
    pub _selectedServer: *mut INetworkPlayer,
}
#[cfg(feature = "GameServersListTableView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for GameServersListTableView => ""
    ."GameServersListTableView"
);
#[cfg(feature = "GameServersListTableView")]
impl std::ops::Deref for GameServersListTableView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameServersListTableView")]
impl std::ops::DerefMut for GameServersListTableView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameServersListTableView")]
impl GameServersListTableView {
    pub const kCellReuseIdentifier: &'static str = "Cell";
    pub const kDetailCellReuseIdentifier: &'static str = "DetailCell";
    pub fn CellForContent(
        &mut self,
        tableView: *mut TableViewWithDetailCell,
        idx: i32,
        detailOpened: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::TableCell> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::TableCell = __cordl_object
            .invoke("CellForContent", (tableView, idx, detailOpened))?;
        Ok(__cordl_ret)
    }
    pub fn CellForDetail(
        &mut self,
        tableView: *mut TableViewWithDetailCell,
        contentIdx: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::TableCell> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::TableCell = __cordl_object
            .invoke("CellForDetail", (tableView, contentIdx))?;
        Ok(__cordl_ret)
    }
    pub fn CellSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("CellSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleGameServerListDetailTableCellJoinServerButtonWasPressed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleGameServerListDetailTableCellJoinServerButtonWasPressed",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleTableViewDidDeselectCellWithIdx(
        &mut self,
        arg1: *mut TableViewWithDetailCell,
        arg2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleTableViewDidDeselectCellWithIdx", (arg1, arg2))?;
        Ok(__cordl_ret)
    }
    pub fn HandleTableViewDidSelectCellWithIdx(
        &mut self,
        tableView: *mut crate::HMUI::TableView,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleTableViewDidSelectCellWithIdx", (tableView, id))?;
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
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetData(
        &mut self,
        servers: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut INetworkPlayer,
        >,
        clearSelection: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (servers, clearSelection))?;
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
    pub fn add_joinButtonPressedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut INetworkPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_joinButtonPressedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_joinButtonPressedEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut INetworkPlayer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_joinButtonPressedEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "GameServersListTableView")]
impl quest_hook::libil2cpp::ObjectType for GameServersListTableView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
