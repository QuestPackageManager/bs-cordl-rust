#[cfg(feature = "UnityEngine+UIElements+VisualElementListPool")]
#[repr(C)]
#[derive(Debug)]
pub struct VisualElementListPool {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementListPool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::VisualElementListPool
    => "UnityEngine.UIElements"."VisualElementListPool"
);
#[cfg(feature = "UnityEngine+UIElements+VisualElementListPool")]
impl std::ops::Deref for crate::UnityEngine::UIElements::VisualElementListPool {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementListPool")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::VisualElementListPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementListPool")]
impl crate::UnityEngine::UIElements::VisualElementListPool {
    #[cfg(feature = "UnityEngine+UIElements+VisualElementListPool+__c")]
    pub type __c = crate::UnityEngine::UIElements::VisualElementListPool___c;
}
#[cfg(feature = "UnityEngine+UIElements+VisualElementListPool")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::VisualElementListPool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
