#[cfg(feature = "Org+BouncyCastle+X509+X509V2CrlGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct X509V2CrlGenerator {
    __cordl_parent: crate::System::Object,
    pub extGenerator: *mut crate::Org::BouncyCastle::Asn1::X509::X509ExtensionsGenerator,
    pub tbsGen: *mut crate::Org::BouncyCastle::Asn1::X509::V2TbsCertListGenerator,
    pub sigOID: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub sigAlgId: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub signatureAlgorithm: *mut crate::System::String,
}
#[cfg(feature = "Org+BouncyCastle+X509+X509V2CrlGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::X509::X509V2CrlGenerator =>
    "Org.BouncyCastle.X509"."X509V2CrlGenerator"
);
#[cfg(feature = "Org+BouncyCastle+X509+X509V2CrlGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::X509::X509V2CrlGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509V2CrlGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::X509::X509V2CrlGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509V2CrlGenerator")]
impl crate::Org::BouncyCastle::X509::X509V2CrlGenerator {
    pub fn AddCrl(
        &mut self,
        other: *mut crate::Org::BouncyCastle::X509::X509Crl,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCrl", (other))?;
        Ok(__cordl_ret)
    }
    pub fn AddCrlEntry_X509Extensions2(
        &mut self,
        userCertificate: *mut crate::Org::BouncyCastle::Math::BigInteger,
        revocationDate: crate::System::DateTime,
        extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCrlEntry", (userCertificate, revocationDate, extensions))?;
        Ok(__cordl_ret)
    }
    pub fn AddCrlEntry_i32_0(
        &mut self,
        userCertificate: *mut crate::Org::BouncyCastle::Math::BigInteger,
        revocationDate: crate::System::DateTime,
        reason: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCrlEntry", (userCertificate, revocationDate, reason))?;
        Ok(__cordl_ret)
    }
    pub fn AddCrlEntry_i32_DateTime1(
        &mut self,
        userCertificate: *mut crate::Org::BouncyCastle::Math::BigInteger,
        revocationDate: crate::System::DateTime,
        reason: i32,
        invalidityDate: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddCrlEntry",
                (userCertificate, revocationDate, reason, invalidityDate),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AddExtension_DerObjectIdentifier_Asn1Encodable1(
        &mut self,
        oid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        critical: bool,
        extensionValue: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddExtension", (oid, critical, extensionValue))?;
        Ok(__cordl_ret)
    }
    pub fn AddExtension_DerObjectIdentifier_Il2CppArray3(
        &mut self,
        oid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        critical: bool,
        extensionValue: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddExtension", (oid, critical, extensionValue))?;
        Ok(__cordl_ret)
    }
    pub fn AddExtension_String_Asn1Encodable0(
        &mut self,
        oid: *mut crate::System::String,
        critical: bool,
        extensionValue: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddExtension", (oid, critical, extensionValue))?;
        Ok(__cordl_ret)
    }
    pub fn AddExtension_String_Il2CppArray2(
        &mut self,
        oid: *mut crate::System::String,
        critical: bool,
        extensionValue: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddExtension", (oid, critical, extensionValue))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateCertList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::TbsCertificateList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::TbsCertificateList = __cordl_object
            .invoke("GenerateCertList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GenerateJcaObject(
        &mut self,
        tbsCrl: *mut crate::Org::BouncyCastle::Asn1::X509::TbsCertificateList,
        algId: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        signature: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::X509::X509Crl> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::X509Crl = __cordl_object
            .invoke("GenerateJcaObject", (tbsCrl, algId, signature))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_AsymmetricKeyParameter0(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::X509::X509Crl> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::X509Crl = __cordl_object
            .invoke("Generate", (privateKey))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_AsymmetricKeyParameter_SecureRandom1(
        &mut self,
        privateKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::X509::X509Crl> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::X509Crl = __cordl_object
            .invoke("Generate", (privateKey, random))?;
        Ok(__cordl_ret)
    }
    pub fn Generate_ISignatureFactory2(
        &mut self,
        signatureCalculatorFactory: *mut crate::Org::BouncyCastle::Crypto::ISignatureFactory,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::X509::X509Crl> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::X509Crl = __cordl_object
            .invoke("Generate", (signatureCalculatorFactory))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetIssuerDN(
        &mut self,
        issuer: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIssuerDN", (issuer))?;
        Ok(__cordl_ret)
    }
    pub fn SetNextUpdate(
        &mut self,
        date: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNextUpdate", (date))?;
        Ok(__cordl_ret)
    }
    pub fn SetSignatureAlgorithm(
        &mut self,
        signatureAlgorithm: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSignatureAlgorithm", (signatureAlgorithm))?;
        Ok(__cordl_ret)
    }
    pub fn SetThisUpdate(
        &mut self,
        date: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetThisUpdate", (date))?;
        Ok(__cordl_ret)
    }
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
    pub fn get_SignatureAlgNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerable = __cordl_object
            .invoke("get_SignatureAlgNames", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+X509V2CrlGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::X509::X509V2CrlGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
