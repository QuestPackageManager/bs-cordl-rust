#[cfg(feature = "UnityEngine+WaitForFixedUpdate")]
#[repr(C)]
#[derive(Debug)]
pub struct WaitForFixedUpdate {
    __cordl_parent: crate::UnityEngine::YieldInstruction,
}
#[cfg(feature = "UnityEngine+WaitForFixedUpdate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::WaitForFixedUpdate => "UnityEngine"
    ."WaitForFixedUpdate"
);
#[cfg(feature = "UnityEngine+WaitForFixedUpdate")]
impl std::ops::Deref for crate::UnityEngine::WaitForFixedUpdate {
    type Target = crate::UnityEngine::YieldInstruction;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+WaitForFixedUpdate")]
impl std::ops::DerefMut for crate::UnityEngine::WaitForFixedUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+WaitForFixedUpdate")]
impl crate::UnityEngine::WaitForFixedUpdate {}
#[cfg(feature = "UnityEngine+WaitForFixedUpdate")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::WaitForFixedUpdate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
