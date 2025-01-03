#[cfg(feature = "Org+BouncyCastle+Operators+CmsContentEncryptorBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsContentEncryptorBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub encryptionOID: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub keySize: i32,
    pub helper: *mut crate::Org::BouncyCastle::Cms::EnvelopedDataHelper,
}
#[cfg(feature = "Org+BouncyCastle+Operators+CmsContentEncryptorBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Operators::CmsContentEncryptorBuilder =>
    "Org.BouncyCastle.Operators"."CmsContentEncryptorBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Operators+CmsContentEncryptorBuilder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Operators::CmsContentEncryptorBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Operators+CmsContentEncryptorBuilder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Operators::CmsContentEncryptorBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Operators+CmsContentEncryptorBuilder")]
impl crate::Org::BouncyCastle::Operators::CmsContentEncryptorBuilder {
    pub fn Build(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherBuilderWithKey,
        > = __cordl_object.invoke("Build", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetKeySize(
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetKeySize", (oid))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_DerObjectIdentifier0(
        encryptionOID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encryptionOID))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_1(
        encryptionOID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        keySize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encryptionOID, keySize))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_DerObjectIdentifier0(
        &mut self,
        encryptionOID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encryptionOID))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        encryptionOID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        keySize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encryptionOID, keySize))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Operators+CmsContentEncryptorBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Operators::CmsContentEncryptorBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
