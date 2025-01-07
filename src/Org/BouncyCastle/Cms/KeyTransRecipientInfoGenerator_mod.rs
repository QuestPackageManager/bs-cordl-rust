#[cfg(feature = "Org+BouncyCastle+Cms+KeyTransRecipientInfoGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyTransRecipientInfoGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub recipientTbsCert: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::TbsCertificateStructure,
    >,
    pub recipientPublicKey: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    >,
    pub subjectKeyIdentifier: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    >,
    pub info: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
    >,
    pub issuerAndSerialNumber: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Cms::IssuerAndSerialNumber,
    >,
    pub random: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Security::SecureRandom,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyTransRecipientInfoGenerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cms::KeyTransRecipientInfoGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cms";
    const CLASS_NAME: &'static str = "KeyTransRecipientInfoGenerator";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyTransRecipientInfoGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::KeyTransRecipientInfoGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyTransRecipientInfoGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Cms::KeyTransRecipientInfoGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyTransRecipientInfoGenerator")]
impl crate::Org::BouncyCastle::Cms::KeyTransRecipientInfoGenerator {
    pub fn Generate(
        &mut self,
        contentEncryptionKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cms::RecipientInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::RecipientInfo,
        > = __cordl_object.invoke("Generate", (contentEncryptionKey, random))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateWrappedKey(
        &mut self,
        contentEncryptionKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::KeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GenerateWrappedKey", (contentEncryptionKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray2(
        subjectKeyIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (subjectKeyIdentifier))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IssuerAndSerialNumber1(
        issuerAndSerialNumber: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::IssuerAndSerialNumber,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (issuerAndSerialNumber))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray2(
        &mut self,
        subjectKeyIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (subjectKeyIdentifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IssuerAndSerialNumber1(
        &mut self,
        issuerAndSerialNumber: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cms::IssuerAndSerialNumber,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (issuerAndSerialNumber))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AlgorithmDetails(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        > = __cordl_object.invoke("get_AlgorithmDetails", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_RecipientCert(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RecipientCert", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_RecipientPublicKey(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_RecipientPublicKey", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SubjectKeyIdentifier(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1OctetString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SubjectKeyIdentifier", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyTransRecipientInfoGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::KeyTransRecipientInfoGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyTransRecipientInfoGenerator")]
impl AsRef<crate::Org::BouncyCastle::Cms::RecipientInfoGenerator>
for crate::Org::BouncyCastle::Cms::KeyTransRecipientInfoGenerator {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Cms::RecipientInfoGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+KeyTransRecipientInfoGenerator")]
impl AsMut<crate::Org::BouncyCastle::Cms::RecipientInfoGenerator>
for crate::Org::BouncyCastle::Cms::KeyTransRecipientInfoGenerator {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Cms::RecipientInfoGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
