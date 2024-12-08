#[cfg(feature = "HMUI+SegmentedControl+IDataSource")]
#[repr(C)]
#[derive(Debug)]
pub struct SegmentedControl_IDataSource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HMUI+SegmentedControl+IDataSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::SegmentedControl_IDataSource => "HMUI"
    ."SegmentedControl/IDataSource"
);
#[cfg(feature = "HMUI+SegmentedControl+IDataSource")]
impl std::ops::Deref for crate::HMUI::SegmentedControl_IDataSource {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+SegmentedControl+IDataSource")]
impl std::ops::DerefMut for crate::HMUI::SegmentedControl_IDataSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+SegmentedControl+IDataSource")]
impl crate::HMUI::SegmentedControl_IDataSource {
    pub fn CellForCellNumber(
        &mut self,
        cellNumber: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::SegmentedControlCell> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::SegmentedControlCell = __cordl_object
            .invoke("CellForCellNumber", (cellNumber))?;
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
#[cfg(feature = "HMUI+SegmentedControl+IDataSource")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::SegmentedControl_IDataSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+SegmentedControl")]
#[repr(C)]
#[derive(Debug)]
pub struct SegmentedControl {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _separatorPrefab: *mut crate::UnityEngine::Transform,
    pub _container: *mut crate::Zenject::DiContainer,
    pub didSelectCellEvent: *mut crate::System::Action_2<
        *mut crate::HMUI::SegmentedControl,
        i32,
    >,
    pub _numberOfCells: i32,
    pub _cells: *mut crate::System::Collections::Generic::List_1<
        *mut crate::HMUI::SegmentedControlCell,
    >,
    pub _separators: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::GameObject,
    >,
    pub _dataSource: *mut crate::HMUI::SegmentedControl_IDataSource,
    pub _selectedCellNumber: i32,
    pub _callbacks: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::System::Action_1<i32>,
    >,
    pub _reusableCellPools: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::UnityEngine::Object,
        *mut crate::System::Collections::Generic::Queue_1<
            *mut crate::HMUI::SegmentedControlCell,
        >,
    >,
    pub _cellToPrefabMap: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::HMUI::SegmentedControlCell,
        *mut crate::UnityEngine::Object,
    >,
}
#[cfg(feature = "HMUI+SegmentedControl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::SegmentedControl => "HMUI"
    ."SegmentedControl"
);
#[cfg(feature = "HMUI+SegmentedControl")]
impl std::ops::Deref for crate::HMUI::SegmentedControl {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+SegmentedControl")]
impl std::ops::DerefMut for crate::HMUI::SegmentedControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+SegmentedControl")]
impl crate::HMUI::SegmentedControl {
    #[cfg(feature = "HMUI+SegmentedControl+IDataSource")]
    type IDataSource = crate::HMUI::SegmentedControl_IDataSource;
    pub fn CreateCells(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateCells", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetReusableCell<T>(
        &mut self,
        prefab: *mut crate::UnityEngine::Object,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("GetReusableCell", (prefab))?;
        Ok(__cordl_ret)
    }
    pub fn HandleCellSelectionDidChange(
        &mut self,
        selectableCell: *mut crate::HMUI::SelectableCell,
        transitionType: crate::HMUI::SelectableCell_TransitionType,
        changeOwner: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleCellSelectionDidChange",
                (selectableCell, transitionType, changeOwner),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ReloadData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReloadData", ())?;
        Ok(__cordl_ret)
    }
    pub fn SelectCellWithNumber(
        &mut self,
        selectCellNumber: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectCellWithNumber", (selectCellNumber))?;
        Ok(__cordl_ret)
    }
    pub fn SetCallbackForCell(
        &mut self,
        cellNumber: i32,
        callback: *mut crate::System::Action_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCallbackForCell", (cellNumber, callback))?;
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
    pub fn add_didSelectCellEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut crate::HMUI::SegmentedControl, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectCellEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_cells(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::HMUI::SegmentedControlCell,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::HMUI::SegmentedControlCell,
        > = __cordl_object.invoke("get_cells", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_dataSource(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HMUI::SegmentedControl_IDataSource> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HMUI::SegmentedControl_IDataSource = __cordl_object
            .invoke("get_dataSource", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedCellNumber(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_selectedCellNumber", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSelectCellEvent(
        &mut self,
        value: *mut crate::System::Action_2<*mut crate::HMUI::SegmentedControl, i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectCellEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_dataSource(
        &mut self,
        value: *mut crate::HMUI::SegmentedControl_IDataSource,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_dataSource", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HMUI+SegmentedControl")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::SegmentedControl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}