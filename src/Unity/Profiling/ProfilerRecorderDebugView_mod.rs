#[cfg(feature = "Unity+Profiling+ProfilerRecorderDebugView")]
#[repr(C)]
#[derive(Debug)]
pub struct ProfilerRecorderDebugView {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Unity+Profiling+ProfilerRecorderDebugView")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Profiling::ProfilerRecorderDebugView =>
    "Unity.Profiling"."ProfilerRecorderDebugView"
);
#[cfg(feature = "Unity+Profiling+ProfilerRecorderDebugView")]
impl std::ops::Deref for crate::Unity::Profiling::ProfilerRecorderDebugView {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerRecorderDebugView")]
impl std::ops::DerefMut for crate::Unity::Profiling::ProfilerRecorderDebugView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Profiling+ProfilerRecorderDebugView")]
impl crate::Unity::Profiling::ProfilerRecorderDebugView {}
#[cfg(feature = "Unity+Profiling+ProfilerRecorderDebugView")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Profiling::ProfilerRecorderDebugView {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
