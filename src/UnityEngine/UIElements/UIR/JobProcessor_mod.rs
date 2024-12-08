#[cfg(feature = "UnityEngine+UIElements+UIR+JobProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct JobProcessor {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+JobProcessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::JobProcessor =>
    "UnityEngine.UIElements.UIR"."JobProcessor"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+JobProcessor")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::JobProcessor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+JobProcessor")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::JobProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+JobProcessor")]
impl crate::UnityEngine::UIElements::UIR::JobProcessor {}
#[cfg(feature = "UnityEngine+UIElements+UIR+JobProcessor")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::JobProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
