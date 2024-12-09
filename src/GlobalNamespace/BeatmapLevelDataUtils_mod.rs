#[cfg(feature = "BeatmapLevelDataUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatmapLevelDataUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatmapLevelDataUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BeatmapLevelDataUtils => ""
    ."BeatmapLevelDataUtils"
);
#[cfg(feature = "BeatmapLevelDataUtils")]
impl std::ops::Deref for crate::GlobalNamespace::BeatmapLevelDataUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelDataUtils")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatmapLevelDataUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatmapLevelDataUtils")]
impl crate::GlobalNamespace::BeatmapLevelDataUtils {
    pub const gzipByte0: u8 = 31u8;
    pub const gzipByte1: u8 = 139u8;
    #[cfg(feature = "BeatmapLevelDataUtils+_ReadAllTextFromUrlAsync_d__8")]
    pub type _ReadAllTextFromUrlAsync_d__8 = crate::GlobalNamespace::BeatmapLevelDataUtils__ReadAllTextFromUrlAsync_d__8;
    #[cfg(feature = "BeatmapLevelDataUtils+__c__DisplayClass7_0")]
    pub type __c__DisplayClass7_0 = crate::GlobalNamespace::BeatmapLevelDataUtils___c__DisplayClass7_0;
    #[cfg(feature = "BeatmapLevelDataUtils+__c__DisplayClass8_0")]
    pub type __c__DisplayClass8_0 = crate::GlobalNamespace::BeatmapLevelDataUtils___c__DisplayClass8_0;
    #[cfg(feature = "BeatmapLevelDataUtils+__c__DisplayClass9_0")]
    pub type __c__DisplayClass9_0 = crate::GlobalNamespace::BeatmapLevelDataUtils___c__DisplayClass9_0;
}
#[cfg(feature = "BeatmapLevelDataUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatmapLevelDataUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
