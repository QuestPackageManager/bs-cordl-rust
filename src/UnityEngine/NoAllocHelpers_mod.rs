#[cfg(feature = "UnityEngine+NoAllocHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct NoAllocHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+NoAllocHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::NoAllocHelpers => "UnityEngine"
    ."NoAllocHelpers"
);
#[cfg(feature = "UnityEngine+NoAllocHelpers")]
impl std::ops::Deref for crate::UnityEngine::NoAllocHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+NoAllocHelpers")]
impl std::ops::DerefMut for crate::UnityEngine::NoAllocHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+NoAllocHelpers")]
impl crate::UnityEngine::NoAllocHelpers {}
#[cfg(feature = "UnityEngine+NoAllocHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::NoAllocHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
