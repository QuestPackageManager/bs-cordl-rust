#[cfg(feature = "UnityEngine+Profiling+Profiler")]
#[repr(C)]
#[derive(Debug)]
pub struct Profiler {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Profiling+Profiler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Profiling::Profiler =>
    "UnityEngine.Profiling"."Profiler"
);
#[cfg(feature = "UnityEngine+Profiling+Profiler")]
impl std::ops::Deref for crate::UnityEngine::Profiling::Profiler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Profiling+Profiler")]
impl std::ops::DerefMut for crate::UnityEngine::Profiling::Profiler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Profiling+Profiler")]
impl crate::UnityEngine::Profiling::Profiler {}
#[cfg(feature = "UnityEngine+Profiling+Profiler")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Profiling::Profiler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
