#[cfg(feature = "BeatmapData")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _updateAllBeatmapDataOnInsert_k__BackingField: bool,
    pub _cuttableNotesCount_k__BackingField: i32,
    pub _obstaclesCount_k__BackingField: i32,
    pub _bombsCount_k__BackingField: i32,
    pub beatmapEventDataWasInsertedEvent: *mut crate::System::Action_2<
        *mut crate::GlobalNamespace::BeatmapEventData,
        *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut crate::GlobalNamespace::BeatmapDataItem,
        >,
    >,
    pub beatmapEventDataWillBeRemovedEvent: *mut crate::System::Action_2<
        *mut crate::GlobalNamespace::BeatmapEventData,
        *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut crate::GlobalNamespace::BeatmapDataItem,
        >,
    >,
    pub beatmapEventDataWasRemovedEvent: *mut crate::System::Action_1<
        *mut crate::GlobalNamespace::BeatmapEventData,
    >,
    pub _allBeatmapData: *mut crate::GlobalNamespace::ISortedList_1<
        *mut crate::GlobalNamespace::BeatmapDataItem,
    >,
    pub _allBeatmapDataItemToNodeMap: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::GlobalNamespace::BeatmapDataItem,
        *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut crate::GlobalNamespace::BeatmapDataItem,
        >,
    >,
    pub _beatmapDataItemsPerTypeAndId: *mut crate::GlobalNamespace::BeatmapDataSortedListForTypeAndIds_1<
        *mut crate::GlobalNamespace::BeatmapDataItem,
    >,
    pub _numberOfLines: i32,
    pub _specialBasicBeatmapEventKeywords: *mut crate::System::Collections::Generic::HashSet_1<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub _beatmapObjectsInTimeRowProcessor: *mut crate::GlobalNamespace::BeatmapObjectsInTimeRowProcessor,
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
    #[cfg(feature = "BeatmapData+__c__DisplayClass46_0_1")]
    pub type __c__DisplayClass46_0_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::BeatmapData___c__DisplayClass46_0_1<
        T,
    >;
    pub fn AddBeatmapObjectData(
        &mut self,
        beatmapObjectData: *mut crate::GlobalNamespace::BeatmapObjectData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddBeatmapObjectData", (beatmapObjectData))?;
        Ok(__cordl_ret)
    }
    pub fn AddBeatmapObjectDataInOrder(
        &mut self,
        beatmapObjectData: *mut crate::GlobalNamespace::BeatmapObjectData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddBeatmapObjectDataInOrder", (beatmapObjectData))?;
        Ok(__cordl_ret)
    }
    pub fn AddSpecialBasicBeatmapEventKeyword(
        &mut self,
        specialBasicBeatmapEventKeyword: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddSpecialBasicBeatmapEventKeyword",
                (specialBasicBeatmapEventKeyword),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetBeatmapDataItems<T>(
        &mut self,
        subtypeGroupIdentifier: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<T> = __cordl_object
            .invoke("GetBeatmapDataItems", (subtypeGroupIdentifier))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn GetBeatmapDataItemsMerged<T>(
        &mut self,
        subtypeGroupIdentifiers: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<T> = __cordl_object
            .invoke("GetBeatmapDataItemsMerged", (subtypeGroupIdentifiers))?;
        Ok(__cordl_ret)
    }
    pub fn GetBeatmapDataItemsMergedCount<T>(
        &mut self,
        subtypeGroupIdentifiers: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
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
        Ok(__cordl_ret)
    }
    pub fn GetCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::BeatmapData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::BeatmapData = __cordl_object
            .invoke("GetCopy", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetFilteredCopy(
        &mut self,
        processDataItem: *mut crate::System::Func_2<
            *mut crate::GlobalNamespace::BeatmapDataItem,
            *mut crate::GlobalNamespace::BeatmapDataItem,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::BeatmapData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::BeatmapData = __cordl_object
            .invoke("GetFilteredCopy", (processDataItem))?;
        Ok(__cordl_ret)
    }
    pub fn InsertBeatmapEventData(
        &mut self,
        beatmapEventData: *mut crate::GlobalNamespace::BeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InsertBeatmapEventData", (beatmapEventData))?;
        Ok(__cordl_ret)
    }
    pub fn InsertBeatmapEventDataInOrder(
        &mut self,
        beatmapEventData: *mut crate::GlobalNamespace::BeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InsertBeatmapEventDataInOrder", (beatmapEventData))?;
        Ok(__cordl_ret)
    }
    pub fn InsertToAllBeatmapData(
        &mut self,
        beatmapDataItem: *mut crate::GlobalNamespace::BeatmapDataItem,
        node: *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut crate::GlobalNamespace::BeatmapDataItem,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut crate::GlobalNamespace::BeatmapDataItem,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut crate::GlobalNamespace::BeatmapDataItem,
        > = __cordl_object.invoke("InsertToAllBeatmapData", (beatmapDataItem, node))?;
        Ok(__cordl_ret)
    }
    pub fn New(numberOfLines: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (numberOfLines))?;
        Ok(__cordl_object)
    }
    pub fn ProcessAndSortBeatmapData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessAndSortBeatmapData", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessRemainingData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessRemainingData", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveBeatmapEventData(
        &mut self,
        beatmapEventData: *mut crate::GlobalNamespace::BeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveBeatmapEventData", (beatmapEventData))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn add_beatmapEventDataWasInsertedEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::BeatmapEventData,
            *mut crate::System::Collections::Generic::LinkedListNode_1<
                *mut crate::GlobalNamespace::BeatmapDataItem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_beatmapEventDataWasInsertedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_beatmapEventDataWasRemovedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::BeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_beatmapEventDataWasRemovedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_beatmapEventDataWillBeRemovedEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::BeatmapEventData,
            *mut crate::System::Collections::Generic::LinkedListNode_1<
                *mut crate::GlobalNamespace::BeatmapDataItem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_beatmapEventDataWillBeRemovedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_allBeatmapDataItems(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::LinkedList_1<
            *mut crate::GlobalNamespace::BeatmapDataItem,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::LinkedList_1<
            *mut crate::GlobalNamespace::BeatmapDataItem,
        > = __cordl_object.invoke("get_allBeatmapDataItems", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_bombsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_bombsCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_cuttableNotesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_cuttableNotesCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_numberOfLines(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_numberOfLines", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_obstaclesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_obstaclesCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_spawnRotationEventsCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_spawnRotationEventsCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_specialBasicBeatmapEventKeywords(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_specialBasicBeatmapEventKeywords", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_updateAllBeatmapDataOnInsert(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_updateAllBeatmapDataOnInsert", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_beatmapEventDataWasInsertedEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::BeatmapEventData,
            *mut crate::System::Collections::Generic::LinkedListNode_1<
                *mut crate::GlobalNamespace::BeatmapDataItem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_beatmapEventDataWasInsertedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_beatmapEventDataWasRemovedEvent(
        &mut self,
        value: *mut crate::System::Action_1<
            *mut crate::GlobalNamespace::BeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_beatmapEventDataWasRemovedEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_beatmapEventDataWillBeRemovedEvent(
        &mut self,
        value: *mut crate::System::Action_2<
            *mut crate::GlobalNamespace::BeatmapEventData,
            *mut crate::System::Collections::Generic::LinkedListNode_1<
                *mut crate::GlobalNamespace::BeatmapDataItem,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_beatmapEventDataWillBeRemovedEvent", (value))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
#[cfg(feature = "BeatmapData+BeatmapDataBinaryHeapItem")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapData_BeatmapDataBinaryHeapItem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub node: *mut crate::System::Collections::Generic::LinkedListNode_1<
        *mut crate::GlobalNamespace::BeatmapDataItem,
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
        other: *mut crate::GlobalNamespace::BeatmapData_BeatmapDataBinaryHeapItem,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (other))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        node: *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut crate::GlobalNamespace::BeatmapDataItem,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (node))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        node: *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut crate::GlobalNamespace::BeatmapDataItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (node))?;
        Ok(__cordl_ret)
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
