#[cfg(feature = "UnityEngine+UIElements+PointerType")]
#[repr(C)]
#[derive(Debug)]
pub struct PointerType {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+PointerType")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::PointerType =>
    "UnityEngine.UIElements"."PointerType"
);
#[cfg(feature = "UnityEngine+UIElements+PointerType")]
impl std::ops::Deref for crate::UnityEngine::UIElements::PointerType {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerType")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::PointerType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PointerType")]
impl crate::UnityEngine::UIElements::PointerType {}
#[cfg(feature = "UnityEngine+UIElements+PointerType")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::PointerType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
