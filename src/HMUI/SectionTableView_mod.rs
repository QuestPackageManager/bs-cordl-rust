#[cfg(feature = "HMUI+SectionTableView+IDataSource")]
#[repr(C)]
#[derive(Debug)]
pub struct SectionTableView_IDataSource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HMUI+SectionTableView+IDataSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::SectionTableView_IDataSource => "HMUI"
    ."SectionTableView/IDataSource"
);
#[cfg(feature = "HMUI+SectionTableView+IDataSource")]
impl std::ops::Deref for crate::HMUI::SectionTableView_IDataSource {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+SectionTableView+IDataSource")]
impl std::ops::DerefMut for crate::HMUI::SectionTableView_IDataSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+SectionTableView+IDataSource")]
impl crate::HMUI::SectionTableView_IDataSource {
    pub fn CellForSectionHeader(
        &mut self,
        section: i32,
        unfolded: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::TableCell> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::TableCell = __cordl_object
            .invoke("CellForSectionHeader", (section, unfolded))?;
        Ok(__cordl_ret)
    }
    pub fn RowHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("RowHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn NumberOfSections(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("NumberOfSections", ())?;
        Ok(__cordl_ret)
    }
    pub fn CellForRowInSection(
        &mut self,
        section: i32,
        row: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::TableCell> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::TableCell = __cordl_object
            .invoke("CellForRowInSection", (section, row))?;
        Ok(__cordl_ret)
    }
    pub fn NumberOfRowsInSection(
        &mut self,
        section: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("NumberOfRowsInSection", (section))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "HMUI+SectionTableView+IDataSource")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::SectionTableView_IDataSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+SectionTableView+Section")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SectionTableView_Section {
    pub unfolded: bool,
    pub startBaseRow: i32,
    pub numberOfBaseRows: i32,
}
#[cfg(feature = "HMUI+SectionTableView+Section")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HMUI::SectionTableView_Section => "HMUI"
    ."SectionTableView/Section"
);
#[cfg(feature = "HMUI+SectionTableView+Section")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HMUI::SectionTableView_Section {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HMUI+SectionTableView+Section")]
impl crate::HMUI::SectionTableView_Section {}
#[cfg(feature = "HMUI+SectionTableView")]
#[repr(C)]
#[derive(Debug)]
pub struct SectionTableView {
    __cordl_parent: crate::HMUI::TableView,
    pub _unfoldSectionsByDefault: bool,
    pub didSelectRowInSectionEvent: *mut crate::System::Action_3<
        *mut crate::HMUI::SectionTableView,
        i32,
        i32,
    >,
    pub didSelectHeaderEvent: *mut crate::System::Action_2<
        *mut crate::HMUI::SectionTableView,
        i32,
    >,
    pub _dataSource: *mut crate::HMUI::SectionTableView_IDataSource,
    pub _sections: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::HMUI::SectionTableView_Section,
    >,
}
#[cfg(feature = "HMUI+SectionTableView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::SectionTableView => "HMUI"
    ."SectionTableView"
);
#[cfg(feature = "HMUI+SectionTableView")]
impl std::ops::Deref for crate::HMUI::SectionTableView {
    type Target = crate::HMUI::TableView;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+SectionTableView")]
impl std::ops::DerefMut for crate::HMUI::SectionTableView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+SectionTableView")]
impl crate::HMUI::SectionTableView {
    #[cfg(feature = "HMUI+SectionTableView+IDataSource")]
    type IDataSource = crate::HMUI::SectionTableView_IDataSource;
    #[cfg(feature = "HMUI+SectionTableView+Section")]
    pub type Section = crate::HMUI::SectionTableView_Section;
    pub fn add_didSelectRowInSectionEvent(
        &mut self,
        value: *mut crate::System::Action_3<*mut crate::HMUI::SectionTableView, i32, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectRowInSectionEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didSelectHeaderEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut crate::HMUI::SectionTableView, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectHeaderEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn CellForIdx(
        &mut self,
        tableView: *mut crate::HMUI::TableView,
        baseRow: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::TableCell> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::TableCell = __cordl_object
            .invoke("CellForIdx", (tableView, baseRow))?;
        Ok(__cordl_ret)
    }
    pub fn DidSelectCellWithIdx(
        &mut self,
        baseRow: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DidSelectCellWithIdx", (baseRow))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSelectHeaderEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut crate::HMUI::SectionTableView, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectHeaderEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn FoldAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FoldAll", ())?;
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
    pub fn ReloadData__cordl_bool1(
        &mut self,
        resetFoldState: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReloadData", (resetFoldState))?;
        Ok(__cordl_ret)
    }
    pub fn UnfoldAllSections(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnfoldAllSections", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_dataSource(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::SectionTableView_IDataSource> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::SectionTableView_IDataSource = __cordl_object
            .invoke("get_dataSource", ())?;
        Ok(__cordl_ret)
    }
    pub fn FoldSection(
        &mut self,
        section: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FoldSection", (section))?;
        Ok(__cordl_ret)
    }
    pub fn CellSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("CellSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn ScrollToRow(
        &mut self,
        section: i32,
        row: i32,
        scrollPositionType: crate::HMUI::TableView_ScrollPositionType,
        animated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ScrollToRow", (section, row, scrollPositionType, animated))?;
        Ok(__cordl_ret)
    }
    pub fn SectionAndRowForBaseRow(
        &mut self,
        baseRow: i32,
        section: quest_hook::libil2cpp::ByRefMut<i32>,
        row: quest_hook::libil2cpp::ByRefMut<i32>,
        isSectionHeader: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SectionAndRowForBaseRow",
                (baseRow, section, row, isSectionHeader),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UnfoldSection(
        &mut self,
        section: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnfoldSection", (section))?;
        Ok(__cordl_ret)
    }
    pub fn set_dataSource(
        &mut self,
        value: *mut crate::HMUI::SectionTableView_IDataSource,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_dataSource", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSelectRowInSectionEvent(
        &mut self,
        value: *mut crate::System::Action_3<*mut crate::HMUI::SectionTableView, i32, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectRowInSectionEvent", (value))?;
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
    pub fn NumberOfCells(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("NumberOfCells", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsSectionUnfolded(
        &mut self,
        section: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSectionUnfolded", (section))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "HMUI+SectionTableView")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::SectionTableView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
