#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+SignerInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct SignerInfo {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub version: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub issuerAndSerialNumber: *mut crate::Org::BouncyCastle::Asn1::Pkcs::IssuerAndSerialNumber,
    pub digAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub authenticatedAttributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    pub digEncryptionAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub encryptedDigest: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    pub unauthenticatedAttributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+SignerInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Pkcs::SignerInfo =>
    "Org.BouncyCastle.Asn1.Pkcs"."SignerInfo"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+SignerInfo")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Pkcs::SignerInfo {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+SignerInfo")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Pkcs::SignerInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+SignerInfo")]
impl crate::Org::BouncyCastle::Asn1::Pkcs::SignerInfo {
    pub fn New_Asn1Sequence1(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_DerInteger_IssuerAndSerialNumber_AlgorithmIdentifier_Asn1Set_AlgorithmIdentifier_Asn1OctetString_Asn1Set0(
        version: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        issuerAndSerialNumber: *mut crate::Org::BouncyCastle::Asn1::Pkcs::IssuerAndSerialNumber,
        digAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        authenticatedAttributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        digEncryptionAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        encryptedDigest: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
        unauthenticatedAttributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    version,
                    issuerAndSerialNumber,
                    digAlgorithm,
                    authenticatedAttributes,
                    digEncryptionAlgorithm,
                    encryptedDigest,
                    unauthenticatedAttributes,
                ),
            )?;
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
    pub fn _ctor_DerInteger_IssuerAndSerialNumber_AlgorithmIdentifier_Asn1Set_AlgorithmIdentifier_Asn1OctetString_Asn1Set0(
        &mut self,
        version: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
        issuerAndSerialNumber: *mut crate::Org::BouncyCastle::Asn1::Pkcs::IssuerAndSerialNumber,
        digAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        authenticatedAttributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        digEncryptionAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        encryptedDigest: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
        unauthenticatedAttributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    version,
                    issuerAndSerialNumber,
                    digAlgorithm,
                    authenticatedAttributes,
                    digEncryptionAlgorithm,
                    encryptedDigest,
                    unauthenticatedAttributes,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_AuthenticatedAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Set> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Set = __cordl_object
            .invoke("get_AuthenticatedAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DigestAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_DigestAlgorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DigestEncryptionAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_DigestEncryptionAlgorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EncryptedDigest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString = __cordl_object
            .invoke("get_EncryptedDigest", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IssuerAndSerialNumber(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Pkcs::IssuerAndSerialNumber,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Pkcs::IssuerAndSerialNumber = __cordl_object
            .invoke("get_IssuerAndSerialNumber", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UnauthenticatedAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Set> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Set = __cordl_object
            .invoke("get_UnauthenticatedAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Version(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::DerInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerInteger = __cordl_object
            .invoke("get_Version", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+SignerInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Pkcs::SignerInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
