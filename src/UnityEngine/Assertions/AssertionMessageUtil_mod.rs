#[cfg(feature = "UnityEngine+Assertions+AssertionMessageUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct AssertionMessageUtil {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Assertions+AssertionMessageUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Assertions::AssertionMessageUtil =>
    "UnityEngine.Assertions"."AssertionMessageUtil"
);
#[cfg(feature = "UnityEngine+Assertions+AssertionMessageUtil")]
impl std::ops::Deref for crate::UnityEngine::Assertions::AssertionMessageUtil {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Assertions+AssertionMessageUtil")]
impl std::ops::DerefMut for crate::UnityEngine::Assertions::AssertionMessageUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Assertions+AssertionMessageUtil")]
impl crate::UnityEngine::Assertions::AssertionMessageUtil {}
#[cfg(feature = "UnityEngine+Assertions+AssertionMessageUtil")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Assertions::AssertionMessageUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
