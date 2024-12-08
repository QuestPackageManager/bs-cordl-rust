#[cfg(feature = "UnityEngine+UIElements+UQueryExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct UQueryExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+UQueryExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UQueryExtensions =>
    "UnityEngine.UIElements"."UQueryExtensions"
);
#[cfg(feature = "UnityEngine+UIElements+UQueryExtensions")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UQueryExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryExtensions")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UQueryExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UQueryExtensions")]
impl crate::UnityEngine::UIElements::UQueryExtensions {}
#[cfg(feature = "UnityEngine+UIElements+UQueryExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UQueryExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
