#[cfg(feature = "MockBeatmapDataConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct MockBeatmapDataConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "MockBeatmapDataConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MockBeatmapDataConverter => ""
    ."MockBeatmapDataConverter"
);
#[cfg(feature = "MockBeatmapDataConverter")]
impl std::ops::Deref for crate::GlobalNamespace::MockBeatmapDataConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockBeatmapDataConverter")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockBeatmapDataConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockBeatmapDataConverter")]
impl crate::GlobalNamespace::MockBeatmapDataConverter {
    #[cfg(feature = "MockBeatmapDataConverter+__c")]
    pub type __c = crate::GlobalNamespace::MockBeatmapDataConverter___c;
}
#[cfg(feature = "MockBeatmapDataConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MockBeatmapDataConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
