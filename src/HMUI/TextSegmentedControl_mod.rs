#[cfg(feature = "HMUI+TextSegmentedControl")]
#[repr(C)]
#[derive(Debug)]
pub struct TextSegmentedControl {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::HMUI::SegmentedControl>,
    pub _fontSize: f32,
    pub _overrideCellSize: bool,
    pub _fixedCellSize: bool,
    pub _fixedCellSizeAmount: f32,
    pub _padding: f32,
    pub _hideCellBackground: bool,
    pub _firstCellPrefab: quest_hook::libil2cpp::Gc<
        crate::HMUI::TextSegmentedControlCell,
    >,
    pub _lastCellPrefab: quest_hook::libil2cpp::Gc<
        crate::HMUI::TextSegmentedControlCell,
    >,
    pub _singleCellPrefab: quest_hook::libil2cpp::Gc<
        crate::HMUI::TextSegmentedControlCell,
    >,
    pub _middleCellPrefab: quest_hook::libil2cpp::Gc<
        crate::HMUI::TextSegmentedControlCell,
    >,
    pub _texts: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >,
    pub _disabledIndexes: quest_hook::libil2cpp::Gc<i32>,
}
#[cfg(feature = "HMUI+TextSegmentedControl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::TextSegmentedControl => "HMUI"
    ."TextSegmentedControl"
);
#[cfg(feature = "HMUI+TextSegmentedControl")]
impl std::ops::Deref for crate::HMUI::TextSegmentedControl {
    type Target = quest_hook::libil2cpp::Gc<crate::HMUI::SegmentedControl>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+TextSegmentedControl")]
impl std::ops::DerefMut for crate::HMUI::TextSegmentedControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HMUI+TextSegmentedControl")]
impl crate::HMUI::TextSegmentedControl {
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
    pub fn SetTexts(
        &mut self,
        texts: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        disabledIndexes: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTexts", (texts, disabledIndexes))?;
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
#[cfg(feature = "HMUI+TextSegmentedControl")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::TextSegmentedControl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HMUI+TextSegmentedControl")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::HMUI::SegmentedControl_IDataSource>>
for crate::HMUI::TextSegmentedControl {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::HMUI::SegmentedControl_IDataSource> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "HMUI+TextSegmentedControl")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::HMUI::SegmentedControl_IDataSource>>
for crate::HMUI::TextSegmentedControl {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::HMUI::SegmentedControl_IDataSource> {
        unsafe { std::mem::transmute(self) }
    }
}
