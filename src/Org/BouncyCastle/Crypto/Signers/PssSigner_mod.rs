#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+PssSigner")]
#[repr(C)]
#[derive(Debug)]
pub struct PssSigner {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub contentDigest1: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    pub contentDigest2: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    pub mgfDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    pub cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
    pub random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    pub hLen: i32,
    pub mgfhLen: i32,
    pub sLen: i32,
    pub sSet: bool,
    pub emBits: i32,
    pub salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mDash: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub block: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub trailer: u8,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+PssSigner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Signers::PssSigner =>
    "Org.BouncyCastle.Crypto.Signers"."PssSigner"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+PssSigner")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Signers::PssSigner {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+PssSigner")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Signers::PssSigner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+PssSigner")]
impl crate::Org::BouncyCastle::Crypto::Signers::PssSigner {
    pub const TrailerImplicit: u8 = 188u8;
    pub fn BlockUpdate(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BlockUpdate", (input, inOff, length))?;
        Ok(__cordl_ret)
    }
    pub fn ClearBlock(
        &mut self,
        block: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearBlock", (block))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateSignature(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GenerateSignature", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        forSigning: bool,
        parameters: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (forSigning, parameters))?;
        Ok(__cordl_ret)
    }
    pub fn ItoOSP(
        &mut self,
        i: i32,
        sp: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ItoOSP", (i, sp))?;
        Ok(__cordl_ret)
    }
    pub fn MaskGeneratorFunction1(
        &mut self,
        Z: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        zOff: i32,
        zLen: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("MaskGeneratorFunction1", (Z, zOff, zLen, length))?;
        Ok(__cordl_ret)
    }
    pub fn New_IAsymmetricBlockCipher_IDigest0(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher, digest))?;
        Ok(__cordl_object)
    }
    pub fn New_IDigest_IDigest_i32_Il2CppArray_u8_7(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        contentDigest1: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        contentDigest2: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        mgfDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        saltLen: i32,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        trailer: u8,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    cipher,
                    contentDigest1,
                    contentDigest2,
                    mgfDigest,
                    saltLen,
                    salt,
                    trailer,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_IDigest_Il2CppArray4(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        contentDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        mgfDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher, contentDigest, mgfDigest, salt))?;
        Ok(__cordl_object)
    }
    pub fn New_IDigest_i32_3(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        contentDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        mgfDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        saltLen: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher, contentDigest, mgfDigest, saltLen))?;
        Ok(__cordl_object)
    }
    pub fn New_IDigest_i32_u8_6(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        contentDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        mgfDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        saltLen: i32,
        trailer: u8,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher, contentDigest, mgfDigest, saltLen, trailer))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray2(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher, digest, salt))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_1(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        saltLen: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher, digest, saltLen))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_u8_5(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        saltLen: i32,
        trailer: u8,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher, digest, saltLen, trailer))?;
        Ok(__cordl_object)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
        input: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (input))?;
        Ok(__cordl_ret)
    }
    pub fn VerifySignature(
        &mut self,
        signature: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("VerifySignature", (signature))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IAsymmetricBlockCipher_IDigest0(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipher, digest))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IDigest_IDigest_i32_Il2CppArray_u8_7(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        contentDigest1: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        contentDigest2: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        mgfDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        saltLen: i32,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        trailer: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    cipher,
                    contentDigest1,
                    contentDigest2,
                    mgfDigest,
                    saltLen,
                    salt,
                    trailer,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IDigest_Il2CppArray4(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        contentDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        mgfDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipher, contentDigest, mgfDigest, salt))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IDigest_i32_3(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        contentDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        mgfDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        saltLen: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipher, contentDigest, mgfDigest, saltLen))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IDigest_i32_u8_6(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        contentDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        mgfDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        saltLen: i32,
        trailer: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipher, contentDigest, mgfDigest, saltLen, trailer))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray2(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        salt: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipher, digest, salt))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_1(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        saltLen: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipher, digest, saltLen))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_u8_5(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        saltLen: i32,
        trailer: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipher, digest, saltLen, trailer))?;
        Ok(__cordl_ret)
    }
    pub fn get_AlgorithmName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_AlgorithmName", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+PssSigner")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Signers::PssSigner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
