#[cfg(feature = "SpawnRotationBeatmapEventDataProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct SpawnRotationBeatmapEventDataProcessor {
    __cordl_parent: BeatmapEventDataProcessor_1<*mut SpawnRotationBeatmapEventData>,
}
#[cfg(feature = "SpawnRotationBeatmapEventDataProcessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SpawnRotationBeatmapEventDataProcessor => ""
    ."SpawnRotationBeatmapEventDataProcessor"
);
#[cfg(feature = "SpawnRotationBeatmapEventDataProcessor")]
impl std::ops::Deref for SpawnRotationBeatmapEventDataProcessor {
    type Target = BeatmapEventDataProcessor_1<*mut SpawnRotationBeatmapEventData>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SpawnRotationBeatmapEventDataProcessor")]
impl std::ops::DerefMut for SpawnRotationBeatmapEventDataProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SpawnRotationBeatmapEventDataProcessor")]
impl SpawnRotationBeatmapEventDataProcessor {
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
    pub fn ProcessInsertedEventDataInternal(
        &mut self,
        insertedNode: *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut BeatmapDataItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessInsertedEventDataInternal", (insertedNode))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessBeforeDeleteEventDataInternal(
        &mut self,
        nodeToDelete: *mut crate::System::Collections::Generic::LinkedListNode_1<
            *mut BeatmapDataItem,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessBeforeDeleteEventDataInternal", (nodeToDelete))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "SpawnRotationBeatmapEventDataProcessor")]
impl quest_hook::libil2cpp::ObjectType for SpawnRotationBeatmapEventDataProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
