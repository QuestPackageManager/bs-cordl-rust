#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandom")]
#[repr(C)]
#[derive(Debug)]
pub struct SP800SecureRandom {
    __cordl_parent: crate::Org::BouncyCastle::Security::SecureRandom,
    pub mDrbgProvider: *mut crate::Org::BouncyCastle::Crypto::Prng::IDrbgProvider,
    pub mPredictionResistant: bool,
    pub mRandomSource: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    pub mEntropySource: *mut crate::Org::BouncyCastle::Crypto::IEntropySource,
    pub mDrbg: *mut crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandom")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Prng::SP800SecureRandom =>
    "Org.BouncyCastle.Crypto.Prng"."SP800SecureRandom"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandom")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandom {
    type Target = crate::Org::BouncyCastle::Security::SecureRandom;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandom")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandom {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandom")]
impl crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandom {
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
    pub fn _ctor(
        &mut self,
        randomSource: *mut crate::Org::BouncyCastle::Security::SecureRandom,
        entropySource: *mut crate::Org::BouncyCastle::Crypto::IEntropySource,
        drbgProvider: *mut crate::Org::BouncyCastle::Crypto::Prng::IDrbgProvider,
        predictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (randomSource, entropySource, drbgProvider, predictionResistant),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetSeed_Il2CppArray0(
        &mut self,
        seed: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSeed", (seed))?;
        Ok(__cordl_ret)
    }
    pub fn SetSeed_i64_1(
        &mut self,
        seed: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSeed", (seed))?;
        Ok(__cordl_ret)
    }
    pub fn NextBytes_Il2CppArray0(
        &mut self,
        bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NextBytes", (bytes))?;
        Ok(__cordl_ret)
    }
    pub fn NextBytes_i32_i32_1(
        &mut self,
        buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("NextBytes", (buf, off, len))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateSeed(
        &mut self,
        numBytes: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GenerateSeed", (numBytes))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        randomSource: *mut crate::Org::BouncyCastle::Security::SecureRandom,
        entropySource: *mut crate::Org::BouncyCastle::Crypto::IEntropySource,
        drbgProvider: *mut crate::Org::BouncyCastle::Crypto::Prng::IDrbgProvider,
        predictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (randomSource, entropySource, drbgProvider, predictionResistant),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandom")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandom {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
