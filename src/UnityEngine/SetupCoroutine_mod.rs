#[cfg(feature = "UnityEngine+SetupCoroutine")]
#[repr(C)]
#[derive(Debug)]
pub struct SetupCoroutine {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+SetupCoroutine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SetupCoroutine => "UnityEngine"
    ."SetupCoroutine"
);
#[cfg(feature = "UnityEngine+SetupCoroutine")]
impl std::ops::Deref for crate::UnityEngine::SetupCoroutine {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SetupCoroutine")]
impl std::ops::DerefMut for crate::UnityEngine::SetupCoroutine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SetupCoroutine")]
impl crate::UnityEngine::SetupCoroutine {
    pub fn InvokeMember(
        behaviour: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        variable: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeMember", (behaviour, name, variable))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeMoveNext(
        enumerator: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
        returnValueAddress: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InvokeMoveNext", (enumerator, returnValueAddress))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+SetupCoroutine")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::SetupCoroutine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
