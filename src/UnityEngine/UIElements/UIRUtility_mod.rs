#[cfg(feature = "UnityEngine+UIElements+UIRUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct UIRUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+UIRUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIRUtility =>
    "UnityEngine.UIElements"."UIRUtility"
);
#[cfg(feature = "UnityEngine+UIElements+UIRUtility")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIRUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIRUtility")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIRUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIRUtility")]
impl crate::UnityEngine::UIElements::UIRUtility {}
#[cfg(feature = "UnityEngine+UIElements+UIRUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::UIRUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
