#[cfg(feature = "UnityEngine+UI+LayoutUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct LayoutUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UI+LayoutUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UI::LayoutUtility =>
    "UnityEngine.UI"."LayoutUtility"
);
#[cfg(feature = "UnityEngine+UI+LayoutUtility")]
impl std::ops::Deref for crate::UnityEngine::UI::LayoutUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+LayoutUtility")]
impl std::ops::DerefMut for crate::UnityEngine::UI::LayoutUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UI+LayoutUtility")]
impl crate::UnityEngine::UI::LayoutUtility {
    #[cfg(feature = "UnityEngine+UI+LayoutUtility+__c")]
    pub type __c = crate::UnityEngine::UI::LayoutUtility___c;
}
#[cfg(feature = "UnityEngine+UI+LayoutUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UI::LayoutUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
