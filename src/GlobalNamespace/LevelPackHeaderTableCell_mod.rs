#[cfg(feature = "LevelPackHeaderTableCell")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelPackHeaderTableCell {
    __cordl_parent: crate::HMUI::TableCell,
    pub _nameText: *mut crate::TMPro::TextMeshProUGUI,
    pub _backgroundImage: *mut crate::UnityEngine::UI::Image,
    pub _highlightBackgroundColor: crate::UnityEngine::Color,
    pub _selectedBackgroundColor: crate::UnityEngine::Color,
    pub _selectedAndHighlightedBackgroundColor: crate::UnityEngine::Color,
}
#[cfg(feature = "LevelPackHeaderTableCell")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LevelPackHeaderTableCell => ""
    ."LevelPackHeaderTableCell"
);
#[cfg(feature = "LevelPackHeaderTableCell")]
impl std::ops::Deref for LevelPackHeaderTableCell {
    type Target = crate::HMUI::TableCell;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelPackHeaderTableCell")]
impl std::ops::DerefMut for LevelPackHeaderTableCell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelPackHeaderTableCell")]
impl LevelPackHeaderTableCell {
    pub fn HighlightDidChange(
        &mut self,
        transitionType: crate::HMUI::SelectableCell_TransitionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HighlightDidChange", (transitionType))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RefreshVisuals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshVisuals", ())?;
        Ok(__cordl_ret)
    }
    pub fn SelectionDidChange(
        &mut self,
        transitionType: crate::HMUI::SelectableCell_TransitionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectionDidChange", (transitionType))?;
        Ok(__cordl_ret)
    }
    pub fn SetData(
        &mut self,
        headerText: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (headerText))?;
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
#[cfg(feature = "LevelPackHeaderTableCell")]
impl quest_hook::libil2cpp::ObjectType for LevelPackHeaderTableCell {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}