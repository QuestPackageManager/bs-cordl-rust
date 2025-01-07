#[cfg(feature = "UnityEngine+Events+UnityEventTools")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityEventTools {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Events+UnityEventTools")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::Events::UnityEventTools {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Events";
    const CLASS_NAME: &'static str = "UnityEventTools";
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
#[cfg(feature = "UnityEngine+Events+UnityEventTools")]
impl std::ops::Deref for crate::UnityEngine::Events::UnityEventTools {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Events+UnityEventTools")]
impl std::ops::DerefMut for crate::UnityEngine::Events::UnityEventTools {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Events+UnityEventTools")]
impl crate::UnityEngine::Events::UnityEventTools {
    pub fn TidyAssemblyTypeName(
        assemblyTypeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TidyAssemblyTypeName", (assemblyTypeName))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Events+UnityEventTools")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Events::UnityEventTools {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
