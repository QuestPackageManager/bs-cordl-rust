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
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::BeatmapData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapData";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapObjectData>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddBeatmapObjectData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddBeatmapObjectData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (beatmapObjectData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddBeatmapObjectDataInOrder(
        &mut self,
        beatmapObjectData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapObjectData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapObjectData>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddBeatmapObjectDataInOrder")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddBeatmapObjectDataInOrder", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (beatmapObjectData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddSpecialBasicBeatmapEventKeyword(
        &mut self,
        specialBasicBeatmapEventKeyword: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddSpecialBasicBeatmapEventKeyword")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddSpecialBasicBeatmapEventKeyword", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (specialBasicBeatmapEventKeyword))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<T>,
                >,
                1usize,
            >("GetBeatmapDataItems")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetBeatmapDataItems", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        > = unsafe { method.invoke_unchecked(self, (subtypeGroupIdentifier)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), i32, 1usize>("GetBeatmapDataItemsCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetBeatmapDataItemsCount", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (subtypeGroupIdentifier))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<T>,
                >,
                1usize,
            >("GetBeatmapDataItemsMerged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetBeatmapDataItemsMerged", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        > = unsafe { method.invoke_unchecked(self, (subtypeGroupIdentifiers)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>),
                i32,
                1usize,
            >("GetBeatmapDataItemsMergedCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetBeatmapDataItemsMergedCount", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (subtypeGroupIdentifiers))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCopy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
                0usize,
            >("GetCopy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetCopy", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapData,
        > = unsafe { method.invoke_unchecked(self, ()) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Func_2<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapDataItem,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapDataItem,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapData>,
                1usize,
            >("GetFilteredCopy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetFilteredCopy", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapData,
        > = unsafe { method.invoke_unchecked(self, (processDataItem)) };
        Ok(__cordl_ret.into())
    }
    pub fn InsertBeatmapEventData(
        &mut self,
        beatmapEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InsertBeatmapEventData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InsertBeatmapEventData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (beatmapEventData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn InsertBeatmapEventDataInOrder(
        &mut self,
        beatmapEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("InsertBeatmapEventDataInOrder")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InsertBeatmapEventDataInOrder", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (beatmapEventData))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::LinkedListNode_1<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatmapDataItem,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::LinkedListNode_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapDataItem,
                        >,
                    >,
                >,
                2usize,
            >("InsertToAllBeatmapData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "InsertToAllBeatmapData", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
            >,
        > = unsafe { method.invoke_unchecked(self, (beatmapDataItem, node)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsBasicEventSpecialKeywordEnabled(
        &mut self,
        keyword: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                bool,
                1usize,
            >("IsBasicEventSpecialKeywordEnabled")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsBasicEventSpecialKeywordEnabled", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (keyword)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ProcessAndSortBeatmapData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessAndSortBeatmapData", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ProcessRemainingData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ProcessRemainingData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ProcessRemainingData", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn RemoveBeatmapEventData(
        &mut self,
        beatmapEventData: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatmapEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapEventData>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RemoveBeatmapEventData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RemoveBeatmapEventData", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (beatmapEventData))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        numberOfLines: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (numberOfLines))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapEventData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::LinkedListNode_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapDataItem,
                                >,
                            >,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_beatmapEventDataWasInsertedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_beatmapEventDataWasInsertedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapEventData,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_beatmapEventDataWasRemovedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_beatmapEventDataWasRemovedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapEventData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::LinkedListNode_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapDataItem,
                                >,
                            >,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_beatmapEventDataWillBeRemovedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_beatmapEventDataWillBeRemovedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::LinkedList_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapDataItem,
                        >,
                    >,
                >,
                0usize,
            >("get_allBeatmapDataItems")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_allBeatmapDataItems", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedList_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapDataItem>,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_areValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_areValid")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_areValid", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_bombsCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_bombsCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_bombsCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_cuttableNotesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_cuttableNotesCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_cuttableNotesCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_numberOfLines(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_numberOfLines")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_numberOfLines", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_obstaclesCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_obstaclesCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_obstaclesCount", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
                0usize,
            >("get_specialBasicBeatmapEventKeywords")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_specialBasicBeatmapEventKeywords", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_updateAllBeatmapDataOnInsert(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_updateAllBeatmapDataOnInsert")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_updateAllBeatmapDataOnInsert", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapEventData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::LinkedListNode_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapDataItem,
                                >,
                            >,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_beatmapEventDataWasInsertedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_beatmapEventDataWasInsertedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapEventData,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_beatmapEventDataWasRemovedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_beatmapEventDataWasRemovedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapEventData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::LinkedListNode_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::BeatmapDataItem,
                                >,
                            >,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_beatmapEventDataWillBeRemovedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_beatmapEventDataWillBeRemovedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_bombsCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_bombsCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_bombsCount", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_cuttableNotesCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_cuttableNotesCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_cuttableNotesCount", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_obstaclesCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_obstaclesCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_obstaclesCount", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_updateAllBeatmapDataOnInsert(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_updateAllBeatmapDataOnInsert")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_updateAllBeatmapDataOnInsert", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatmapData_BeatmapDataBinaryHeapItem {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatmapData/BeatmapDataBinaryHeapItem";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatmapData_BeatmapDataBinaryHeapItem,
                >),
                i32,
                1usize,
            >("CompareTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompareTo", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (other)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::LinkedListNode_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatmapDataItem,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (node))
        };
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
