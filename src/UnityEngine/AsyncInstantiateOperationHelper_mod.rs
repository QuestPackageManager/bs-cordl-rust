#[cfg(feature = "UnityEngine+AsyncInstantiateOperationHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncInstantiateOperationHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+AsyncInstantiateOperationHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AsyncInstantiateOperationHelper =>
    "UnityEngine"."AsyncInstantiateOperationHelper"
);
#[cfg(feature = "UnityEngine+AsyncInstantiateOperationHelper")]
impl std::ops::Deref for crate::UnityEngine::AsyncInstantiateOperationHelper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AsyncInstantiateOperationHelper")]
impl std::ops::DerefMut for crate::UnityEngine::AsyncInstantiateOperationHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AsyncInstantiateOperationHelper")]
impl crate::UnityEngine::AsyncInstantiateOperationHelper {}
#[cfg(feature = "UnityEngine+AsyncInstantiateOperationHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AsyncInstantiateOperationHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
