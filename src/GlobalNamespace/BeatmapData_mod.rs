#[cfg(feature = "BeatmapData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _updateAllBeatmapDataOnInsert_k__BackingField: bool,
    pub _cuttableNotesCount_k__BackingField: i32,
    pub _obstaclesCount_k__BackingField: i32,
    pub _bombsCount_k__BackingField: i32,
    pub beatmapEventDataWasInsertedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::LinkedListNode_1<
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
                >,
            >,
        >,
    >,
    pub beatmapEventDataWillBeRemovedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::LinkedListNode_1<
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
                >,
            >,
        >,
    >,
    pub beatmapEventDataWasRemovedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
        >,
    >,
    pub _allBeatmapData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ISortedList_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
        >,
    >,
    pub _allBeatmapDataItemToNodeMap: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::LinkedListNode_1<
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
                >,
            >,
        >,
    >,
    pub _beatmapDataItemsPerTypeAndId: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapDataSortedListForTypeAndIds_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
        >,
    >,
    pub _numberOfLines: i32,
    pub _specialBasicBeatmapEventKeywords: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _beatmapObjectsInTimeRowProcessor: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor,
    >,
    pub _prevAddedBeatmapObjectDataTime: f32,
    pub _isCreatingFilteredCopy: bool,
}
#[cfg(feature = "BeatmapData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapData => ""."BeatmapData"
);
#[cfg(feature = "BeatmapData")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapData")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapData")]
impl crate::GlobalNamespace::BeatmapData {
    pub const kDefaultNumberOfLines: i32 = 4i32;
    #[cfg(feature = "BeatmapData+BeatmapDataBinaryHeapItem")]
    pub type BeatmapDataBinaryHeapItem = crate::GlobalNamespace::BeatmapData_BeatmapDataBinaryHeapItem;
    pub fn AddBeatmapObjectData(
        &mut self,
        beatmapObjectData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddBeatmapObjectData", (beatmapObjectData))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddBeatmapObjectDataInOrder(
        &mut self,
        beatmapObjectData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddBeatmapObjectDataInOrder", (beatmapObjectData))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddSpecialBasicBeatmapEventKeyword(
        &mut self,
        specialBasicBeatmapEventKeyword: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddSpecialBasicBeatmapEventKeyword",
                (specialBasicBeatmapEventKeyword),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapDataItems<T>(
        &mut self,
        subtypeGroupIdentifier: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IEnumerable_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        > = __cordl_object.invoke("GetBeatmapDataItems", (subtypeGroupIdentifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapDataItemsCount<T>(
        &mut self,
        subtypeGroupIdentifier: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetBeatmapDataItemsCount", (subtypeGroupIdentifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapDataItemsMerged<T>(
        &mut self,
        subtypeGroupIdentifiers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IEnumerable_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        > = __cordl_object
            .invoke("GetBeatmapDataItemsMerged", (subtypeGroupIdentifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBeatmapDataItemsMergedCount<T>(
        &mut self,
        subtypeGroupIdentifiers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetBeatmapDataItemsMergedCount", (subtypeGroupIdentifiers))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapData,
        > = __cordl_object.invoke("GetCopy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFilteredCopy(
        &mut self,
        processDataItem: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapData,
        > = __cordl_object.invoke("GetFilteredCopy", (processDataItem))?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertBeatmapEventData(
        &mut self,
        beatmapEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InsertBeatmapEventData", (beatmapEventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertBeatmapEventDataInOrder(
        &mut self,
        beatmapEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InsertBeatmapEventDataInOrder", (beatmapEventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertToAllBeatmapData(
        &mut self,
        beatmapDataItem: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapDataItem,
        >,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
            >,
        > = __cordl_object.invoke("InsertToAllBeatmapData", (beatmapDataItem, node))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsBasicEventSpecialKeywordEnabled(
        &mut self,
        keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsBasicEventSpecialKeywordEnabled", (keyword))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        numberOfLines: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (numberOfLines))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessAndSortBeatmapData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessAndSortBeatmapData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessRemainingData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessRemainingData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveBeatmapEventData(
        &mut self,
        beatmapEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveBeatmapEventData", (beatmapEventData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        numberOfLines: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (numberOfLines))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_beatmapEventDataWasInsertedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::LinkedListNode_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapDataItem,
                        >,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_beatmapEventDataWasInsertedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_beatmapEventDataWasRemovedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_beatmapEventDataWasRemovedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_beatmapEventDataWillBeRemovedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::LinkedListNode_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapDataItem,
                        >,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_beatmapEventDataWillBeRemovedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_allBeatmapDataItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
            >,
        > = __cordl_object.invoke("get_allBeatmapDataItems", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_areValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_areValid", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bombsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_bombsCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cuttableNotesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_cuttableNotesCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_numberOfLines(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_numberOfLines", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_obstaclesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_obstaclesCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_specialBasicBeatmapEventKeywords(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = __cordl_object.invoke("get_specialBasicBeatmapEventKeywords", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_updateAllBeatmapDataOnInsert(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_updateAllBeatmapDataOnInsert", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_beatmapEventDataWasInsertedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::LinkedListNode_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapDataItem,
                        >,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_beatmapEventDataWasInsertedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_beatmapEventDataWasRemovedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_beatmapEventDataWasRemovedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_beatmapEventDataWillBeRemovedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::LinkedListNode_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapDataItem,
                        >,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_beatmapEventDataWillBeRemovedEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bombsCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bombsCount", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_cuttableNotesCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cuttableNotesCount", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_obstaclesCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_obstaclesCount", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_updateAllBeatmapDataOnInsert(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_updateAllBeatmapDataOnInsert", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatmapData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapData")]
impl AsRef<crate::GlobalNamespace::IReadonlyBeatmapData>
for crate::GlobalNamespace::BeatmapData {
    fn as_ref(&self) -> &crate::GlobalNamespace::IReadonlyBeatmapData {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapData")]
impl AsMut<crate::GlobalNamespace::IReadonlyBeatmapData>
for crate::GlobalNamespace::BeatmapData {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IReadonlyBeatmapData {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapData+BeatmapDataBinaryHeapItem")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapData_BeatmapDataBinaryHeapItem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub node: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::LinkedListNode_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
        >,
    >,
}
#[cfg(feature = "BeatmapData+BeatmapDataBinaryHeapItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BeatmapData_BeatmapDataBinaryHeapItem => ""
    ."BeatmapData/BeatmapDataBinaryHeapItem"
);
#[cfg(feature = "BeatmapData+BeatmapDataBinaryHeapItem")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapData_BeatmapDataBinaryHeapItem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapData+BeatmapDataBinaryHeapItem")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BeatmapData_BeatmapDataBinaryHeapItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapData+BeatmapDataBinaryHeapItem")]
impl crate::GlobalNamespace::BeatmapData_BeatmapDataBinaryHeapItem {
    pub fn CompareTo(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapData_BeatmapDataBinaryHeapItem,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        node: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (node))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (node))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatmapData+BeatmapDataBinaryHeapItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapData_BeatmapDataBinaryHeapItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatmapData+BeatmapDataBinaryHeapItem")]
impl AsRef<
    crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapData_BeatmapDataBinaryHeapItem,
        >,
    >,
> for crate::GlobalNamespace::BeatmapData_BeatmapDataBinaryHeapItem {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapData_BeatmapDataBinaryHeapItem,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatmapData+BeatmapDataBinaryHeapItem")]
impl AsMut<
    crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapData_BeatmapDataBinaryHeapItem,
        >,
    >,
> for crate::GlobalNamespace::BeatmapData_BeatmapDataBinaryHeapItem {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapData_BeatmapDataBinaryHeapItem,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
