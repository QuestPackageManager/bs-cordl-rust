#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+CertTemplateBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct CertTemplateBuilder {
    __cordl_parent: crate::System::Object,
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+CertTemplateBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder => "Org.BouncyCastle.Asn1.Crmf"
    ."CertTemplateBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+CertTemplateBuilder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+CertTemplateBuilder")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+CertTemplateBuilder")]
impl crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder {
    pub fn AddOptional(
        &mut self,
        v: *mut crate::Org::BouncyCastle::Asn1::Asn1EncodableVector,
        tagNo: i32,
        isExplicit: bool,
        obj: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddOptional", (v, tagNo, isExplicit, obj))?;
        Ok(__cordl_ret)
    }
    pub fn Build(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplate = __cordl_object
            .invoke("Build", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetExtensions(
        &mut self,
        extens: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder = __cordl_object
            .invoke("SetExtensions", (extens))?;
        Ok(__cordl_ret)
    }
    pub fn SetIssuer(
        &mut self,
        name: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder = __cordl_object
            .invoke("SetIssuer", (name))?;
        Ok(__cordl_ret)
    }
    pub fn SetIssuerUID(
        &mut self,
        uid: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder = __cordl_object
            .invoke("SetIssuerUID", (uid))?;
        Ok(__cordl_ret)
    }
    pub fn SetPublicKey(
        &mut self,
        spki: *mut crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder = __cordl_object
            .invoke("SetPublicKey", (spki))?;
        Ok(__cordl_ret)
    }
    pub fn SetSerialNumber(
        &mut self,
        ser: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder = __cordl_object
            .invoke("SetSerialNumber", (ser))?;
        Ok(__cordl_ret)
    }
    pub fn SetSigningAlg(
        &mut self,
        aid: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder = __cordl_object
            .invoke("SetSigningAlg", (aid))?;
        Ok(__cordl_ret)
    }
    pub fn SetSubject(
        &mut self,
        name: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder = __cordl_object
            .invoke("SetSubject", (name))?;
        Ok(__cordl_ret)
    }
    pub fn SetSubjectUID(
        &mut self,
        uid: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder = __cordl_object
            .invoke("SetSubjectUID", (uid))?;
        Ok(__cordl_ret)
    }
    pub fn SetValidity(
        &mut self,
        v: *mut crate::Org::BouncyCastle::Asn1::Crmf::OptionalValidity,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder = __cordl_object
            .invoke("SetValidity", (v))?;
        Ok(__cordl_ret)
    }
    pub fn SetVersion(
        &mut self,
        ver: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder = __cordl_object
            .invoke("SetVersion", (ver))?;
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
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Crmf+CertTemplateBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Crmf::CertTemplateBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
