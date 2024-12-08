#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2AttributeCertificateInfoGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct V2AttributeCertificateInfoGenerator {
    __cordl_parent: crate::System::Object,
    pub version: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub holder: *mut crate::Org::BouncyCastle::Asn1::X509::Holder,
    pub issuer: *mut crate::Org::BouncyCastle::Asn1::X509::AttCertIssuer,
    pub signature: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub serialNumber: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub attributes: *mut crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
    pub issuerUniqueID: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    pub extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    pub startDate: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    pub endDate: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2AttributeCertificateInfoGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::X509::V2AttributeCertificateInfoGenerator =>
    "Org.BouncyCastle.Asn1.X509"."V2AttributeCertificateInfoGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2AttributeCertificateInfoGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::X509::V2AttributeCertificateInfoGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2AttributeCertificateInfoGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::X509::V2AttributeCertificateInfoGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2AttributeCertificateInfoGenerator")]
impl crate::Org::BouncyCastle::Asn1::X509::V2AttributeCertificateInfoGenerator {
    pub fn AddAttribute_String_Asn1Encodable0(
        &mut self,
        oid: *mut crate::System::String,
        value: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAttribute", (oid, value))?;
        Ok(__cordl_ret)
    }
    pub fn AddAttribute_AttributeX509_1(
        &mut self,
        attribute: *mut crate::Org::BouncyCastle::Asn1::X509::AttributeX509,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAttribute", (attribute))?;
        Ok(__cordl_ret)
    }
    pub fn SetIssuer(
        &mut self,
        issuer: *mut crate::Org::BouncyCastle::Asn1::X509::AttCertIssuer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIssuer", (issuer))?;
        Ok(__cordl_ret)
    }
    pub fn SetIssuerUniqueID(
        &mut self,
        issuerUniqueID: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIssuerUniqueID", (issuerUniqueID))?;
        Ok(__cordl_ret)
    }
    pub fn SetSignature(
        &mut self,
        signature: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSignature", (signature))?;
        Ok(__cordl_ret)
    }
    pub fn SetHolder(
        &mut self,
        holder: *mut crate::Org::BouncyCastle::Asn1::X509::Holder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHolder", (holder))?;
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
    pub fn SetSerialNumber(
        &mut self,
        serialNumber: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSerialNumber", (serialNumber))?;
        Ok(__cordl_ret)
    }
    pub fn SetEndDate(
        &mut self,
        endDate: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetEndDate", (endDate))?;
        Ok(__cordl_ret)
    }
    pub fn SetStartDate(
        &mut self,
        startDate: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStartDate", (startDate))?;
        Ok(__cordl_ret)
    }
    pub fn SetExtensions(
        &mut self,
        extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetExtensions", (extensions))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateAttributeCertificateInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AttributeCertificateInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AttributeCertificateInfo = __cordl_object
            .invoke("GenerateAttributeCertificateInfo", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2AttributeCertificateInfoGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::V2AttributeCertificateInfoGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
