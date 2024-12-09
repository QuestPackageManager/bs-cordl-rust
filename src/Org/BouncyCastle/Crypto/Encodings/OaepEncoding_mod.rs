#[cfg(feature = "Org+BouncyCastle+Crypto+Encodings+OaepEncoding")]
#[repr(C)]
#[derive(Debug)]
pub struct OaepEncoding {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub defHash: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mgf1Hash: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    pub engine: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
    pub random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    pub forEncryption: bool,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Encodings+OaepEncoding")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Encodings::OaepEncoding =>
    "Org.BouncyCastle.Crypto.Encodings"."OaepEncoding"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Encodings+OaepEncoding")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Encodings::OaepEncoding {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Encodings+OaepEncoding")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Encodings::OaepEncoding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Encodings+OaepEncoding")]
impl crate::Org::BouncyCastle::Crypto::Encodings::OaepEncoding {
    pub fn DecodeBlock(
        &mut self,
        inBytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        inLen: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("DecodeBlock", (inBytes, inOff, inLen))?;
        Ok(__cordl_ret)
    }
    pub fn EncodeBlock(
        &mut self,
        inBytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        inLen: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("EncodeBlock", (inBytes, inOff, inLen))?;
        Ok(__cordl_ret)
    }
    pub fn GetInputBlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetInputBlockSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetOutputBlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetOutputBlockSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUnderlyingCipher(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher = __cordl_object
            .invoke("GetUnderlyingCipher", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        forEncryption: bool,
        param: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (forEncryption, param))?;
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
    pub fn New_IAsymmetricBlockCipher0(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher))?;
        Ok(__cordl_object)
    }
    pub fn New_IDigest1(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        hash: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher, hash))?;
        Ok(__cordl_object)
    }
    pub fn New_IDigest_IDigest_Il2CppArray3(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        hash: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        mgf1Hash: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        encodingParams: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher, hash, mgf1Hash, encodingParams))?;
        Ok(__cordl_object)
    }
    pub fn New_IDigest_Il2CppArray2(
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        hash: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        encodingParams: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher, hash, encodingParams))?;
        Ok(__cordl_object)
    }
    pub fn ProcessBlock(
        &mut self,
        inBytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        inLen: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("ProcessBlock", (inBytes, inOff, inLen))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IAsymmetricBlockCipher0(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipher))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IDigest1(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        hash: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipher, hash))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IDigest_IDigest_Il2CppArray3(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        hash: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        mgf1Hash: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        encodingParams: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipher, hash, mgf1Hash, encodingParams))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IDigest_Il2CppArray2(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        hash: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        encodingParams: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (cipher, hash, encodingParams))?;
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
    pub fn maskGeneratorFunction1(
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
            .invoke("maskGeneratorFunction1", (Z, zOff, zLen, length))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Encodings+OaepEncoding")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Encodings::OaepEncoding {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
