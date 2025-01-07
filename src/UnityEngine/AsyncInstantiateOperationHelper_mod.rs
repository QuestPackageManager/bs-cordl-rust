#[cfg(feature = "UnityEngine+AsyncInstantiateOperationHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncInstantiateOperationHelper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+AsyncInstantiateOperationHelper")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::AsyncInstantiateOperationHelper {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "AsyncInstantiateOperationHelper";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
impl crate::UnityEngine::AsyncInstantiateOperationHelper {
    pub fn SetAsyncInstantiateOperationResult(
        op: quest_hook::libil2cpp::Gc<crate::UnityEngine::AsyncInstantiateOperation>,
        result: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetAsyncInstantiateOperationResult", (op, result))?;
        Ok(__cordl_ret.into())
    }
}
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
