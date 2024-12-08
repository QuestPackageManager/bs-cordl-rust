#[cfg(feature = "UnityEngine+SetupCoroutine")]
#[repr(C)]
#[derive(Debug)]
pub struct SetupCoroutine {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+SetupCoroutine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SetupCoroutine => "UnityEngine"
    ."SetupCoroutine"
);
#[cfg(feature = "UnityEngine+SetupCoroutine")]
impl std::ops::Deref for crate::UnityEngine::SetupCoroutine {
    type Target = crate::System::Object;
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
impl crate::UnityEngine::SetupCoroutine {}
#[cfg(feature = "UnityEngine+SetupCoroutine")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::SetupCoroutine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
