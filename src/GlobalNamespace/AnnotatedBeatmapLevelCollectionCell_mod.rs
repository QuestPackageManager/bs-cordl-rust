#[cfg(feature = "AnnotatedBeatmapLevelCollectionCell")]
#[repr(C)]
#[derive(Debug)]
pub struct AnnotatedBeatmapLevelCollectionCell {
    __cordl_parent: crate::HMUI::SelectableCell,
    pub _infoText: quest_hook::libil2cpp::Gc<crate::TMPro::TextMeshProUGUI>,
    pub _coverImage: quest_hook::libil2cpp::Gc<crate::HMUI::ImageView>,
    pub _selectionImage: quest_hook::libil2cpp::Gc<crate::HMUI::ImageView>,
    pub _downloadIconObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _newBadgeObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _updatedBadgeObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _selectedColor0: crate::UnityEngine::Color,
    pub _selectedColor1: crate::UnityEngine::Color,
    pub _highlightedColor0: crate::UnityEngine::Color,
    pub _highlightedColor1: crate::UnityEngine::Color,
    pub _defaultCoverSprite: quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
    pub _cellIndex_k__BackingField: i32,
    pub _beatmapLevelPack: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelPack,
    >,
    pub _cancellationTokenSource: quest_hook::libil2cpp::Gc<
        crate::System::Threading::CancellationTokenSource,
    >,
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionCell")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::AnnotatedBeatmapLevelCollectionCell => ""
    ."AnnotatedBeatmapLevelCollectionCell"
);
#[cfg(feature = "AnnotatedBeatmapLevelCollectionCell")]
impl std::ops::Deref for crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionCell {
    type Target = crate::HMUI::SelectableCell;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionCell")]
impl std::ops::DerefMut for crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionCell {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionCell")]
impl crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionCell {
    pub fn CancelAsyncOperations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAsyncOperations", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInfoText(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        songs: i32,
        purchased: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetInfoText", (name, songs, purchased))?;
        Ok(__cordl_ret.into())
    }
    pub fn HighlightDidChange(
        &mut self,
        transitionType: crate::HMUI::SelectableCell_TransitionType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HighlightDidChange", (transitionType))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalToggle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalToggle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RefreshAvailabilityAsync(
        &mut self,
        entitlementModel: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IEntitlementModel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshAvailabilityAsync", (entitlementModel))?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshVisuals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshVisuals", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SetData(
        &mut self,
        beatmapLevelPack: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapLevelPack,
        >,
        isPromoted: bool,
        isUpdated: bool,
        interactable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (beatmapLevelPack, isPromoted, isUpdated, interactable))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDownloadIconVisible(
        &mut self,
        visible: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDownloadIconVisible", (visible))?;
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
    pub fn get_cellIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_cellIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_cellIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cellIndex", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionCell")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionCell {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
