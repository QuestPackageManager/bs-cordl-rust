#[cfg(feature = "Unity+Properties+Internal+DefaultPropertyBagInitializer")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultPropertyBagInitializer {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Unity+Properties+Internal+DefaultPropertyBagInitializer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::DefaultPropertyBagInitializer =>
    "Unity.Properties.Internal"."DefaultPropertyBagInitializer"
);
#[cfg(feature = "Unity+Properties+Internal+DefaultPropertyBagInitializer")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::DefaultPropertyBagInitializer {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+DefaultPropertyBagInitializer")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::DefaultPropertyBagInitializer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+DefaultPropertyBagInitializer")]
impl crate::Unity::Properties::Internal::DefaultPropertyBagInitializer {
    pub fn Initialize() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Properties+Internal+DefaultPropertyBagInitializer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::DefaultPropertyBagInitializer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
