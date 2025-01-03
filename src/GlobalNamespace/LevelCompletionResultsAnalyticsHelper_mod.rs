#[cfg(feature = "LevelCompletionResultsAnalyticsHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelCompletionResultsAnalyticsHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LevelCompletionResultsAnalyticsHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::LevelCompletionResultsAnalyticsHelper => ""
    ."LevelCompletionResultsAnalyticsHelper"
);
#[cfg(feature = "LevelCompletionResultsAnalyticsHelper")]
impl std::ops::Deref for crate::GlobalNamespace::LevelCompletionResultsAnalyticsHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelCompletionResultsAnalyticsHelper")]
impl std::ops::DerefMut
for crate::GlobalNamespace::LevelCompletionResultsAnalyticsHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelCompletionResultsAnalyticsHelper")]
impl crate::GlobalNamespace::LevelCompletionResultsAnalyticsHelper {
    pub fn FillEventData(
        levelCompletionResults: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::LevelCompletionResults,
        >,
        eventData: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut quest_hook::libil2cpp::Il2CppString,
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FillEventData", (levelCompletionResults, eventData))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LevelCompletionResultsAnalyticsHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LevelCompletionResultsAnalyticsHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
