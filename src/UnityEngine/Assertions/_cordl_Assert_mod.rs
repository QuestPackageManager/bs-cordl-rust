#[cfg(feature = "UnityEngine+Assertions+Assert")]
#[repr(C)]
#[derive(Debug)]
pub struct _cordl_Assert {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Assertions+Assert")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Assertions::_cordl_Assert =>
    "UnityEngine.Assertions"."Assert"
);
#[cfg(feature = "UnityEngine+Assertions+Assert")]
impl std::ops::Deref for crate::UnityEngine::Assertions::_cordl_Assert {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Assertions+Assert")]
impl std::ops::DerefMut for crate::UnityEngine::Assertions::_cordl_Assert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Assertions+Assert")]
impl crate::UnityEngine::Assertions::_cordl_Assert {}
#[cfg(feature = "UnityEngine+Assertions+Assert")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Assertions::_cordl_Assert {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
