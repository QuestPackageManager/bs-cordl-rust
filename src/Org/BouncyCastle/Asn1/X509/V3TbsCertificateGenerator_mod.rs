#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V3TbsCertificateGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct V3TbsCertificateGenerator {
    __cordl_parent: crate::System::Object,
    pub version: *mut crate::Org::BouncyCastle::Asn1::DerTaggedObject,
    pub serialNumber: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub signature: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub issuer: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    pub startDate: *mut crate::Org::BouncyCastle::Asn1::X509::Time,
    pub endDate: *mut crate::Org::BouncyCastle::Asn1::X509::Time,
    pub subject: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    pub subjectPublicKeyInfo: *mut crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
    pub extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    pub altNamePresentAndCritical: bool,
    pub issuerUniqueID: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    pub subjectUniqueID: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V3TbsCertificateGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::X509::V3TbsCertificateGenerator =>
    "Org.BouncyCastle.Asn1.X509"."V3TbsCertificateGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V3TbsCertificateGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::X509::V3TbsCertificateGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V3TbsCertificateGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::X509::V3TbsCertificateGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V3TbsCertificateGenerator")]
impl crate::Org::BouncyCastle::Asn1::X509::V3TbsCertificateGenerator {
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
    pub fn SetSubject(
        &mut self,
        subject: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSubject", (subject))?;
        Ok(__cordl_ret)
    }
    pub fn SetEndDate_DerUtcTime0(
        &mut self,
        endDate: *mut crate::Org::BouncyCastle::Asn1::DerUtcTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetEndDate", (endDate))?;
        Ok(__cordl_ret)
    }
    pub fn SetEndDate_Time1(
        &mut self,
        endDate: *mut crate::Org::BouncyCastle::Asn1::X509::Time,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetEndDate", (endDate))?;
        Ok(__cordl_ret)
    }
    pub fn SetStartDate_DerUtcTime0(
        &mut self,
        startDate: *mut crate::Org::BouncyCastle::Asn1::DerUtcTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStartDate", (startDate))?;
        Ok(__cordl_ret)
    }
    pub fn SetStartDate_Time1(
        &mut self,
        startDate: *mut crate::Org::BouncyCastle::Asn1::X509::Time,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStartDate", (startDate))?;
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
    pub fn SetIssuerUniqueID(
        &mut self,
        uniqueID: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIssuerUniqueID", (uniqueID))?;
        Ok(__cordl_ret)
    }
    pub fn SetSubjectPublicKeyInfo(
        &mut self,
        pubKeyInfo: *mut crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSubjectPublicKeyInfo", (pubKeyInfo))?;
        Ok(__cordl_ret)
    }
    pub fn SetIssuer(
        &mut self,
        issuer: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIssuer", (issuer))?;
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
    pub fn SetSubjectUniqueID(
        &mut self,
        uniqueID: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSubjectUniqueID", (uniqueID))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateTbsCertificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::TbsCertificateStructure,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::TbsCertificateStructure = __cordl_object
            .invoke("GenerateTbsCertificate", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+V3TbsCertificateGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::V3TbsCertificateGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
