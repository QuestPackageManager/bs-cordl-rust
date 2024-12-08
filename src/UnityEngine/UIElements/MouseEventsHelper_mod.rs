#[cfg(feature = "UnityEngine+UIElements+MouseEventsHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct MouseEventsHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+MouseEventsHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::MouseEventsHelper =>
    "UnityEngine.UIElements"."MouseEventsHelper"
);
#[cfg(feature = "UnityEngine+UIElements+MouseEventsHelper")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MouseEventsHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MouseEventsHelper")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::MouseEventsHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MouseEventsHelper")]
impl crate::UnityEngine::UIElements::MouseEventsHelper {}
#[cfg(feature = "UnityEngine+UIElements+MouseEventsHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MouseEventsHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
