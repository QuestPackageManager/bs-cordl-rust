#[cfg(feature = "GameServersListTableView")]
#[repr(C)]
#[derive(Debug)]
pub struct GameServersListTableView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _tableView: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::TableViewWithDetailCell,
    >,
    pub _gameServerListCellPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameServerListTableCell,
    >,
    pub _gameServerDetailCellPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameServerListDetailTableCell,
    >,
    pub _container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub joinButtonPressedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
        >,
    >,
    pub _isInitialized: bool,
    pub _gamesList: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::INetworkPlayer>,
    >,
    pub _selectedServer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::INetworkPlayer,
    >,
}
#[cfg(feature = "GameServersListTableView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameServersListTableView => ""
    ."GameServersListTableView"
);
#[cfg(feature = "GameServersListTableView")]
impl std::ops::Deref for crate::GlobalNamespace::GameServersListTableView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameServersListTableView")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameServersListTableView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameServersListTableView")]
impl crate::GlobalNamespace::GameServersListTableView {
    pub const kCellReuseIdentifier: &'static str = "Cell";
    pub const kDetailCellReuseIdentifier: &'static str = "DetailCell";
    pub fn CellForContent(
        &mut self,
        tableView: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TableViewWithDetailCell,
        >,
        idx: i32,
        detailOpened: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::TableCell>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::TableCell> = __cordl_object
            .invoke("CellForContent", (tableView, idx, detailOpened))?;
        Ok(__cordl_ret.into())
    }
    pub fn CellForDetail(
        &mut self,
        tableView: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::TableViewWithDetailCell,
        >,
        contentIdx: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::TableCell>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::TableCell> = __cordl_object
            .invoke("CellForDetail", (tableView, contentIdx))?;
        Ok(__cordl_ret.into())
    }
    pub fn CellSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("CellSize", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleTableViewDidDeselectCellWithIdx(
        &mut self,
        arg1: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::TableViewWithDetailCell>,
        arg2: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleTableViewDidDeselectCellWithIdx", (arg1, arg2))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleTableViewDidSelectCellWithIdx(
        &mut self,
        tableView: quest_hook::libil2cpp::Gc<crate::HMUI::TableView>,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleTableViewDidSelectCellWithIdx", (tableView, id))?;
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
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetData(
        &mut self,
        servers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
            >,
        >,
        clearSelection: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (servers, clearSelection))?;
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
    pub fn add_joinButtonPressedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_joinButtonPressedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_joinButtonPressedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_joinButtonPressedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GameServersListTableView")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameServersListTableView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GameServersListTableView")]
impl AsRef<crate::GlobalNamespace::TableViewWithDetailCell_IDataSource>
for crate::GlobalNamespace::GameServersListTableView {
    fn as_ref(&self) -> &crate::GlobalNamespace::TableViewWithDetailCell_IDataSource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "GameServersListTableView")]
impl AsMut<crate::GlobalNamespace::TableViewWithDetailCell_IDataSource>
for crate::GlobalNamespace::GameServersListTableView {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::TableViewWithDetailCell_IDataSource {
        unsafe { std::mem::transmute(self) }
    }
}
