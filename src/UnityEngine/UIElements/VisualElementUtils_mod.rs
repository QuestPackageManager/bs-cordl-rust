#[cfg(feature = "UnityEngine+UIElements+VisualElementUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualElementUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::VisualElementUtils =>
    "UnityEngine.UIElements"."VisualElementUtils"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElementUtils")]
impl std::ops::Deref for crate::UnityEngine::UIElements::VisualElementUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementUtils")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::VisualElementUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementUtils")]
impl crate::UnityEngine::UIElements::VisualElementUtils {
    #[cfg(feature = "UnityEngine+UIElements+VisualElementUtils+__c")]
    pub type __c = crate::UnityEngine::UIElements::VisualElementUtils___c;
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualElementUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}