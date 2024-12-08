#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapDataLoader")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataLoader {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapDataLoader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatmapDataLoaderVersion4::BeatmapDataLoader =>
    "BeatmapDataLoaderVersion4"."BeatmapDataLoader"
);
#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapDataLoader")]
impl std::ops::Deref for crate::BeatmapDataLoaderVersion4::BeatmapDataLoader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapDataLoader")]
impl std::ops::DerefMut for crate::BeatmapDataLoaderVersion4::BeatmapDataLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapDataLoader")]
impl crate::BeatmapDataLoaderVersion4::BeatmapDataLoader {
    #[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapDataLoader+__c__DisplayClass0_0")]
    pub type __c__DisplayClass0_0 = crate::BeatmapDataLoaderVersion4::BeatmapDataLoader___c__DisplayClass0_0;
    #[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapDataLoader+__c__DisplayClass1_0")]
    pub type __c__DisplayClass1_0 = crate::BeatmapDataLoaderVersion4::BeatmapDataLoader___c__DisplayClass1_0;
}
#[cfg(feature = "BeatmapDataLoaderVersion4+BeatmapDataLoader")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatmapDataLoaderVersion4::BeatmapDataLoader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
