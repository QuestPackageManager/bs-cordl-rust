#[cfg(feature = "NetworkPlayersTableView+CellInfo+CellType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellInfo_NetworkPlayersTableView_CellType {
    Header = 0i32,
    Options = 2i32,
    Player = 1i32,
}
#[cfg(feature = "NetworkPlayersTableView+CellInfo+CellType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::CellInfo_NetworkPlayersTableView_CellType => ""
    ."NetworkPlayersTableView/CellInfo/CellType"
);
#[cfg(feature = "NetworkPlayersTableView")]
#[repr(C)]
#[derive(Debug)]
pub struct NetworkPlayersTableView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _tableView: *mut crate::HMUI::TableView,
    pub _playerCellPrefab: *mut crate::GlobalNamespace::NetworkPlayerTableCell,
    pub _optionsCellPrefab: *mut crate::GlobalNamespace::NetworkPlayerOptionsTableCell,
    pub _headerCellPrefab: *mut crate::GlobalNamespace::LevelPackHeaderTableCell,
    pub _rowHeight: f32,
    pub _cellInfo: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::NetworkPlayersTableView_CellInfo,
    >,
    pub _selectedCellIndex: i32,
    pub _selectedPlayerID: *mut quest_hook::libil2cpp::Il2CppString,
    pub _selectedCellHasOptions: bool,
}
#[cfg(feature = "NetworkPlayersTableView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NetworkPlayersTableView => ""
    ."NetworkPlayersTableView"
);
#[cfg(feature = "NetworkPlayersTableView")]
impl std::ops::Deref for crate::GlobalNamespace::NetworkPlayersTableView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkPlayersTableView")]
impl std::ops::DerefMut for crate::GlobalNamespace::NetworkPlayersTableView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkPlayersTableView")]
impl crate::GlobalNamespace::NetworkPlayersTableView {
    pub const kHeaderCellIdentifier: &'static str = "HeaderCell";
    pub const kOptionsCellIdentifier: &'static str = "OptionsCell";
    pub const kPlayerCellIdentifier: &'static str = "PlayerCell";
    #[cfg(feature = "NetworkPlayersTableView+CellInfo")]
    pub type CellInfo = crate::GlobalNamespace::NetworkPlayersTableView_CellInfo;
    pub fn AddPlayers(
        &mut self,
        players: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::INetworkPlayer,
        >,
        title: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddPlayers", (players, title))?;
        Ok(__cordl_ret)
    }
    pub fn CellForIdx(
        &mut self,
        tableView: *mut crate::HMUI::TableView,
        row: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::TableCell> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::TableCell = __cordl_object
            .invoke("CellForIdx", (tableView, row))?;
        Ok(__cordl_ret)
    }
    pub fn CellSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("CellSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleCellWasPressed(
        &mut self,
        tableView: *mut crate::HMUI::TableView,
        tableCell: *mut crate::HMUI::TableCell,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleCellWasPressed", (tableView, tableCell))?;
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
    pub fn SetParties(
        &mut self,
        partyPlayers: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::INetworkPlayer,
        >,
        otherPlayers: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::INetworkPlayer,
        >,
        myPartyTitle: *mut quest_hook::libil2cpp::Il2CppString,
        otherPlayersTitle: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetParties",
                (partyPlayers, otherPlayers, myPartyTitle, otherPlayersTitle),
            )?;
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
}
#[cfg(feature = "NetworkPlayersTableView")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NetworkPlayersTableView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NetworkPlayersTableView+CellInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct NetworkPlayersTableView_CellInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cordl_type: crate::GlobalNamespace::CellInfo_NetworkPlayersTableView_CellType,
    pub headerString: *mut quest_hook::libil2cpp::Il2CppString,
    pub player: *mut crate::GlobalNamespace::INetworkPlayer,
    pub lastCellInParty: bool,
}
#[cfg(feature = "NetworkPlayersTableView+CellInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::NetworkPlayersTableView_CellInfo => ""
    ."NetworkPlayersTableView/CellInfo"
);
#[cfg(feature = "NetworkPlayersTableView+CellInfo")]
impl std::ops::Deref for crate::GlobalNamespace::NetworkPlayersTableView_CellInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkPlayersTableView+CellInfo")]
impl std::ops::DerefMut for crate::GlobalNamespace::NetworkPlayersTableView_CellInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NetworkPlayersTableView+CellInfo")]
impl crate::GlobalNamespace::NetworkPlayersTableView_CellInfo {
    #[cfg(feature = "NetworkPlayersTableView+CellInfo+CellType")]
    pub type CellType = crate::GlobalNamespace::CellInfo_NetworkPlayersTableView_CellType;
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
}
#[cfg(feature = "NetworkPlayersTableView+CellInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NetworkPlayersTableView_CellInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
