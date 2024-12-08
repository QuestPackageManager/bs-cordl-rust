#[cfg(feature = "UnityEngine+UINumericFieldsUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct UINumericFieldsUtils {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UINumericFieldsUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UINumericFieldsUtils =>
    "UnityEngine"."UINumericFieldsUtils"
);
#[cfg(feature = "UnityEngine+UINumericFieldsUtils")]
impl std::ops::Deref for crate::UnityEngine::UINumericFieldsUtils {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UINumericFieldsUtils")]
impl std::ops::DerefMut for crate::UnityEngine::UINumericFieldsUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UINumericFieldsUtils")]
impl crate::UnityEngine::UINumericFieldsUtils {}
#[cfg(feature = "UnityEngine+UINumericFieldsUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UINumericFieldsUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
