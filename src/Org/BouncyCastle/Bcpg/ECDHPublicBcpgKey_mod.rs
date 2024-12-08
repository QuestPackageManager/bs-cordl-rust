#[cfg(feature = "Org+BouncyCastle+Bcpg+ECDHPublicBcpgKey")]
#[repr(C)]
#[derive(Debug)]
pub struct ECDHPublicBcpgKey {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::ECPublicBcpgKey,
    pub reserved: u8,
    pub hashFunctionId: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
    pub symAlgorithmId: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+ECDHPublicBcpgKey")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::ECDHPublicBcpgKey =>
    "Org.BouncyCastle.Bcpg"."ECDHPublicBcpgKey"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+ECDHPublicBcpgKey")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::ECDHPublicBcpgKey {
    type Target = crate::Org::BouncyCastle::Bcpg::ECPublicBcpgKey;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+ECDHPublicBcpgKey")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::ECDHPublicBcpgKey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+ECDHPublicBcpgKey")]
impl crate::Org::BouncyCastle::Bcpg::ECDHPublicBcpgKey {
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
    pub fn get_SymmetricKeyAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag = __cordl_object
            .invoke("get_SymmetricKeyAlgorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn VerifyHashAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("VerifyHashAlgorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn VerifySymmetricKeyAlgorithm(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("VerifySymmetricKeyAlgorithm", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Reserved(&mut self) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("get_Reserved", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn _ctor_DerObjectIdentifier_ECPoint_HashAlgorithmTag_SymmetricKeyAlgorithmTag1(
        &mut self,
        oid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        point: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        symmetricKeyAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (oid, point, hashAlgorithm, symmetricKeyAlgorithm))?;
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
    pub fn New_DerObjectIdentifier_ECPoint_HashAlgorithmTag_SymmetricKeyAlgorithmTag1(
        oid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        point: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        hashAlgorithm: crate::Org::BouncyCastle::Bcpg::HashAlgorithmTag,
        symmetricKeyAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (oid, point, hashAlgorithm, symmetricKeyAlgorithm))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+ECDHPublicBcpgKey")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::ECDHPublicBcpgKey {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
