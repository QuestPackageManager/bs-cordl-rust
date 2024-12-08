#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+CertTemplate")]
#[repr(C)]
#[derive(Debug)]
pub struct CertTemplate {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    pub version: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub serialNumber: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub signingAlg: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub issuer: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    pub validity: *mut crate::Org::BouncyCastle::Asn1::Crmf::OptionalValidity,
    pub subject: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    pub publicKey: *mut crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
    pub issuerUID: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    pub subjectUID: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    pub extensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+CertTemplate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Crmf::CertTemplate =>
    "Org.BouncyCastle.Asn1.Crmf"."CertTemplate"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+CertTemplate")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+CertTemplate")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+CertTemplate")]
impl crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate {
    pub fn get_PublicKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo = __cordl_object
            .invoke("get_PublicKey", ())?;
        Ok(__cordl_ret)
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
    pub fn get_Extensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions = __cordl_object
            .invoke("get_Extensions", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Validity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::OptionalValidity,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::OptionalValidity = __cordl_object
            .invoke("get_Validity", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SigningAlg(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_SigningAlg", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SerialNumber(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::DerInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerInteger = __cordl_object
            .invoke("get_SerialNumber", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IssuerUID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerBitString = __cordl_object
            .invoke("get_IssuerUID", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SubjectUID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerBitString = __cordl_object
            .invoke("get_SubjectUID", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Issuer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name = __cordl_object
            .invoke("get_Issuer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Subject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name = __cordl_object
            .invoke("get_Subject", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Version(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Version", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
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
    pub fn New(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+CertTemplate")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
