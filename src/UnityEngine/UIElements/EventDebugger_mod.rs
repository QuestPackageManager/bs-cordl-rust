#[cfg(feature = "UnityEngine+UIElements+EventDebugger")]
#[repr(C)]
#[derive(Debug)]
pub struct EventDebugger {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+EventDebugger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::EventDebugger =>
    "UnityEngine.UIElements"."EventDebugger"
);
#[cfg(feature = "UnityEngine+UIElements+EventDebugger")]
impl std::ops::Deref for crate::UnityEngine::UIElements::EventDebugger {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventDebugger")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::EventDebugger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventDebugger")]
impl crate::UnityEngine::UIElements::EventDebugger {}
#[cfg(feature = "UnityEngine+UIElements+EventDebugger")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::EventDebugger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
