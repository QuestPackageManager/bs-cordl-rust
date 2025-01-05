#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridView")]
#[repr(C)]
#[derive(Debug)]
pub struct AnnotatedBeatmapLevelCollectionsGridView {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _gridView: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GridView>,
    pub _pageControl: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PageControl>,
    pub _animator: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionsGridViewAnimator,
    >,
    pub _cellPrefab: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionCell,
    >,
    pub _cellWidth: f32,
    pub _cellHeight: f32,
    pub _contentWarningLabel: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub _additionalContentModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IAdditionalContentModel,
    >,
    pub _entitlementModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IEntitlementModel,
    >,
    pub _beatmapLevelsPromoModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapLevelsPromoModel,
    >,
    pub _vrPlatformHelper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IVRPlatformHelper,
    >,
    pub _playerDataModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PlayerDataModel,
    >,
    pub didOpenAnnotatedBeatmapLevelCollectionEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
    pub didCloseAnnotatedBeatmapLevelCollectionEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action,
    >,
    pub didSelectAnnotatedBeatmapLevelCollectionEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        >,
    >,
    pub _isInitialized: bool,
    pub _isHovering: bool,
    pub _annotatedBeatmapLevelCollections: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IReadOnlyList_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
        >,
    >,
    pub _selectedCellIndex: i32,
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::AnnotatedBeatmapLevelCollectionsGridView => ""
    ."AnnotatedBeatmapLevelCollectionsGridView"
);
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridView")]
impl std::ops::Deref
for crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionsGridView {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridView")]
impl std::ops::DerefMut
for crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionsGridView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridView")]
impl crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionsGridView {
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
    pub fn CellForIdx(
        &mut self,
        gridView: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::GridView>,
        idx: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour> = __cordl_object
            .invoke("CellForIdx", (gridView, idx))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn HandleAdditionalContentModelDidInvalidateData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAdditionalContentModelDidInvalidateData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleCellSelectionDidChange(
        &mut self,
        selectableCell: quest_hook::libil2cpp::Gc<crate::HMUI::SelectableCell>,
        transition: crate::HMUI::SelectableCell_TransitionType,
        changeOwner: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleCellSelectionDidChange",
                (selectableCell, transition, changeOwner),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleVRPlatformHelperInputFocusCaptured(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleVRPlatformHelperInputFocusCaptured", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Hide(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Hide", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerEnter(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerEnter", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPointerExit(
        &mut self,
        eventData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPointerExit", (eventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn RefreshAvailability(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshAvailability", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn SetData(
        &mut self,
        annotatedBeatmapLevelCollections: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetData", (annotatedBeatmapLevelCollections))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldExpandCollection(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ShouldExpandCollection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Show(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Show", ())?;
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
    pub fn add_didCloseAnnotatedBeatmapLevelCollectionEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didCloseAnnotatedBeatmapLevelCollectionEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didOpenAnnotatedBeatmapLevelCollectionEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didOpenAnnotatedBeatmapLevelCollectionEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_didSelectAnnotatedBeatmapLevelCollectionEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didSelectAnnotatedBeatmapLevelCollectionEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cellHeight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_cellHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cellWidth(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_cellWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_numberOfCells(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_numberOfCells", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didCloseAnnotatedBeatmapLevelCollectionEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didCloseAnnotatedBeatmapLevelCollectionEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didOpenAnnotatedBeatmapLevelCollectionEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didOpenAnnotatedBeatmapLevelCollectionEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didSelectAnnotatedBeatmapLevelCollectionEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevelPack>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didSelectAnnotatedBeatmapLevelCollectionEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridView")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionsGridView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridView")]
impl AsRef<crate::GlobalNamespace::GridView_IDataSource>
for crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionsGridView {
    fn as_ref(&self) -> &crate::GlobalNamespace::GridView_IDataSource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridView")]
impl AsMut<crate::GlobalNamespace::GridView_IDataSource>
for crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionsGridView {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::GridView_IDataSource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridView")]
impl AsRef<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionsGridView {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridView")]
impl AsMut<crate::UnityEngine::EventSystems::IEventSystemHandler>
for crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionsGridView {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IEventSystemHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridView")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerEnterHandler>
for crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionsGridView {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerEnterHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridView")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerEnterHandler>
for crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionsGridView {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerEnterHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridView")]
impl AsRef<crate::UnityEngine::EventSystems::IPointerExitHandler>
for crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionsGridView {
    fn as_ref(&self) -> &crate::UnityEngine::EventSystems::IPointerExitHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsGridView")]
impl AsMut<crate::UnityEngine::EventSystems::IPointerExitHandler>
for crate::GlobalNamespace::AnnotatedBeatmapLevelCollectionsGridView {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::EventSystems::IPointerExitHandler {
        unsafe { std::mem::transmute(self) }
    }
}
