#[cfg(feature = "AnnotatedBeatmapLevelCollectionsViewController")]
#[repr(C)]
#[derive(Debug)]
pub struct AnnotatedBeatmapLevelCollectionsViewController {
    __cordl_parent: crate::HMUI::ViewController,
    pub _annotatedBeatmapLevelCollectionsGridView: *mut AnnotatedBeatmapLevelCollectionsGridView,
    pub _loadingControl: *mut LoadingControl,
    pub _additionalContentModel: *mut IAdditionalContentModel,
    pub didOpenBeatmapLevelCollectionsEvent: *mut crate::System::Action,
    pub didCloseBeatmapLevelCollectionsEvent: *mut crate::System::Action,
    pub didSelectAnnotatedBeatmapLevelCollectionEvent: *mut crate::System::Action_1<
        *mut BeatmapLevelPack,
    >,
    pub _selectedItemIndex: i32,
    pub _annotatedBeatmapLevelCollections: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut BeatmapLevelPack,
    >,
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsViewController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for AnnotatedBeatmapLevelCollectionsViewController => ""
    ."AnnotatedBeatmapLevelCollectionsViewController"
);
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsViewController")]
impl std::ops::Deref for AnnotatedBeatmapLevelCollectionsViewController {
    type Target = crate::HMUI::ViewController;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsViewController")]
impl std::ops::DerefMut for AnnotatedBeatmapLevelCollectionsViewController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsViewController")]
impl AnnotatedBeatmapLevelCollectionsViewController {
    pub fn DidActivate(
        &mut self,
        firstActivation: bool,
        addedToHierarchy: bool,
        screenSystemEnabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DidActivate",
                (firstActivation, addedToHierarchy, screenSystemEnabling),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DidDeactivate(
        &mut self,
        removedFromHierarchy: bool,
        screenSystemDisabling: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DidDeactivate", (removedFromHierarchy, screenSystemDisabling))?;
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
    pub fn HandleAnnotatedBeatmapLevelCollectionsGridViewClose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAnnotatedBeatmapLevelCollectionsGridViewClose", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleAnnotatedBeatmapLevelCollectionsGridViewOpen(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleAnnotatedBeatmapLevelCollectionsGridViewOpen", ())?;
        Ok(__cordl_ret)
    }
    pub fn HandleDidSelectAnnotatedBeatmapLevelCollection(
        &mut self,
        beatmapLevelCollection: *mut BeatmapLevelPack,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "HandleDidSelectAnnotatedBeatmapLevelCollection",
                (beatmapLevelCollection),
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
    pub fn SetData(
        &mut self,
        annotatedBeatmapLevelCollections: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut BeatmapLevelPack,
        >,
        selectedItemIndex: i32,
        hideIfOneOrNoPacks: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetData",
                (annotatedBeatmapLevelCollections, selectedItemIndex, hideIfOneOrNoPacks),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ShowLoading(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowLoading", ())?;
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
    pub fn add_didCloseBeatmapLevelCollectionsEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didCloseBeatmapLevelCollectionsEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_didOpenBeatmapLevelCollectionsEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didOpenBeatmapLevelCollectionsEvent", (value))?;
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
    pub fn get_selectedAnnotatedBeatmapLevelPack(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut BeatmapLevelPack> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut BeatmapLevelPack = __cordl_object
            .invoke("get_selectedAnnotatedBeatmapLevelPack", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_selectedItemIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_selectedItemIndex", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_didCloseBeatmapLevelCollectionsEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didCloseBeatmapLevelCollectionsEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_didOpenBeatmapLevelCollectionsEvent(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didOpenBeatmapLevelCollectionsEvent", (value))?;
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
#[cfg(feature = "AnnotatedBeatmapLevelCollectionsViewController")]
impl quest_hook::libil2cpp::ObjectType
for AnnotatedBeatmapLevelCollectionsViewController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
