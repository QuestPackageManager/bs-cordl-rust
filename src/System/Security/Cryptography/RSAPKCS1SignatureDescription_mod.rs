#[cfg(feature = "System+Security+Cryptography+RSAPKCS1SignatureDescription")]
#[repr(C)]
#[derive(Debug)]
pub struct RSAPKCS1SignatureDescription {
    __cordl_parent: crate::System::Security::Cryptography::SignatureDescription,
    pub _hashAlgorithm: *mut crate::System::String,
}
#[cfg(feature = "System+Security+Cryptography+RSAPKCS1SignatureDescription")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::RSAPKCS1SignatureDescription =>
    "System.Security.Cryptography"."RSAPKCS1SignatureDescription"
);
#[cfg(feature = "System+Security+Cryptography+RSAPKCS1SignatureDescription")]
impl std::ops::Deref
for crate::System::Security::Cryptography::RSAPKCS1SignatureDescription {
    type Target = crate::System::Security::Cryptography::SignatureDescription;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+RSAPKCS1SignatureDescription")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::RSAPKCS1SignatureDescription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+RSAPKCS1SignatureDescription")]
impl crate::System::Security::Cryptography::RSAPKCS1SignatureDescription {
    pub fn New(
        hashAlgorithm: *mut crate::System::String,
        digestAlgorithm: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hashAlgorithm, digestAlgorithm))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        hashAlgorithm: *mut crate::System::String,
        digestAlgorithm: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (hashAlgorithm, digestAlgorithm))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Security+Cryptography+RSAPKCS1SignatureDescription")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::RSAPKCS1SignatureDescription {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
