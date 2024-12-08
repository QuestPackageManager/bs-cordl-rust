#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertifiedKeyPair")]
#[repr(C)]
#[derive(Debug)]
pub struct CertifiedKeyPair {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub certOrEncCert: *mut crate::Org::BouncyCastle::Asn1::Cmp::CertOrEncCert,
    pub privateKey: *mut crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue,
    pub publicationInfo: *mut crate::Org::BouncyCastle::Asn1::Crmf::PkiPublicationInfo,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertifiedKeyPair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cmp::CertifiedKeyPair
    => "Org.BouncyCastle.Asn1.Cmp"."CertifiedKeyPair"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertifiedKeyPair")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cmp::CertifiedKeyPair {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertifiedKeyPair")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cmp::CertifiedKeyPair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertifiedKeyPair")]
impl crate::Org::BouncyCastle::Asn1::Cmp::CertifiedKeyPair {
    pub fn New_Asn1Sequence0(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_CertOrEncCert1(
        certOrEncCert: *mut crate::Org::BouncyCastle::Asn1::Cmp::CertOrEncCert,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certOrEncCert))?;
        Ok(__cordl_object)
    }
    pub fn New_CertOrEncCert_EncryptedValue_PkiPublicationInfo2(
        certOrEncCert: *mut crate::Org::BouncyCastle::Asn1::Cmp::CertOrEncCert,
        privateKey: *mut crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue,
        publicationInfo: *mut crate::Org::BouncyCastle::Asn1::Crmf::PkiPublicationInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certOrEncCert, privateKey, publicationInfo))?;
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
    pub fn _ctor_Asn1Sequence0(
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
    pub fn _ctor_CertOrEncCert1(
        &mut self,
        certOrEncCert: *mut crate::Org::BouncyCastle::Asn1::Cmp::CertOrEncCert,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certOrEncCert))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_CertOrEncCert_EncryptedValue_PkiPublicationInfo2(
        &mut self,
        certOrEncCert: *mut crate::Org::BouncyCastle::Asn1::Cmp::CertOrEncCert,
        privateKey: *mut crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue,
        publicationInfo: *mut crate::Org::BouncyCastle::Asn1::Crmf::PkiPublicationInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certOrEncCert, privateKey, publicationInfo))?;
        Ok(__cordl_ret)
    }
    pub fn get_CertOrEncCert(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::CertOrEncCert,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::CertOrEncCert = __cordl_object
            .invoke("get_CertOrEncCert", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PrivateKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue = __cordl_object
            .invoke("get_PrivateKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PublicationInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::PkiPublicationInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::PkiPublicationInfo = __cordl_object
            .invoke("get_PublicationInfo", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertifiedKeyPair")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cmp::CertifiedKeyPair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
