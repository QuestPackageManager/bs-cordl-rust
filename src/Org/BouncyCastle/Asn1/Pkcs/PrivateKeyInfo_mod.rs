#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PrivateKeyInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct PrivateKeyInfo {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub version: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub privateKeyAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub privateKey: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    pub attributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    pub publicKey: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PrivateKeyInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo =>
    "Org.BouncyCastle.Asn1.Pkcs"."PrivateKeyInfo"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PrivateKeyInfo")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PrivateKeyInfo")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PrivateKeyInfo")]
impl crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo {
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
    pub fn ParsePrivateKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("ParsePrivateKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasPublicKey(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasPublicKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PrivateKeyAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_PrivateKeyAlgorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_AlgorithmIdentifier_Asn1Encodable0(
        &mut self,
        privateKeyAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        privateKey: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privateKeyAlgorithm, privateKey))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_AlgorithmIdentifier_Asn1Encodable_Asn1Set1(
        &mut self,
        privateKeyAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        privateKey: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
        attributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privateKeyAlgorithm, privateKey, attributes))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_AlgorithmIdentifier_Asn1Encodable_Asn1Set_Il2CppArray2(
        &mut self,
        privateKeyAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        privateKey: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
        attributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        publicKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (privateKeyAlgorithm, privateKey, attributes, publicKey))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1Sequence3(
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
    pub fn ParsePublicKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("ParsePublicKey", ())?;
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
    pub fn get_Attributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Set> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Set = __cordl_object
            .invoke("get_Attributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PrivateKeyData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString = __cordl_object
            .invoke("get_PrivateKeyData", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PublicKeyData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerBitString = __cordl_object
            .invoke("get_PublicKeyData", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_AlgorithmIdentifier_Asn1Encodable0(
        privateKeyAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        privateKey: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privateKeyAlgorithm, privateKey))?;
        Ok(__cordl_object)
    }
    pub fn New_AlgorithmIdentifier_Asn1Encodable_Asn1Set1(
        privateKeyAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        privateKey: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
        attributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (privateKeyAlgorithm, privateKey, attributes))?;
        Ok(__cordl_object)
    }
    pub fn New_AlgorithmIdentifier_Asn1Encodable_Asn1Set_Il2CppArray2(
        privateKeyAlgorithm: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        privateKey: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
        attributes: *mut crate::Org::BouncyCastle::Asn1::Asn1Set,
        publicKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (privateKeyAlgorithm, privateKey, attributes, publicKey),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_Asn1Sequence3(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+PrivateKeyInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Pkcs::PrivateKeyInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
