#[cfg(feature = "HMUI+TextSegmentedControl")]
#[repr(C)]
#[derive(Debug)]
pub struct TextSegmentedControl {
    __cordl_parent: crate::HMUI::SegmentedControl,
    pub _fontSize: f32,
    pub _overrideCellSize: bool,
    pub _padding: f32,
    pub _hideCellBackground: bool,
    pub _firstCellPrefab: *mut crate::HMUI::TextSegmentedControlCell,
    pub _lastCellPrefab: *mut crate::HMUI::TextSegmentedControlCell,
    pub _singleCellPrefab: *mut crate::HMUI::TextSegmentedControlCell,
    pub _middleCellPrefab: *mut crate::HMUI::TextSegmentedControlCell,
    pub _texts: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "HMUI+TextSegmentedControl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HMUI::TextSegmentedControl => "HMUI"
    ."TextSegmentedControl"
);
#[cfg(feature = "HMUI+TextSegmentedControl")]
impl std::ops::Deref for crate::HMUI::TextSegmentedControl {
    type Target = crate::HMUI::SegmentedControl;
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
    pub fn SetTexts(
        &mut self,
        texts: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTexts", (texts))?;
        Ok(__cordl_ret)
    }
    pub fn NumberOfCells(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("NumberOfCells", ())?;
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
#[cfg(feature = "HMUI+TextSegmentedControl")]
impl quest_hook::libil2cpp::ObjectType for crate::HMUI::TextSegmentedControl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
