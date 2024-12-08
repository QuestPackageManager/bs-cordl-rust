#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampTokenGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct TimeStampTokenGenerator {
    __cordl_parent: crate::System::Object,
    pub accuracySeconds: i32,
    pub accuracyMillis: i32,
    pub accuracyMicros: i32,
    pub ordering: bool,
    pub tsa: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    pub tsaPolicyOID: *mut crate::System::String,
    pub key: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
    pub cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
    pub digestOID: *mut crate::System::String,
    pub signedAttr: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    pub unsignedAttr: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    pub x509Certs: *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
    pub x509Crls: *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampTokenGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Tsp::TimeStampTokenGenerator
    => "Org.BouncyCastle.Tsp"."TimeStampTokenGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampTokenGenerator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampTokenGenerator")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampTokenGenerator")]
impl crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator {
    pub fn SetAccuracyMillis(
        &mut self,
        accuracyMillis: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAccuracyMillis", (accuracyMillis))?;
        Ok(__cordl_ret)
    }
    pub fn SetOrdering(
        &mut self,
        ordering: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetOrdering", (ordering))?;
        Ok(__cordl_ret)
    }
    pub fn SetTsa(
        &mut self,
        tsa: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTsa", (tsa))?;
        Ok(__cordl_ret)
    }
    pub fn Generate(
        &mut self,
        request: *mut crate::Org::BouncyCastle::Tsp::TimeStampRequest,
        serialNumber: *mut crate::Org::BouncyCastle::Math::BigInteger,
        genTime: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Tsp::TimeStampToken,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Tsp::TimeStampToken = __cordl_object
            .invoke("Generate", (request, serialNumber, genTime))?;
        Ok(__cordl_ret)
    }
    pub fn SetAccuracySeconds(
        &mut self,
        accuracySeconds: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAccuracySeconds", (accuracySeconds))?;
        Ok(__cordl_ret)
    }
    pub fn SetCertificates(
        &mut self,
        certificates: *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCertificates", (certificates))?;
        Ok(__cordl_ret)
    }
    pub fn SetCrls(
        &mut self,
        crls: *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetCrls", (crls))?;
        Ok(__cordl_ret)
    }
    pub fn SetAccuracyMicros(
        &mut self,
        accuracyMicros: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAccuracyMicros", (accuracyMicros))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_AsymmetricKeyParameter_X509Certificate_String_String0(
        &mut self,
        key: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        digestOID: *mut crate::System::String,
        tsaPolicyOID: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (key, cert, digestOID, tsaPolicyOID))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_AttributeTable_AttributeTable1(
        &mut self,
        key: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        digestOID: *mut crate::System::String,
        tsaPolicyOID: *mut crate::System::String,
        signedAttr: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        unsignedAttr: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (key, cert, digestOID, tsaPolicyOID, signedAttr, unsignedAttr),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New_AsymmetricKeyParameter_X509Certificate_String_String0(
        key: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        digestOID: *mut crate::System::String,
        tsaPolicyOID: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (key, cert, digestOID, tsaPolicyOID))?;
        Ok(__cordl_object)
    }
    pub fn New_AttributeTable_AttributeTable1(
        key: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        cert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        digestOID: *mut crate::System::String,
        tsaPolicyOID: *mut crate::System::String,
        signedAttr: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
        unsignedAttr: *mut crate::Org::BouncyCastle::Asn1::Cms::AttributeTable,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (key, cert, digestOID, tsaPolicyOID, signedAttr, unsignedAttr),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+TimeStampTokenGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Tsp::TimeStampTokenGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
