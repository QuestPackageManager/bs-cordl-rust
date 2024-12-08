#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Iso9796d2Signer")]
#[repr(C)]
#[derive(Debug)]
pub struct Iso9796d2Signer {
    __cordl_parent: crate::System::Object,
    pub digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    pub cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
    pub trailer: i32,
    pub keyBits: i32,
    pub block: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mBuf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub messageLength: i32,
    pub fullMessage: bool,
    pub recoveredMessage: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub preSig: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub preBlock: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Iso9796d2Signer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Signers::Iso9796d2Signer =>
    "Org.BouncyCastle.Crypto.Signers"."Iso9796d2Signer"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Iso9796d2Signer")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2Signer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Iso9796d2Signer")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2Signer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Iso9796d2Signer")]
impl crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2Signer {
    pub const TrailerImplicit: i32 = 188i32;
    pub const TrailerRipeMD128: i32 = 13004i32;
    pub const TrailerRipeMD160: i32 = 12748i32;
    pub const TrailerSha1: i32 = 13260i32;
    pub const TrailerSha256: i32 = 13516i32;
    pub const TrailerSha384: i32 = 14028i32;
    pub const TrailerSha512: i32 = 13772i32;
    pub const TrailerWhirlpool: i32 = 14284i32;
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
    pub fn GetRecoveredMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetRecoveredMessage", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool0(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        isImplicit: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipher, digest, isImplicit))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IAsymmetricBlockCipher_IDigest1(
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
    pub fn get_AlgorithmName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AlgorithmName", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsSameAs(
        &mut self,
        a: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        b: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSameAs", (a, b))?;
        Ok(__cordl_ret)
    }
    pub fn HasFullMessage(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasFullMessage", ())?;
        Ok(__cordl_ret)
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
    pub fn ReturnFalse(
        &mut self,
        block: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ReturnFalse", (block))?;
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
    pub fn UpdateWithRecoveredMessage(
        &mut self,
        signature: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateWithRecoveredMessage", (signature))?;
        Ok(__cordl_ret)
    }
    pub fn New__cordl_bool0(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        isImplicit: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher, digest, isImplicit))?;
        Ok(__cordl_object)
    }
    pub fn New_IAsymmetricBlockCipher_IDigest1(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher, digest))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Iso9796d2Signer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2Signer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
