#[cfg(feature = "Org+BouncyCastle+Asn1+X509+SubjectPublicKeyInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct SubjectPublicKeyInfo {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub algID: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub keyData: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+SubjectPublicKeyInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo => "Org.BouncyCastle.Asn1.X509"
    ."SubjectPublicKeyInfo"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+SubjectPublicKeyInfo")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+SubjectPublicKeyInfo")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+SubjectPublicKeyInfo")]
impl crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo {
    pub fn get_AlgorithmID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_AlgorithmID", ())?;
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
    pub fn GetPublicKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("GetPublicKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_AlgorithmIdentifier_Asn1Encodable0(
        &mut self,
        algID: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        publicKey: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algID, publicKey))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_AlgorithmIdentifier_Il2CppArray1(
        &mut self,
        algID: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        publicKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algID, publicKey))?;
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
    pub fn New_AlgorithmIdentifier_Asn1Encodable0(
        algID: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        publicKey: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algID, publicKey))?;
        Ok(__cordl_object)
    }
    pub fn New_AlgorithmIdentifier_Il2CppArray1(
        algID: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        publicKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algID, publicKey))?;
        Ok(__cordl_object)
    }
    pub fn New_Asn1Sequence2(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+SubjectPublicKeyInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
