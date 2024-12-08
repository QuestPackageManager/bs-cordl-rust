#[cfg(feature = "UnityEngine+StackTraceUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct StackTraceUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+StackTraceUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::StackTraceUtility => "UnityEngine"
    ."StackTraceUtility"
);
#[cfg(feature = "UnityEngine+StackTraceUtility")]
impl std::ops::Deref for crate::UnityEngine::StackTraceUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+StackTraceUtility")]
impl std::ops::DerefMut for crate::UnityEngine::StackTraceUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+StackTraceUtility")]
impl crate::UnityEngine::StackTraceUtility {}
#[cfg(feature = "UnityEngine+StackTraceUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::StackTraceUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
