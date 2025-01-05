#[cfg(feature = "UnityEngine+Events+UnityEventTools")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityEventTools {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Events+UnityEventTools")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Events::UnityEventTools =>
    "UnityEngine.Events"."UnityEventTools"
);
#[cfg(feature = "UnityEngine+Events+UnityEventTools")]
impl std::ops::Deref for crate::UnityEngine::Events::UnityEventTools {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
