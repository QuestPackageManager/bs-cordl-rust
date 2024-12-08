#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CmpCertificate")]
#[repr(C)]
#[derive(Debug)]
pub struct CmpCertificate {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub x509v3PKCert: *mut crate::Org::BouncyCastle::Asn1::X509::X509CertificateStructure,
    pub x509v2AttrCert: *mut crate::Org::BouncyCastle::Asn1::X509::AttributeCertificate,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CmpCertificate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cmp::CmpCertificate =>
    "Org.BouncyCastle.Asn1.Cmp"."CmpCertificate"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CmpCertificate")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CmpCertificate")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CmpCertificate")]
impl crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate {
    pub fn New_AttributeCertificate0(
        x509v2AttrCert: *mut crate::Org::BouncyCastle::Asn1::X509::AttributeCertificate,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (x509v2AttrCert))?;
        Ok(__cordl_object)
    }
    pub fn New_X509CertificateStructure1(
        x509v3PKCert: *mut crate::Org::BouncyCastle::Asn1::X509::X509CertificateStructure,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (x509v3PKCert))?;
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
    pub fn _ctor_AttributeCertificate0(
        &mut self,
        x509v2AttrCert: *mut crate::Org::BouncyCastle::Asn1::X509::AttributeCertificate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (x509v2AttrCert))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_X509CertificateStructure1(
        &mut self,
        x509v3PKCert: *mut crate::Org::BouncyCastle::Asn1::X509::X509CertificateStructure,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (x509v3PKCert))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsX509v3PKCert(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsX509v3PKCert", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_X509v2AttrCert(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AttributeCertificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AttributeCertificate = __cordl_object
            .invoke("get_X509v2AttrCert", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_X509v3PKCert(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::X509CertificateStructure,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::X509CertificateStructure = __cordl_object
            .invoke("get_X509v3PKCert", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CmpCertificate")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cmp::CmpCertificate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
