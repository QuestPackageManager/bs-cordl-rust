#[cfg(feature = "NetworkPlayersTableView+CellInfo+CellType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CellInfo_NetworkPlayersTableView_CellType {
    #[default]
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
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub _tableView: quest_hook::libil2cpp::Gc<crate::HMUI::TableView>,
    pub _playerCellPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NetworkPlayerTableCell,
    >,
    pub _optionsCellPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NetworkPlayerOptionsTableCell,
    >,
    pub _headerCellPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::LevelPackHeaderTableCell,
    >,
    pub _rowHeight: f32,
    pub _cellInfo: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::NetworkPlayersTableView_CellInfo,
        >,
    >,
    pub _selectedCellIndex: i32,
    pub _selectedPlayerID: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _selectedCellHasOptions: bool,
}
#[cfg(feature = "NetworkPlayersTableView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::NetworkPlayersTableView => ""
    ."NetworkPlayersTableView"
);
#[cfg(feature = "NetworkPlayersTableView")]
impl std::ops::Deref for crate::GlobalNamespace::NetworkPlayersTableView {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
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
        players: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
        >,
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddPlayers", (players, title))?;
        Ok(__cordl_ret.into())
    }
    pub fn CellForIdx(
        &mut self,
        tableView: quest_hook::libil2cpp::Gc<crate::HMUI::TableView>,
        row: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::TableCell>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::TableCell> = __cordl_object
            .invoke("CellForIdx", (tableView, row))?;
        Ok(__cordl_ret.into())
    }
    pub fn CellSize(&mut self, idx: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("CellSize", (idx))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleCellWasPressed(
        &mut self,
        tableView: quest_hook::libil2cpp::Gc<crate::HMUI::TableView>,
        tableCell: quest_hook::libil2cpp::Gc<crate::HMUI::TableCell>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleCellWasPressed", (tableView, tableCell))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasVisibleOptions(
        player: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasVisibleOptions", (player))?;
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
    pub fn SetParties(
        &mut self,
        partyPlayers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
        >,
        otherPlayers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
        >,
        myPartyTitle: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        otherPlayersTitle: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetParties",
                (partyPlayers, otherPlayers, myPartyTitle, otherPlayersTitle),
            )?;
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
#[cfg(feature = "NetworkPlayersTableView")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::HMUI::TableView_IDataSource>>
for crate::GlobalNamespace::NetworkPlayersTableView {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::HMUI::TableView_IDataSource> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NetworkPlayersTableView")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::HMUI::TableView_IDataSource>>
for crate::GlobalNamespace::NetworkPlayersTableView {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::HMUI::TableView_IDataSource> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NetworkPlayersTableView+CellInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct NetworkPlayersTableView_CellInfo {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _cordl_type: crate::GlobalNamespace::CellInfo_NetworkPlayersTableView_CellType,
    pub headerString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub player: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkPlayer>,
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
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
