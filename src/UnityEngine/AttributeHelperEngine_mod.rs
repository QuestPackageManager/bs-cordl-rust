#[cfg(feature = "UnityEngine+AttributeHelperEngine")]
#[repr(C)]
#[derive(Debug)]
pub struct AttributeHelperEngine {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+AttributeHelperEngine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AttributeHelperEngine =>
    "UnityEngine"."AttributeHelperEngine"
);
#[cfg(feature = "UnityEngine+AttributeHelperEngine")]
impl std::ops::Deref for crate::UnityEngine::AttributeHelperEngine {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AttributeHelperEngine")]
impl std::ops::DerefMut for crate::UnityEngine::AttributeHelperEngine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AttributeHelperEngine")]
impl crate::UnityEngine::AttributeHelperEngine {}
#[cfg(feature = "UnityEngine+AttributeHelperEngine")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AttributeHelperEngine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}