#[cfg(feature = "IReadonlyBeatmapData")]
#[repr(C)]
#[derive(Debug)]
pub struct IReadonlyBeatmapData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IReadonlyBeatmapData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IReadonlyBeatmapData => ""
    ."IReadonlyBeatmapData"
);
#[cfg(feature = "IReadonlyBeatmapData")]
impl std::ops::Deref for crate::GlobalNamespace::IReadonlyBeatmapData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IReadonlyBeatmapData")]
impl std::ops::DerefMut for crate::GlobalNamespace::IReadonlyBeatmapData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IReadonlyBeatmapData")]
impl crate::GlobalNamespace::IReadonlyBeatmapData {
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
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
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
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_specialBasicBeatmapEventKeywords", ())?;
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
}
#[cfg(feature = "IReadonlyBeatmapData")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IReadonlyBeatmapData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
