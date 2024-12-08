#[cfg(feature = "BeatmapDataMirrorTransform")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapDataMirrorTransform {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapDataMirrorTransform")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapDataMirrorTransform =>
    ""."BeatmapDataMirrorTransform"
);
#[cfg(feature = "BeatmapDataMirrorTransform")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapDataMirrorTransform {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataMirrorTransform")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapDataMirrorTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapDataMirrorTransform")]
impl crate::GlobalNamespace::BeatmapDataMirrorTransform {
    #[cfg(feature = "BeatmapDataMirrorTransform+__c__DisplayClass0_0")]
    pub type __c__DisplayClass0_0 = crate::GlobalNamespace::BeatmapDataMirrorTransform___c__DisplayClass0_0;
}
#[cfg(feature = "BeatmapDataMirrorTransform")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapDataMirrorTransform {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
