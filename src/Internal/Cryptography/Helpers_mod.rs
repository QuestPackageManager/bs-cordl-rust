#[cfg(feature = "Internal+Cryptography+Helpers")]
#[repr(C)]
#[derive(Debug)]
pub struct Helpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Internal+Cryptography+Helpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Internal::Cryptography::Helpers =>
    "Internal.Cryptography"."Helpers"
);
#[cfg(feature = "Internal+Cryptography+Helpers")]
impl std::ops::Deref for crate::Internal::Cryptography::Helpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Cryptography+Helpers")]
impl std::ops::DerefMut for crate::Internal::Cryptography::Helpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Internal+Cryptography+Helpers")]
impl crate::Internal::Cryptography::Helpers {}
#[cfg(feature = "Internal+Cryptography+Helpers")]
impl quest_hook::libil2cpp::ObjectType for crate::Internal::Cryptography::Helpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
