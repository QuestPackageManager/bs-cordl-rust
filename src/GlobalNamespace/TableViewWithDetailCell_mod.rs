#[cfg(feature = "TableViewWithDetailCell+IDataSource")]
#[repr(C)]
#[derive(Debug)]
pub struct TableViewWithDetailCell_IDataSource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "TableViewWithDetailCell+IDataSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::TableViewWithDetailCell_IDataSource => ""
    ."TableViewWithDetailCell/IDataSource"
);
#[cfg(feature = "TableViewWithDetailCell+IDataSource")]
impl std::ops::Deref for crate::GlobalNamespace::TableViewWithDetailCell_IDataSource {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TableViewWithDetailCell+IDataSource")]
impl std::ops::DerefMut for crate::GlobalNamespace::TableViewWithDetailCell_IDataSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TableViewWithDetailCell+IDataSource")]
impl crate::GlobalNamespace::TableViewWithDetailCell_IDataSource {
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
    pub fn NumberOfCells(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("NumberOfCells", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "TableViewWithDetailCell+IDataSource")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TableViewWithDetailCell_IDataSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TableViewWithDetailCell")]
#[repr(C)]
#[derive(Debug)]
pub struct TableViewWithDetailCell {
    __cordl_parent: crate::HMUI::TableView,
    pub didSelectContentCellEvent: *mut crate::System::Action_2<
        *mut TableViewWithDetailCell,
        i32,
    >,
    pub didDeselectContentCellEvent: *mut crate::System::Action_2<
        *mut TableViewWithDetailCell,
        i32,
    >,
    pub _dataSource: *mut crate::GlobalNamespace::TableViewWithDetailCell_IDataSource,
    pub _selectedId: i32,
}
#[cfg(feature = "TableViewWithDetailCell")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for TableViewWithDetailCell => ""."TableViewWithDetailCell"
);
#[cfg(feature = "TableViewWithDetailCell")]
impl std::ops::Deref for TableViewWithDetailCell {
    type Target = crate::HMUI::TableView;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TableViewWithDetailCell")]
impl std::ops::DerefMut for TableViewWithDetailCell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TableViewWithDetailCell")]
impl TableViewWithDetailCell {
    #[cfg(feature = "TableViewWithDetailCell+IDataSource")]
    type IDataSource = crate::GlobalNamespace::TableViewWithDetailCell_IDataSource;
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
    pub fn DidSelectCellWithIdx(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DidSelectCellWithIdx", (idx))?;
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
    pub fn ReloadData_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReloadData", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReloadData_i32_1(
        &mut self,
        currentNewIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReloadData", (currentNewIndex))?;
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
    pub fn add_didDeselectContentCellEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut TableViewWithDetailCell, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didDeselectContentCellEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didSelectContentCellEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut TableViewWithDetailCell, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectContentCellEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_dataSource(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::TableViewWithDetailCell_IDataSource,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::TableViewWithDetailCell_IDataSource = __cordl_object
            .invoke("get_dataSource", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didDeselectContentCellEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut TableViewWithDetailCell, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didDeselectContentCellEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSelectContentCellEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut TableViewWithDetailCell, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectContentCellEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_dataSource(
        &mut self,
        value: *mut crate::GlobalNamespace::TableViewWithDetailCell_IDataSource,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_dataSource", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "TableViewWithDetailCell")]
impl quest_hook::libil2cpp::ObjectType for TableViewWithDetailCell {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
