#[cfg(feature = "HMUI+IconSegmentedControl")]
#[repr(C)]
#[derive(Debug)]
pub struct IconSegmentedControl {
    __cordl_parent: crate::HMUI::SegmentedControl,
    pub _iconSize: f32,
    pub _overrideCellSize: bool,
    pub _padding: f32,
    pub _hideCellBackground: bool,
    pub _firstCellPrefab: quest_hook::libil2cpp::Gc<
        crate::HMUI::IconSegmentedControlCell,
    >,
    pub _lastCellPrefab: quest_hook::libil2cpp::Gc<
        crate::HMUI::IconSegmentedControlCell,
    >,
    pub _middleCellPrefab: quest_hook::libil2cpp::Gc<
        crate::HMUI::IconSegmentedControlCell,
    >,
    pub _singleCellPrefab: quest_hook::libil2cpp::Gc<
        crate::HMUI::IconSegmentedControlCell,
    >,
    pub _dataItems: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::HMUI::IconSegmentedControl_DataItem>,
        >,
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::HMUI::SegmentedControlCell>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::HMUI::SegmentedControlCell> = __cordl_object
            .invoke("CellForCellNumber", (cellNumber))?;
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
    pub fn SetData(
        &mut self,
        dataItems: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::HMUI::IconSegmentedControl_DataItem>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (dataItems))?;
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
#[cfg(feature = "HMUI+IconSegmentedControl")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::IconSegmentedControl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+IconSegmentedControl")]
impl AsRef<crate::HMUI::SegmentedControl_IDataSource>
for crate::HMUI::IconSegmentedControl {
    fn as_ref(&self) -> &crate::HMUI::SegmentedControl_IDataSource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+IconSegmentedControl")]
impl AsMut<crate::HMUI::SegmentedControl_IDataSource>
for crate::HMUI::IconSegmentedControl {
    fn as_mut(&mut self) -> &mut crate::HMUI::SegmentedControl_IDataSource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+IconSegmentedControl+DataItem")]
#[repr(C)]
#[derive(Debug)]
pub struct IconSegmentedControl_DataItem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _icon_k__BackingField: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub _hintText_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _interactable_k__BackingField: bool,
}
#[cfg(feature = "HMUI+IconSegmentedControl+DataItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::IconSegmentedControl_DataItem => "HMUI"
    ."IconSegmentedControl/DataItem"
);
#[cfg(feature = "HMUI+IconSegmentedControl+DataItem")]
impl std::ops::Deref for crate::HMUI::IconSegmentedControl_DataItem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        icon: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        hintText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        interactable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (icon, hintText, interactable))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        icon: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        hintText: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        interactable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (icon, hintText, interactable))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hintText(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_hintText", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_icon(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite> = __cordl_object
            .invoke("get_icon", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_interactable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_interactable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_hintText(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hintText", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_icon(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_icon", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_interactable(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_interactable", (value))?;
        Ok(__cordl_ret.into())
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
