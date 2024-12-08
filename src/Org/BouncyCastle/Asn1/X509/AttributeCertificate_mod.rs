#[cfg(feature = "Org+BouncyCastle+Asn1+X509+AttributeCertificate")]
#[repr(C)]
#[derive(Debug)]
pub struct AttributeCertificate {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub acinfo: *mut crate::Org::BouncyCastle::Asn1::X509::AttributeCertificateInfo,
    pub signatureAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub signatureValue: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+AttributeCertificate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::X509::AttributeCertificate => "Org.BouncyCastle.Asn1.X509"
    ."AttributeCertificate"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+AttributeCertificate")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::AttributeCertificate {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+AttributeCertificate")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X509::AttributeCertificate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+AttributeCertificate")]
impl crate::Org::BouncyCastle::Asn1::X509::AttributeCertificate {
    pub fn GetSignatureOctets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetSignatureOctets", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Asn1Sequence1(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_AttributeCertificateInfo_AlgorithmIdentifier_DerBitString0(
        acinfo: *mut crate::Org::BouncyCastle::Asn1::X509::AttributeCertificateInfo,
        signatureAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        signatureValue: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (acinfo, signatureAlgorithm, signatureValue))?;
        Ok(__cordl_object)
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1Sequence1(
        &mut self,
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seq))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_AttributeCertificateInfo_AlgorithmIdentifier_DerBitString0(
        &mut self,
        acinfo: *mut crate::Org::BouncyCastle::Asn1::X509::AttributeCertificateInfo,
        signatureAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        signatureValue: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (acinfo, signatureAlgorithm, signatureValue))?;
        Ok(__cordl_ret)
    }
    pub fn get_ACInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AttributeCertificateInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AttributeCertificateInfo = __cordl_object
            .invoke("get_ACInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SignatureAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_SignatureAlgorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SignatureValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerBitString = __cordl_object
            .invoke("get_SignatureValue", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+AttributeCertificate")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::AttributeCertificate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
