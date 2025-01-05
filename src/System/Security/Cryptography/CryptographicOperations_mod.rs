#[cfg(feature = "System+Security+Cryptography+CryptographicOperations")]
#[repr(C)]
#[derive(Debug)]
pub struct CryptographicOperations {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Security+Cryptography+CryptographicOperations")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::CryptographicOperations =>
    "System.Security.Cryptography"."CryptographicOperations"
);
#[cfg(feature = "System+Security+Cryptography+CryptographicOperations")]
impl std::ops::Deref for crate::System::Security::Cryptography::CryptographicOperations {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+CryptographicOperations")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::CryptographicOperations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+CryptographicOperations")]
impl crate::System::Security::Cryptography::CryptographicOperations {
    pub fn ZeroMemory(
        buffer: crate::System::Span_1<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ZeroMemory", (buffer))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Security+Cryptography+CryptographicOperations")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::CryptographicOperations {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
