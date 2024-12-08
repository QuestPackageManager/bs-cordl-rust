#[cfg(feature = "Org+BouncyCastle+Asn1+BC+LinkedCertificate")]
#[repr(C)]
#[derive(Debug)]
pub struct LinkedCertificate {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub mDigest: *mut crate::Org::BouncyCastle::Asn1::X509::DigestInfo,
    pub mCertLocation: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    pub mCertIssuer: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    pub mCACerts: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralNames,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BC+LinkedCertificate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::BC::LinkedCertificate
    => "Org.BouncyCastle.Asn1.BC"."LinkedCertificate"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+BC+LinkedCertificate")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::BC::LinkedCertificate {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BC+LinkedCertificate")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::BC::LinkedCertificate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BC+LinkedCertificate")]
impl crate::Org::BouncyCastle::Asn1::BC::LinkedCertificate {
    pub fn New_Asn1Sequence2(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_DigestInfo_GeneralName0(
        digest: *mut crate::Org::BouncyCastle::Asn1::X509::DigestInfo,
        certLocation: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (digest, certLocation))?;
        Ok(__cordl_object)
    }
    pub fn New_DigestInfo_GeneralName_X509Name_GeneralNames1(
        digest: *mut crate::Org::BouncyCastle::Asn1::X509::DigestInfo,
        certLocation: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        certIssuer: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
        caCerts: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralNames,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (digest, certLocation, certIssuer, caCerts))?;
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
    pub fn _ctor_Asn1Sequence2(
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
    pub fn _ctor_DigestInfo_GeneralName0(
        &mut self,
        digest: *mut crate::Org::BouncyCastle::Asn1::X509::DigestInfo,
        certLocation: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (digest, certLocation))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DigestInfo_GeneralName_X509Name_GeneralNames1(
        &mut self,
        digest: *mut crate::Org::BouncyCastle::Asn1::X509::DigestInfo,
        certLocation: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
        certIssuer: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
        caCerts: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralNames,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (digest, certLocation, certIssuer, caCerts))?;
        Ok(__cordl_ret)
    }
    pub fn get_CACerts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::GeneralNames,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralNames = __cordl_object
            .invoke("get_CACerts", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CertIssuer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name = __cordl_object
            .invoke("get_CertIssuer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CertLocation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::GeneralName = __cordl_object
            .invoke("get_CertLocation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Digest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::DigestInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::DigestInfo = __cordl_object
            .invoke("get_Digest", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+BC+LinkedCertificate")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::BC::LinkedCertificate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}