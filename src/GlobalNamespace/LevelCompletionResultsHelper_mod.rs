#[cfg(feature = "LevelCompletionResultsHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelCompletionResultsHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LevelCompletionResultsHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LevelCompletionResultsHelper =>
    ""."LevelCompletionResultsHelper"
);
#[cfg(feature = "LevelCompletionResultsHelper")]
impl std::ops::Deref for crate::GlobalNamespace::LevelCompletionResultsHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelCompletionResultsHelper")]
impl std::ops::DerefMut for crate::GlobalNamespace::LevelCompletionResultsHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelCompletionResultsHelper")]
impl crate::GlobalNamespace::LevelCompletionResultsHelper {}
#[cfg(feature = "LevelCompletionResultsHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LevelCompletionResultsHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
