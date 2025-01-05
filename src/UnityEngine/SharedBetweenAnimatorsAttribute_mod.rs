#[cfg(feature = "UnityEngine+SharedBetweenAnimatorsAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct SharedBetweenAnimatorsAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "UnityEngine+SharedBetweenAnimatorsAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SharedBetweenAnimatorsAttribute =>
    "UnityEngine"."SharedBetweenAnimatorsAttribute"
);
#[cfg(feature = "UnityEngine+SharedBetweenAnimatorsAttribute")]
impl std::ops::Deref for crate::UnityEngine::SharedBetweenAnimatorsAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SharedBetweenAnimatorsAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::SharedBetweenAnimatorsAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+SharedBetweenAnimatorsAttribute")]
impl crate::UnityEngine::SharedBetweenAnimatorsAttribute {}
#[cfg(feature = "UnityEngine+SharedBetweenAnimatorsAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::SharedBetweenAnimatorsAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
