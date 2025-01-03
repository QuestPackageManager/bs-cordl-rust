#[cfg(feature = "FileBrowserTableView")]
#[repr(C)]
#[derive(Debug)]
pub struct FileBrowserTableView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _tableView: quest_hook::libil2cpp::Gc<crate::HMUI::TableView>,
    pub _cellPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::FileBrowserTableCell,
    >,
    pub _cellHeight: f32,
    pub didSelectRow: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            *mut crate::GlobalNamespace::FileBrowserTableView,
            *mut crate::GlobalNamespace::FileBrowserItem,
        >,
    >,
    pub _items: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<*mut crate::GlobalNamespace::FileBrowserItem>,
    >,
}
#[cfg(feature = "FileBrowserTableView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::FileBrowserTableView => ""
    ."FileBrowserTableView"
);
#[cfg(feature = "FileBrowserTableView")]
impl std::ops::Deref for crate::GlobalNamespace::FileBrowserTableView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "FileBrowserTableView")]
impl std::ops::DerefMut for crate::GlobalNamespace::FileBrowserTableView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "FileBrowserTableView")]
impl crate::GlobalNamespace::FileBrowserTableView {
    pub const kCellIdentifier: &'static str = "Cell";
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
    pub fn ClearSelection(
        &mut self,
        animated: bool,
        scrollToRow0: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearSelection", (animated, scrollToRow0))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleDidSelectRowEvent(
        &mut self,
        tableView: quest_hook::libil2cpp::Gc<crate::HMUI::TableView>,
        row: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleDidSelectRowEvent", (tableView, row))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        items: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::FileBrowserItem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (items))?;
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
    pub fn SelectAndScrollRow(
        &mut self,
        row: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectAndScrollRow", (row))?;
        Ok(__cordl_ret.into())
    }
    pub fn SelectAndScrollRowToItemWithPath(
        &mut self,
        folderPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("SelectAndScrollRowToItemWithPath", (folderPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetItems(
        &mut self,
        items: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::GlobalNamespace::FileBrowserItem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetItems", (items))?;
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
    pub fn add_didSelectRow(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::FileBrowserTableView,
                *mut crate::GlobalNamespace::FileBrowserItem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectRow", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSelectRow(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::GlobalNamespace::FileBrowserTableView,
                *mut crate::GlobalNamespace::FileBrowserItem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectRow", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "FileBrowserTableView")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::FileBrowserTableView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "FileBrowserTableView")]
impl AsRef<crate::HMUI::TableView_IDataSource>
for crate::GlobalNamespace::FileBrowserTableView {
    fn as_ref(&self) -> &crate::HMUI::TableView_IDataSource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "FileBrowserTableView")]
impl AsMut<crate::HMUI::TableView_IDataSource>
for crate::GlobalNamespace::FileBrowserTableView {
    fn as_mut(&mut self) -> &mut crate::HMUI::TableView_IDataSource {
        unsafe { std::mem::transmute(self) }
    }
}
