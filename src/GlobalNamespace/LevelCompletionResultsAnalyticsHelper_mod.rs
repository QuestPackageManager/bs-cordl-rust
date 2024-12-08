#[cfg(feature = "LevelCompletionResultsAnalyticsHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelCompletionResultsAnalyticsHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "LevelCompletionResultsAnalyticsHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LevelCompletionResultsAnalyticsHelper => ""
    ."LevelCompletionResultsAnalyticsHelper"
);
#[cfg(feature = "LevelCompletionResultsAnalyticsHelper")]
impl std::ops::Deref for LevelCompletionResultsAnalyticsHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelCompletionResultsAnalyticsHelper")]
impl std::ops::DerefMut for LevelCompletionResultsAnalyticsHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelCompletionResultsAnalyticsHelper")]
impl LevelCompletionResultsAnalyticsHelper {}
#[cfg(feature = "LevelCompletionResultsAnalyticsHelper")]
impl quest_hook::libil2cpp::ObjectType for LevelCompletionResultsAnalyticsHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
