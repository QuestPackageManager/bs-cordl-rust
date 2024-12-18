#[cfg(feature = "OperationResultExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct OperationResultExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OperationResultExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OperationResultExtensions => ""
    ."OperationResultExtensions"
);
#[cfg(feature = "OperationResultExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::OperationResultExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OperationResultExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::OperationResultExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OperationResultExtensions")]
impl crate::GlobalNamespace::OperationResultExtensions {}
#[cfg(feature = "OperationResultExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OperationResultExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
