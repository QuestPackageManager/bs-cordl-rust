#[cfg(feature = "UnityEngine+UIElements+StyleValueFunctionExtension")]
#[repr(C)]
#[derive(Debug)]
pub struct StyleValueFunctionExtension {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueFunctionExtension")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleValueFunctionExtension => "UnityEngine.UIElements"
    ."StyleValueFunctionExtension"
);
#[cfg(feature = "UnityEngine+UIElements+StyleValueFunctionExtension")]
impl std::ops::Deref for crate::UnityEngine::UIElements::StyleValueFunctionExtension {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueFunctionExtension")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::StyleValueFunctionExtension {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleValueFunctionExtension")]
impl crate::UnityEngine::UIElements::StyleValueFunctionExtension {}
#[cfg(feature = "UnityEngine+UIElements+StyleValueFunctionExtension")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::StyleValueFunctionExtension {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}