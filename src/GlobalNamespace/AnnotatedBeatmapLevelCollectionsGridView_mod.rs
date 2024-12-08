#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridView")]
#[repr(C)]
#[derive(Debug)]
pub struct AnnotatedBeatmapLevelCollectionsGridView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _gridView: *mut GridView,
    pub _pageControl: *mut PageControl,
    pub _animator: *mut AnnotatedBeatmapLevelCollectionsGridViewAnimator,
    pub _cellPrefab: *mut AnnotatedBeatmapLevelCollectionCell,
    pub _cellWidth: f32,
    pub _cellHeight: f32,
    pub _contentWarningLabel: *mut crate::UnityEngine::GameObject,
    pub _additionalContentModel: *mut IAdditionalContentModel,
    pub _entitlementModel: *mut IEntitlementModel,
    pub _beatmapLevelsPromoModel: *mut BeatmapLevelsPromoModel,
    pub _vrPlatformHelper: *mut IVRPlatformHelper,
    pub _playerDataModel: *mut PlayerDataModel,
    pub didOpenAnnotatedBeatmapLevelCollectionEvent: *mut crate::System::Action,
    pub didCloseAnnotatedBeatmapLevelCollectionEvent: *mut crate::System::Action,
    pub didSelectAnnotatedBeatmapLevelCollectionEvent: *mut crate::System::Action_1<
        *mut BeatmapLevelPack,
    >,
    pub _isInitialized: bool,
    pub _isHovering: bool,
    pub _annotatedBeatmapLevelCollections: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut BeatmapLevelPack,
    >,
    pub _selectedCellIndex: i32,
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for AnnotatedBeatmapLevelCollectionsGridView => ""
    ."AnnotatedBeatmapLevelCollectionsGridView"
);
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridView")]
impl std::ops::Deref for AnnotatedBeatmapLevelCollectionsGridView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridView")]
impl std::ops::DerefMut for AnnotatedBeatmapLevelCollectionsGridView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridView")]
impl AnnotatedBeatmapLevelCollectionsGridView {
    pub fn CancelAsyncOperations(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CancelAsyncOperations", ())?;
        Ok(__cordl_ret)
    }
    pub fn CellForIdx(
        &mut self,
        gridView: *mut GridView,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::MonoBehaviour> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::MonoBehaviour = __cordl_object
            .invoke("CellForIdx", (gridView, idx))?;
        Ok(__cordl_ret)
    }
    pub fn CloseLevelCollection(
        &mut self,
        animated: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloseLevelCollection", (animated))?;
        Ok(__cordl_ret)
    }
    pub fn GetCellHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetCellHeight", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetCellWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetCellWidth", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNumberOfCells(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetNumberOfCells", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleAdditionalContentModelDidInvalidateData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAdditionalContentModelDidInvalidateData", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleCellSelectionDidChange(
        &mut self,
        selectableCell: *mut crate::HMUI::SelectableCell,
        transition: crate::HMUI::SelectableCell_TransitionType,
        changeOwner: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleCellSelectionDidChange",
                (selectableCell, transition, changeOwner),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HandleVRPlatformHelperInputFocusCaptured(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleVRPlatformHelperInputFocusCaptured", ())?;
        Ok(__cordl_ret)
    }
    pub fn Hide(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Hide", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerEnter(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerEnter", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn OnPointerExit(
        &mut self,
        eventData: *mut crate::UnityEngine::EventSystems::PointerEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerExit", (eventData))?;
        Ok(__cordl_ret)
    }
    pub fn RefreshAvailability(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshAvailability", ())?;
        Ok(__cordl_ret)
    }
    pub fn SelectAndScrollToCellWithIdx(
        &mut self,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SelectAndScrollToCellWithIdx", (idx))?;
        Ok(__cordl_ret)
    }
    pub fn SetData(
        &mut self,
        annotatedBeatmapLevelCollections: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut BeatmapLevelPack,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (annotatedBeatmapLevelCollections))?;
        Ok(__cordl_ret)
    }
    pub fn ShouldExpandCollection(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldExpandCollection", ())?;
        Ok(__cordl_ret)
    }
    pub fn Show(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Show", ())?;
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
    pub fn add_didCloseAnnotatedBeatmapLevelCollectionEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didCloseAnnotatedBeatmapLevelCollectionEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didOpenAnnotatedBeatmapLevelCollectionEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didOpenAnnotatedBeatmapLevelCollectionEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didSelectAnnotatedBeatmapLevelCollectionEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut BeatmapLevelPack>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectAnnotatedBeatmapLevelCollectionEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didCloseAnnotatedBeatmapLevelCollectionEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didCloseAnnotatedBeatmapLevelCollectionEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didOpenAnnotatedBeatmapLevelCollectionEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didOpenAnnotatedBeatmapLevelCollectionEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didSelectAnnotatedBeatmapLevelCollectionEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut BeatmapLevelPack>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectAnnotatedBeatmapLevelCollectionEvent", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridView")]
impl quest_hook::libil2cpp::ObjectType for AnnotatedBeatmapLevelCollectionsGridView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
