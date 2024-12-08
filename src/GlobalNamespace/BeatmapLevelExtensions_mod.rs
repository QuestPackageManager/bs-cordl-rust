#[cfg(feature = "BeatmapLevelExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "BeatmapLevelExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for BeatmapLevelExtensions => ""."BeatmapLevelExtensions"
);
#[cfg(feature = "BeatmapLevelExtensions")]
impl std::ops::Deref for BeatmapLevelExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelExtensions")]
impl std::ops::DerefMut for BeatmapLevelExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelExtensions")]
impl BeatmapLevelExtensions {
    #[cfg(feature = "BeatmapLevelExtensions+__c")]
    pub type __c = crate::GlobalNamespace::BeatmapLevelExtensions___c;
}
#[cfg(feature = "BeatmapLevelExtensions")]
impl quest_hook::libil2cpp::ObjectType for BeatmapLevelExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
