#[cfg(feature = "Mono+Security+X509+X509Crl")]
#[repr(C)]
#[derive(Debug)]
pub struct X509Crl {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub issuer: *mut quest_hook::libil2cpp::Il2CppString,
    pub version: u8,
    pub thisUpdate: crate::System::DateTime,
    pub nextUpdate: crate::System::DateTime,
    pub entries: *mut crate::System::Collections::ArrayList,
    pub signatureOID: *mut quest_hook::libil2cpp::Il2CppString,
    pub signature: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub extensions: *mut crate::Mono::Security::X509::X509ExtensionCollection,
    pub encoded: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub hash_value: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Mono+Security+X509+X509Crl")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::X509::X509Crl =>
    "Mono.Security.X509"."X509Crl"
);
#[cfg(feature = "Mono+Security+X509+X509Crl")]
impl std::ops::Deref for crate::Mono::Security::X509::X509Crl {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+X509Crl")]
impl std::ops::DerefMut for crate::Mono::Security::X509::X509Crl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+X509Crl")]
impl crate::Mono::Security::X509::X509Crl {
    #[cfg(feature = "Mono+Security+X509+X509Crl+X509CrlEntry")]
    pub type X509CrlEntry = crate::Mono::Security::X509::X509Crl_X509CrlEntry;
    pub fn Compare(
        &mut self,
        array1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        array2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Compare", (array1, array2))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCrlEntry_Il2CppArray1(
        &mut self,
        serialNumber: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Crl_X509CrlEntry>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509Crl_X509CrlEntry,
        > = __cordl_object.invoke("GetCrlEntry", (serialNumber))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCrlEntry_X509Certificate0(
        &mut self,
        x509: quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Certificate>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509Crl_X509CrlEntry>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509Crl_X509CrlEntry,
        > = __cordl_object.invoke("GetCrlEntry", (x509))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        crl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (crl))?;
        Ok(__cordl_object.into())
    }
    pub fn Parse(
        &mut self,
        crl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Parse", (crl))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifySignature_AsymmetricAlgorithm2(
        &mut self,
        aa: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::AsymmetricAlgorithm,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("VerifySignature", (aa))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifySignature_DSA0(
        &mut self,
        dsa: quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::DSA>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("VerifySignature", (dsa))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifySignature_RSA1(
        &mut self,
        rsa: quest_hook::libil2cpp::Gc<crate::System::Security::Cryptography::RSA>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("VerifySignature", (rsa))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        crl: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (crl))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Extensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509ExtensionCollection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509ExtensionCollection,
        > = __cordl_object.invoke("get_Extensions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Hash(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("get_Hash", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IssuerName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_IssuerName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NextUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_NextUpdate", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Security+X509+X509Crl")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Security::X509::X509Crl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Security+X509+X509Crl+X509CrlEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct X509Crl_X509CrlEntry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub sn: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub revocationDate: crate::System::DateTime,
    pub extensions: *mut crate::Mono::Security::X509::X509ExtensionCollection,
}
#[cfg(feature = "Mono+Security+X509+X509Crl+X509CrlEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::X509::X509Crl_X509CrlEntry =>
    "Mono.Security.X509"."X509Crl/X509CrlEntry"
);
#[cfg(feature = "Mono+Security+X509+X509Crl+X509CrlEntry")]
impl std::ops::Deref for crate::Mono::Security::X509::X509Crl_X509CrlEntry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+X509Crl+X509CrlEntry")]
impl std::ops::DerefMut for crate::Mono::Security::X509::X509Crl_X509CrlEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+X509+X509Crl+X509CrlEntry")]
impl crate::Mono::Security::X509::X509Crl_X509CrlEntry {
    pub fn New(
        entry: quest_hook::libil2cpp::Gc<crate::Mono::Security::ASN1>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (entry))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        entry: quest_hook::libil2cpp::Gc<crate::Mono::Security::ASN1>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (entry))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Extensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Security::X509::X509ExtensionCollection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Security::X509::X509ExtensionCollection,
        > = __cordl_object.invoke("get_Extensions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RevocationDate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_RevocationDate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SerialNumber(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("get_SerialNumber", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Security+X509+X509Crl+X509CrlEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::X509::X509Crl_X509CrlEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
