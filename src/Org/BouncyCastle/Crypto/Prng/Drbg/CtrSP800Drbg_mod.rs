#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+CtrSP800Drbg")]
#[repr(C)]
#[derive(Debug)]
pub struct CtrSP800Drbg {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mEntropySource: *mut crate::Org::BouncyCastle::Crypto::IEntropySource,
    pub mEngine: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    pub mKeySizeInBits: i32,
    pub mSeedLength: i32,
    pub mSecurityStrength: i32,
    pub mKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mV: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mReseedCounter: i64,
    pub mIsTdea: bool,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+CtrSP800Drbg")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Prng::Drbg::CtrSP800Drbg =>
    "Org.BouncyCastle.Crypto.Prng.Drbg"."CtrSP800Drbg"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+CtrSP800Drbg")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Prng::Drbg::CtrSP800Drbg {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+CtrSP800Drbg")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Prng::Drbg::CtrSP800Drbg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+CtrSP800Drbg")]
impl crate::Org::BouncyCastle::Crypto::Prng::Drbg::CtrSP800Drbg {
    pub fn AddOneTo(
        &mut self,
        longer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddOneTo", (longer))?;
        Ok(__cordl_ret)
    }
    pub fn BCC(
        &mut self,
        bccOut: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        k: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        iV: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BCC", (bccOut, k, iV, data))?;
        Ok(__cordl_ret)
    }
    pub fn Block_Cipher_df(
        &mut self,
        inputString: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        bitLength: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Block_Cipher_df", (inputString, bitLength))?;
        Ok(__cordl_ret)
    }
    pub fn CTR_DRBG_Instantiate_algorithm(
        &mut self,
        entropy: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        personalisationString: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CTR_DRBG_Instantiate_algorithm",
                (entropy, nonce, personalisationString),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CTR_DRBG_Reseed_algorithm(
        &mut self,
        additionalInput: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CTR_DRBG_Reseed_algorithm", (additionalInput))?;
        Ok(__cordl_ret)
    }
    pub fn CTR_DRBG_Update(
        &mut self,
        seed: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        v: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CTR_DRBG_Update", (seed, key, v))?;
        Ok(__cordl_ret)
    }
    pub fn ExpandKey(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("ExpandKey", (key))?;
        Ok(__cordl_ret)
    }
    pub fn Generate(
        &mut self,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        additionalInput: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        predictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Generate", (output, additionalInput, predictionResistant))?;
        Ok(__cordl_ret)
    }
    pub fn GetEntropy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetEntropy", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetMaxSecurityStrength(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
        keySizeInBits: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetMaxSecurityStrength", (cipher, keySizeInBits))?;
        Ok(__cordl_ret)
    }
    pub fn IsTdea(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsTdea", (cipher))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        engine: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
        keySizeInBits: i32,
        securityStrength: i32,
        entropySource: *mut crate::Org::BouncyCastle::Crypto::IEntropySource,
        personalizationString: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    engine,
                    keySizeInBits,
                    securityStrength,
                    entropySource,
                    personalizationString,
                    nonce,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn PadKey(
        &mut self,
        keyMaster: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        keyOff: i32,
        tmp: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        tmpOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PadKey", (keyMaster, keyOff, tmp, tmpOff))?;
        Ok(__cordl_ret)
    }
    pub fn Reseed(
        &mut self,
        additionalInput: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reseed", (additionalInput))?;
        Ok(__cordl_ret)
    }
    pub fn XOR(
        &mut self,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        a: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        b: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        bOff: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("XOR", (output, a, b, bOff))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        engine: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
        keySizeInBits: i32,
        securityStrength: i32,
        entropySource: *mut crate::Org::BouncyCastle::Crypto::IEntropySource,
        personalizationString: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    engine,
                    keySizeInBits,
                    securityStrength,
                    entropySource,
                    personalizationString,
                    nonce,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn copyIntToByteArray(
        &mut self,
        buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        value: i32,
        offSet: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("copyIntToByteArray", (buf, value, offSet))?;
        Ok(__cordl_ret)
    }
    pub fn get_BlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_BlockSize", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+CtrSP800Drbg")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Prng::Drbg::CtrSP800Drbg {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
