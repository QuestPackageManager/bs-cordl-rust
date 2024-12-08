#[cfg(feature = "UnityEngine+UIElements+ComputedTransitionUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct ComputedTransitionUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+ComputedTransitionUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ComputedTransitionUtils
    => "UnityEngine.UIElements"."ComputedTransitionUtils"
);
#[cfg(feature = "UnityEngine+UIElements+ComputedTransitionUtils")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ComputedTransitionUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ComputedTransitionUtils")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ComputedTransitionUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ComputedTransitionUtils")]
impl crate::UnityEngine::UIElements::ComputedTransitionUtils {
    #[cfg(feature = "UnityEngine+UIElements+ComputedTransitionUtils+__c")]
    pub type __c = crate::UnityEngine::UIElements::ComputedTransitionUtils___c;
}
#[cfg(feature = "UnityEngine+UIElements+ComputedTransitionUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::ComputedTransitionUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
