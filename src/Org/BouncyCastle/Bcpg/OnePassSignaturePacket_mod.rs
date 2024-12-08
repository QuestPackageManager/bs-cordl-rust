#[cfg(feature = "Org+BouncyCastle+Bcpg+OnePassSignaturePacket")]
#[repr(C)]
#[derive(Debug)]
pub struct OnePassSignaturePacket {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::ContainedPacket,
    pub version: i32,
    pub sigType: i32,
    pub hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    pub keyAlgorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
    pub keyId: i64,
    pub nested: i32,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OnePassSignaturePacket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::OnePassSignaturePacket
    => "Org.BouncyCastle.Bcpg"."OnePassSignaturePacket"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OnePassSignaturePacket")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OnePassSignaturePacket {
    type Target = crate::Org::BouncyCastle::Bcpg::ContainedPacket;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OnePassSignaturePacket")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::OnePassSignaturePacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OnePassSignaturePacket")]
impl crate::Org::BouncyCastle::Bcpg::OnePassSignaturePacket {
    pub fn Encode(
        &mut self,
        bcpgOut: *mut crate::Org::BouncyCastle::Bcpg::BcpgOutputStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Encode", (bcpgOut))?;
        Ok(__cordl_ret)
    }
    pub fn get_KeyId(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_KeyId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_KeyAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag = __cordl_object
            .invoke("get_KeyAlgorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SignatureType(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_SignatureType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HashAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag = __cordl_object
            .invoke("get_HashAlgorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_BcpgInputStream0(
        &mut self,
        bcpgIn: *mut crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bcpgIn))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_HashAlgorithmTag_PublicKeyAlgorithmTag_i64__cordl_bool1(
        &mut self,
        sigType: i32,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        keyAlgorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        keyId: i64,
        isNested: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sigType, hashAlgorithm, keyAlgorithm, keyId, isNested))?;
        Ok(__cordl_ret)
    }
    pub fn New_BcpgInputStream0(
        bcpgIn: *mut crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bcpgIn))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_HashAlgorithmTag_PublicKeyAlgorithmTag_i64__cordl_bool1(
        sigType: i32,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        keyAlgorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        keyId: i64,
        isNested: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (sigType, hashAlgorithm, keyAlgorithm, keyId, isNested),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OnePassSignaturePacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OnePassSignaturePacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
