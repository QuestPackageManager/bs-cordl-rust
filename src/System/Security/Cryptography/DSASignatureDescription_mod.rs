#[cfg(feature = "System+Security+Cryptography+DSASignatureDescription")]
#[repr(C)]
#[derive(Debug)]
pub struct DSASignatureDescription {
    __cordl_parent: crate::System::Security::Cryptography::SignatureDescription,
}
#[cfg(feature = "System+Security+Cryptography+DSASignatureDescription")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::Cryptography::DSASignatureDescription =>
    "System.Security.Cryptography"."DSASignatureDescription"
);
#[cfg(feature = "System+Security+Cryptography+DSASignatureDescription")]
impl std::ops::Deref for crate::System::Security::Cryptography::DSASignatureDescription {
    type Target = crate::System::Security::Cryptography::SignatureDescription;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+DSASignatureDescription")]
impl std::ops::DerefMut
for crate::System::Security::Cryptography::DSASignatureDescription {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Security+Cryptography+DSASignatureDescription")]
impl crate::System::Security::Cryptography::DSASignatureDescription {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Security+Cryptography+DSASignatureDescription")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Security::Cryptography::DSASignatureDescription {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
