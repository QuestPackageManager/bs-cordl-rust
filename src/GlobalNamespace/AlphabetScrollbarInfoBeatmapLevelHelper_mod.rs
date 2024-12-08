#[cfg(feature = "AlphabetScrollbarInfoBeatmapLevelHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct AlphabetScrollbarInfoBeatmapLevelHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "AlphabetScrollbarInfoBeatmapLevelHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for AlphabetScrollbarInfoBeatmapLevelHelper => ""
    ."AlphabetScrollbarInfoBeatmapLevelHelper"
);
#[cfg(feature = "AlphabetScrollbarInfoBeatmapLevelHelper")]
impl std::ops::Deref for AlphabetScrollbarInfoBeatmapLevelHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AlphabetScrollbarInfoBeatmapLevelHelper")]
impl std::ops::DerefMut for AlphabetScrollbarInfoBeatmapLevelHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AlphabetScrollbarInfoBeatmapLevelHelper")]
impl AlphabetScrollbarInfoBeatmapLevelHelper {
    pub const kFirstAlphabet: &'static str = "A";
    pub const kMaxCharactersCount: i32 = 28i32;
    pub const kNonAlphabetChar: char = "#";
    #[cfg(feature = "AlphabetScrollbarInfoBeatmapLevelHelper+__c")]
    pub type __c = crate::GlobalNamespace::AlphabetScrollbarInfoBeatmapLevelHelper___c;
}
#[cfg(feature = "AlphabetScrollbarInfoBeatmapLevelHelper")]
impl quest_hook::libil2cpp::ObjectType for AlphabetScrollbarInfoBeatmapLevelHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
