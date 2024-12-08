#[cfg(feature = "LightColorBeatmapEventDataProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct LightColorBeatmapEventDataProcessor {
    __cordl_parent: BeatmapEventDataProcessor_1<*mut LightColorBeatmapEventData>,
}
#[cfg(feature = "LightColorBeatmapEventDataProcessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LightColorBeatmapEventDataProcessor => ""
    ."LightColorBeatmapEventDataProcessor"
);
#[cfg(feature = "LightColorBeatmapEventDataProcessor")]
impl std::ops::Deref for LightColorBeatmapEventDataProcessor {
    type Target = BeatmapEventDataProcessor_1<*mut LightColorBeatmapEventData>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightColorBeatmapEventDataProcessor")]
impl std::ops::DerefMut for LightColorBeatmapEventDataProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightColorBeatmapEventDataProcessor")]
impl LightColorBeatmapEventDataProcessor {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "LightColorBeatmapEventDataProcessor")]
impl quest_hook::libil2cpp::ObjectType for LightColorBeatmapEventDataProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
