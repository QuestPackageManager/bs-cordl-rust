#[cfg(feature = "Internal+Runtime+Augments+RuntimeAugments")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeAugments {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Internal+Runtime+Augments+RuntimeAugments")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Internal::Runtime::Augments::RuntimeAugments =>
    "Internal.Runtime.Augments"."RuntimeAugments"
);
#[cfg(feature = "Internal+Runtime+Augments+RuntimeAugments")]
impl std::ops::Deref for crate::Internal::Runtime::Augments::RuntimeAugments {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Runtime+Augments+RuntimeAugments")]
impl std::ops::DerefMut for crate::Internal::Runtime::Augments::RuntimeAugments {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Runtime+Augments+RuntimeAugments")]
impl crate::Internal::Runtime::Augments::RuntimeAugments {}
#[cfg(feature = "Internal+Runtime+Augments+RuntimeAugments")]
impl quest_hook::libil2cpp::ObjectType
for crate::Internal::Runtime::Augments::RuntimeAugments {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
