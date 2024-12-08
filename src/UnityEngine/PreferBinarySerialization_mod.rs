#[cfg(feature = "UnityEngine+PreferBinarySerialization")]
#[repr(C)]
#[derive(Debug)]
pub struct PreferBinarySerialization {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "UnityEngine+PreferBinarySerialization")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::PreferBinarySerialization =>
    "UnityEngine"."PreferBinarySerialization"
);
#[cfg(feature = "UnityEngine+PreferBinarySerialization")]
impl std::ops::Deref for crate::UnityEngine::PreferBinarySerialization {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+PreferBinarySerialization")]
impl std::ops::DerefMut for crate::UnityEngine::PreferBinarySerialization {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+PreferBinarySerialization")]
impl crate::UnityEngine::PreferBinarySerialization {}
#[cfg(feature = "UnityEngine+PreferBinarySerialization")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::PreferBinarySerialization {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
