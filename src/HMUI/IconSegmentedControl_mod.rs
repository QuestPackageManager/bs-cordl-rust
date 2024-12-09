#[cfg(feature = "HMUI+IconSegmentedControl")]
#[repr(C)]
#[derive(Debug)]
pub struct IconSegmentedControl {
    __cordl_parent: crate::HMUI::SegmentedControl,
    pub _iconSize: f32,
    pub _overrideCellSize: bool,
    pub _padding: f32,
    pub _hideCellBackground: bool,
    pub _firstCellPrefab: *mut crate::HMUI::IconSegmentedControlCell,
    pub _lastCellPrefab: *mut crate::HMUI::IconSegmentedControlCell,
    pub _middleCellPrefab: *mut crate::HMUI::IconSegmentedControlCell,
    pub _singleCellPrefab: *mut crate::HMUI::IconSegmentedControlCell,
    pub _dataItems: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::HMUI::IconSegmentedControl_DataItem,
    >,
    pub _isInitialized: bool,
}
#[cfg(feature = "HMUI+IconSegmentedControl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::IconSegmentedControl => "HMUI"
    ."IconSegmentedControl"
);
#[cfg(feature = "HMUI+IconSegmentedControl")]
impl std::ops::Deref for crate::HMUI::IconSegmentedControl {
    type Target = crate::HMUI::SegmentedControl;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+IconSegmentedControl")]
impl std::ops::DerefMut for crate::HMUI::IconSegmentedControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+IconSegmentedControl")]
impl crate::HMUI::IconSegmentedControl {
    #[cfg(feature = "HMUI+IconSegmentedControl+DataItem")]
    pub type DataItem = crate::HMUI::IconSegmentedControl_DataItem;
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
        dataItems: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::HMUI::IconSegmentedControl_DataItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (dataItems))?;
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
#[cfg(feature = "HMUI+IconSegmentedControl")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::IconSegmentedControl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+IconSegmentedControl+DataItem")]
#[repr(C)]
#[derive(Debug)]
pub struct IconSegmentedControl_DataItem {
    __cordl_parent: crate::System::Object,
    pub _icon_k__BackingField: *mut crate::UnityEngine::Sprite,
    pub _hintText_k__BackingField: *mut crate::System::String,
}
#[cfg(feature = "HMUI+IconSegmentedControl+DataItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::IconSegmentedControl_DataItem => "HMUI"
    ."IconSegmentedControl/DataItem"
);
#[cfg(feature = "HMUI+IconSegmentedControl+DataItem")]
impl std::ops::Deref for crate::HMUI::IconSegmentedControl_DataItem {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+IconSegmentedControl+DataItem")]
impl std::ops::DerefMut for crate::HMUI::IconSegmentedControl_DataItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+IconSegmentedControl+DataItem")]
impl crate::HMUI::IconSegmentedControl_DataItem {
    pub fn New(
        icon: *mut crate::UnityEngine::Sprite,
        hintText: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (icon, hintText))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        icon: *mut crate::UnityEngine::Sprite,
        hintText: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (icon, hintText))?;
        Ok(__cordl_ret)
    }
    pub fn get_hintText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_hintText", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_icon(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Sprite> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Sprite = __cordl_object
            .invoke("get_icon", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_hintText(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hintText", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_icon(
        &mut self,
        value: *mut crate::UnityEngine::Sprite,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_icon", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HMUI+IconSegmentedControl+DataItem")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::IconSegmentedControl_DataItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
