#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+CtrDrbgProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct SP800SecureRandomBuilder_CtrDrbgProvider {
    __cordl_parent: crate::System::Object,
    pub mBlockCipher: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
    pub mKeySizeInBits: i32,
    pub mNonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mPersonalizationString: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mSecurityStrength: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+CtrDrbgProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_CtrDrbgProvider =>
    "Org.BouncyCastle.Crypto.Prng"."SP800SecureRandomBuilder/CtrDrbgProvider"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+CtrDrbgProvider")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_CtrDrbgProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+CtrDrbgProvider")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_CtrDrbgProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+CtrDrbgProvider")]
impl crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_CtrDrbgProvider {
    pub fn Get(
        &mut self,
        entropySource: *mut crate::Org::BouncyCastle::Crypto::IEntropySource,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg = __cordl_object
            .invoke("Get", (entropySource))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        blockCipher: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
        keySizeInBits: i32,
        nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        personalizationString: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        securityStrength: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    blockCipher,
                    keySizeInBits,
                    nonce,
                    personalizationString,
                    securityStrength,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        blockCipher: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
        keySizeInBits: i32,
        nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        personalizationString: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        securityStrength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    blockCipher,
                    keySizeInBits,
                    nonce,
                    personalizationString,
                    securityStrength,
                ),
            )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+CtrDrbgProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_CtrDrbgProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HMacDrbgProvider"
)]
#[repr(C)]
#[derive(Debug)]
pub struct SP800SecureRandomBuilder_HMacDrbgProvider {
    __cordl_parent: crate::System::Object,
    pub mHMac: *mut crate::Org::BouncyCastle::Crypto::IMac,
    pub mNonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mPersonalizationString: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mSecurityStrength: i32,
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HMacDrbgProvider"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HMacDrbgProvider =>
    "Org.BouncyCastle.Crypto.Prng"."SP800SecureRandomBuilder/HMacDrbgProvider"
);
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HMacDrbgProvider"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HMacDrbgProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HMacDrbgProvider"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HMacDrbgProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HMacDrbgProvider"
)]
impl crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HMacDrbgProvider {
    pub fn Get(
        &mut self,
        entropySource: *mut crate::Org::BouncyCastle::Crypto::IEntropySource,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg = __cordl_object
            .invoke("Get", (entropySource))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        hMac: *mut crate::Org::BouncyCastle::Crypto::IMac,
        nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        personalizationString: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        securityStrength: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (hMac, nonce, personalizationString, securityStrength),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        hMac: *mut crate::Org::BouncyCastle::Crypto::IMac,
        nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        personalizationString: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        securityStrength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (hMac, nonce, personalizationString, securityStrength))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HMacDrbgProvider"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HMacDrbgProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HashDrbgProvider"
)]
#[repr(C)]
#[derive(Debug)]
pub struct SP800SecureRandomBuilder_HashDrbgProvider {
    __cordl_parent: crate::System::Object,
    pub mDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    pub mNonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mPersonalizationString: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mSecurityStrength: i32,
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HashDrbgProvider"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HashDrbgProvider =>
    "Org.BouncyCastle.Crypto.Prng"."SP800SecureRandomBuilder/HashDrbgProvider"
);
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HashDrbgProvider"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HashDrbgProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HashDrbgProvider"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HashDrbgProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HashDrbgProvider"
)]
impl crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HashDrbgProvider {
    pub fn Get(
        &mut self,
        entropySource: *mut crate::Org::BouncyCastle::Crypto::IEntropySource,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg = __cordl_object
            .invoke("Get", (entropySource))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        personalizationString: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        securityStrength: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (digest, nonce, personalizationString, securityStrength),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        personalizationString: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        securityStrength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (digest, nonce, personalizationString, securityStrength))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HashDrbgProvider"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HashDrbgProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct SP800SecureRandomBuilder {
    __cordl_parent: crate::System::Object,
    pub mRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    pub mEntropySourceProvider: *mut crate::Org::BouncyCastle::Crypto::IEntropySourceProvider,
    pub mPersonalizationString: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mSecurityStrength: i32,
    pub mEntropyBitsRequired: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder =>
    "Org.BouncyCastle.Crypto.Prng"."SP800SecureRandomBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder")]
impl crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder {
    #[cfg(
        feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+CtrDrbgProvider"
    )]
    pub type CtrDrbgProvider = crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_CtrDrbgProvider;
    #[cfg(
        feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HashDrbgProvider"
    )]
    pub type HashDrbgProvider = crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HashDrbgProvider;
    #[cfg(
        feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HMacDrbgProvider"
    )]
    pub type HMacDrbgProvider = crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HMacDrbgProvider;
    pub fn BuildCtr(
        &mut self,
        cipher: *mut crate::Org::BouncyCastle::Crypto::IBlockCipher,
        keySizeInBits: i32,
        nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        predictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandom,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandom = __cordl_object
            .invoke("BuildCtr", (cipher, keySizeInBits, nonce, predictionResistant))?;
        Ok(__cordl_ret)
    }
    pub fn BuildHMac(
        &mut self,
        hMac: *mut crate::Org::BouncyCastle::Crypto::IMac,
        nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        predictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandom,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandom = __cordl_object
            .invoke("BuildHMac", (hMac, nonce, predictionResistant))?;
        Ok(__cordl_ret)
    }
    pub fn BuildHash(
        &mut self,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        nonce: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        predictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandom,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandom = __cordl_object
            .invoke("BuildHash", (digest, nonce, predictionResistant))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_IEntropySourceProvider2(
        entropySourceProvider: *mut crate::Org::BouncyCastle::Crypto::IEntropySourceProvider,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (entropySourceProvider))?;
        Ok(__cordl_object)
    }
    pub fn New_SecureRandom__cordl_bool1(
        entropySource: *mut crate::Org::BouncyCastle::Security::SecureRandom,
        predictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (entropySource, predictionResistant))?;
        Ok(__cordl_object)
    }
    pub fn SetEntropyBitsRequired(
        &mut self,
        entropyBitsRequired: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder = __cordl_object
            .invoke("SetEntropyBitsRequired", (entropyBitsRequired))?;
        Ok(__cordl_ret)
    }
    pub fn SetPersonalizationString(
        &mut self,
        personalizationString: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder = __cordl_object
            .invoke("SetPersonalizationString", (personalizationString))?;
        Ok(__cordl_ret)
    }
    pub fn SetSecurityStrength(
        &mut self,
        securityStrength: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder = __cordl_object
            .invoke("SetSecurityStrength", (securityStrength))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IEntropySourceProvider2(
        &mut self,
        entropySourceProvider: *mut crate::Org::BouncyCastle::Crypto::IEntropySourceProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (entropySourceProvider))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SecureRandom__cordl_bool1(
        &mut self,
        entropySource: *mut crate::Org::BouncyCastle::Security::SecureRandom,
        predictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (entropySource, predictionResistant))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}