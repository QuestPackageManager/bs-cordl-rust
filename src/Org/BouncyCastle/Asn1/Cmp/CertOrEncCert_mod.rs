#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertOrEncCert")]
#[repr(C)]
#[derive(Debug)]
pub struct CertOrEncCert {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub certificate: *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
    pub encryptedCert: *mut crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertOrEncCert")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cmp::CertOrEncCert =>
    "Org.BouncyCastle.Asn1.Cmp"."CertOrEncCert"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertOrEncCert")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cmp::CertOrEncCert {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertOrEncCert")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cmp::CertOrEncCert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertOrEncCert")]
impl crate::Org::BouncyCastle::Asn1::Cmp::CertOrEncCert {
    pub fn New_Asn1TaggedObject0(
        tagged: *mut crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tagged))?;
        Ok(__cordl_object)
    }
    pub fn New_CmpCertificate1(
        certificate: *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certificate))?;
        Ok(__cordl_object)
    }
    pub fn New_EncryptedValue2(
        encryptedCert: *mut crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (encryptedCert))?;
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
    pub fn _ctor_Asn1TaggedObject0(
        &mut self,
        tagged: *mut crate::Org::BouncyCastle::Asn1::Asn1TaggedObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tagged))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_CmpCertificate1(
        &mut self,
        certificate: *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certificate))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_EncryptedValue2(
        &mut self,
        encryptedCert: *mut crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (encryptedCert))?;
        Ok(__cordl_ret)
    }
    pub fn get_Certificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate = __cordl_object
            .invoke("get_Certificate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EncryptedCert(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::EncryptedValue = __cordl_object
            .invoke("get_EncryptedCert", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertOrEncCert")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cmp::CertOrEncCert {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
