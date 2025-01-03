#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V2AttributeCertificateInfoGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct V2AttributeCertificateInfoGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub version: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::DerInteger>,
    pub holder: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::Holder>,
    pub issuer: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::AttCertIssuer,
    >,
    pub signature: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    >,
    pub serialNumber: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::DerInteger,
    >,
    pub attributes: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
    >,
    pub issuerUniqueID: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::DerBitString,
    >,
    pub extensions: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    >,
    pub startDate: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    >,
    pub endDate: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    >,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn AddAttribute_AttributeX509_1(
        &mut self,
        attribute: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AttributeX509,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAttribute", (attribute))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddAttribute_Il2CppString_Asn1Encodable0(
        &mut self,
        oid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Asn1Encodable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAttribute", (oid, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateAttributeCertificateInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AttributeCertificateInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AttributeCertificateInfo,
        > = __cordl_object.invoke("GenerateAttributeCertificateInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetEndDate(
        &mut self,
        endDate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetEndDate", (endDate))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetExtensions(
        &mut self,
        extensions: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetExtensions", (extensions))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetHolder(
        &mut self,
        holder: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::X509::Holder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHolder", (holder))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIssuer(
        &mut self,
        issuer: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AttCertIssuer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIssuer", (issuer))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIssuerUniqueID(
        &mut self,
        issuerUniqueID: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerBitString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIssuerUniqueID", (issuerUniqueID))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSerialNumber(
        &mut self,
        serialNumber: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerInteger,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSerialNumber", (serialNumber))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSignature(
        &mut self,
        signature: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSignature", (signature))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStartDate(
        &mut self,
        startDate: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStartDate", (startDate))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
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
