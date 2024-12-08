#[cfg(feature = "LevelCompletionResultsHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct LevelCompletionResultsHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "LevelCompletionResultsHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for LevelCompletionResultsHelper => ""
    ."LevelCompletionResultsHelper"
);
#[cfg(feature = "LevelCompletionResultsHelper")]
impl std::ops::Deref for LevelCompletionResultsHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LevelCompletionResultsHelper")]
impl std::ops::DerefMut for LevelCompletionResultsHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LevelCompletionResultsHelper")]
impl LevelCompletionResultsHelper {}
#[cfg(feature = "LevelCompletionResultsHelper")]
impl quest_hook::libil2cpp::ObjectType for LevelCompletionResultsHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
