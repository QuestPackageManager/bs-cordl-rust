#[cfg(feature = "BeatmapEventDataBoxGroupList")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapEventDataBoxGroupList {
    __cordl_parent: crate::System::Object,
    pub updateBeatmapDataOnInsert: bool,
    pub _beatmapEventDataBoxGroupProcessor: *mut BeatmapEventDataBoxGroupProcessor,
    pub _sortedList: *mut SortedList_2<
        *mut BeatmapEventDataBoxGroup,
        *mut BeatmapEventDataBoxGroup,
    >,
    pub _groupId: i32,
    pub _beatmapData: *mut BeatmapData,
    pub _beatToTimeConverter: *mut IBeatToTimeConverter,
    pub _nonSyncedInsertsExist: bool,
}
#[cfg(feature = "BeatmapEventDataBoxGroupList")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapEventDataBoxGroupList => ""
    ."BeatmapEventDataBoxGroupList"
);
#[cfg(feature = "BeatmapEventDataBoxGroupList")]
impl std::ops::Deref for BeatmapEventDataBoxGroupList {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataBoxGroupList")]
impl std::ops::DerefMut for BeatmapEventDataBoxGroupList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapEventDataBoxGroupList")]
impl BeatmapEventDataBoxGroupList {
    pub fn _ctor(
        &mut self,
        groupId: i32,
        beatmapData: *mut BeatmapData,
        beatToTimeConverter: *mut IBeatToTimeConverter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (groupId, beatmapData, beatToTimeConverter))?;
        Ok(__cordl_ret)
    }
    pub fn Insert(
        &mut self,
        beatmapEventDataBoxGroup: *mut BeatmapEventDataBoxGroup,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut BeatmapEventDataBoxGroup,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut BeatmapEventDataBoxGroup,
        > = __cordl_object.invoke("Insert", (beatmapEventDataBoxGroup))?;
        Ok(__cordl_ret)
    }
    pub fn Remove(
        &mut self,
        nodeToDelete: *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut BeatmapEventDataBoxGroup,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (nodeToDelete))?;
        Ok(__cordl_ret)
    }
    pub fn SyncWithBeatmapData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SyncWithBeatmapData", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        groupId: i32,
        beatmapData: *mut BeatmapData,
        beatToTimeConverter: *mut IBeatToTimeConverter,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (groupId, beatmapData, beatToTimeConverter))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatmapEventDataBoxGroupList")]
impl quest_hook::libil2cpp::ObjectType for BeatmapEventDataBoxGroupList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
