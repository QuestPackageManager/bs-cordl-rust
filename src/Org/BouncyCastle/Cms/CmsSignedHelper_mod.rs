#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedHelper")]
#[repr(C)]
#[derive(Debug)]
pub struct CmsSignedHelper {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedHelper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Cms::CmsSignedHelper =>
    "Org.BouncyCastle.Cms"."CmsSignedHelper"
);
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedHelper")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cms::CmsSignedHelper {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedHelper")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cms::CmsSignedHelper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedHelper")]
impl crate::Org::BouncyCastle::Cms::CmsSignedHelper {
    pub fn CreateAttributeStore(
        &mut self,
        _cordl_type: *mut crate::System::String,
        certSet: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::Store::IX509Store = __cordl_object
            .invoke("CreateAttributeStore", (_cordl_type, certSet))?;
        Ok(__cordl_ret)
    }
    pub fn GetEncOid(
        &mut self,
        key: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        digestOID: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetEncOid", (key, digestOID))?;
        Ok(__cordl_ret)
    }
    pub fn GetDigestAliases(
        &mut self,
        algName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("GetDigestAliases", (algName))?;
        Ok(__cordl_ret)
    }
    pub fn GetSignatureInstance(
        &mut self,
        algorithm: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Crypto::ISigner> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::ISigner = __cordl_object
            .invoke("GetSignatureInstance", (algorithm))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCrlStore(
        &mut self,
        _cordl_type: *mut crate::System::String,
        crlSet: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::Store::IX509Store = __cordl_object
            .invoke("CreateCrlStore", (_cordl_type, crlSet))?;
        Ok(__cordl_ret)
    }
    pub fn GetDigestAlgName(
        &mut self,
        digestAlgOid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetDigestAlgName", (digestAlgOid))?;
        Ok(__cordl_ret)
    }
    pub fn AddCertsFromSet(
        &mut self,
        certs: *mut crate::System::Collections::IList,
        certSet: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCertsFromSet", (certs, certSet))?;
        Ok(__cordl_ret)
    }
    pub fn GetEncAlgorithmIdentifier(
        &mut self,
        encOid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        sigX509Parameters: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("GetEncAlgorithmIdentifier", (encOid, sigX509Parameters))?;
        Ok(__cordl_ret)
    }
    pub fn GetDigestInstance(
        &mut self,
        algorithm: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Crypto::IDigest> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IDigest = __cordl_object
            .invoke("GetDigestInstance", (algorithm))?;
        Ok(__cordl_ret)
    }
    pub fn AddCrlsFromSet(
        &mut self,
        crls: *mut crate::System::Collections::IList,
        crlSet: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddCrlsFromSet", (crls, crlSet))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCertificateStore(
        &mut self,
        _cordl_type: *mut crate::System::String,
        certSet: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::X509::Store::IX509Store,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::X509::Store::IX509Store = __cordl_object
            .invoke("CreateCertificateStore", (_cordl_type, certSet))?;
        Ok(__cordl_ret)
    }
    pub fn GetEncryptionAlgName(
        &mut self,
        encryptionAlgOid: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetEncryptionAlgName", (encryptionAlgOid))?;
        Ok(__cordl_ret)
    }
    pub fn FixAlgID(
        &mut self,
        algId: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("FixAlgID", (algId))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Cms+CmsSignedHelper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cms::CmsSignedHelper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
