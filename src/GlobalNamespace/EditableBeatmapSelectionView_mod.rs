#[cfg(feature = "EditableBeatmapSelectionView")]
#[repr(C)]
#[derive(Debug)]
pub struct EditableBeatmapSelectionView {
    __cordl_parent: crate::GlobalNamespace::BeatmapSelectionView,
    pub _editButton: *mut crate::UnityEngine::UI::Button,
    pub _clearButton: *mut crate::UnityEngine::UI::Button,
    pub _levelBarCanvasGroup: *mut crate::UnityEngine::CanvasGroup,
    pub _interactable: bool,
    pub _showClearButton_k__BackingField: bool,
}
#[cfg(feature = "EditableBeatmapSelectionView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EditableBeatmapSelectionView =>
    ""."EditableBeatmapSelectionView"
);
#[cfg(feature = "EditableBeatmapSelectionView")]
impl std::ops::Deref for crate::GlobalNamespace::EditableBeatmapSelectionView {
    type Target = crate::GlobalNamespace::BeatmapSelectionView;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EditableBeatmapSelectionView")]
impl std::ops::DerefMut for crate::GlobalNamespace::EditableBeatmapSelectionView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EditableBeatmapSelectionView")]
impl crate::GlobalNamespace::EditableBeatmapSelectionView {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetBeatmap(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBeatmap", (beatmapKey))?;
        Ok(__cordl_ret)
    }
    pub fn SetVisibility(
        &mut self,
        visible: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVisibility", (visible))?;
        Ok(__cordl_ret)
    }
    pub fn Setup(
        &mut self,
        showClearButton: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Setup", (showClearButton))?;
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
    pub fn get_clearButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Button> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Button = __cordl_object
            .invoke("get_clearButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_editButton(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::Button> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::Button = __cordl_object
            .invoke("get_editButton", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_interactable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_interactable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_showClearButton(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_showClearButton", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn set_showClearButton(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_showClearButton", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "EditableBeatmapSelectionView")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EditableBeatmapSelectionView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
