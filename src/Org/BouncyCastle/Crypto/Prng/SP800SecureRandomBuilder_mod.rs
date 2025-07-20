#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct SP800SecureRandomBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mRandom: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Security::SecureRandom,
    >,
    pub mEntropySourceProvider: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IEntropySourceProvider,
    >,
    pub mPersonalizationString: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub mSecurityStrength: i32,
    pub mEntropyBitsRequired: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Prng";
    const CLASS_NAME: &'static str = "SP800SecureRandomBuilder";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HMacDrbgProvider"
    )]
    pub type HMacDrbgProvider = crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HMacDrbgProvider;
    #[cfg(
        feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HashDrbgProvider"
    )]
    pub type HashDrbgProvider = crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HashDrbgProvider;
    pub fn BuildCtr(
        &mut self,
        cipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBlockCipher,
        >,
        keySizeInBits: i32,
        nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        predictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandom,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::IBlockCipher,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandom,
                        >,
                        4usize,
                    >("BuildCtr")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildCtr", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandom,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (cipher, keySizeInBits, nonce, predictionResistant),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BuildHMac(
        &mut self,
        hMac: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
        nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        predictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandom,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::IMac,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandom,
                        >,
                        3usize,
                    >("BuildHMac")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildHMac", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandom,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (hMac, nonce, predictionResistant))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BuildHash(
        &mut self,
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        predictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandom,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::IDigest,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandom,
                        >,
                        3usize,
                    >("BuildHash")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildHash", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandom,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (digest, nonce, predictionResistant))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_IEntropySourceProvider2(
        entropySourceProvider: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IEntropySourceProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (entropySourceProvider))?;
        Ok(__cordl_object.into())
    }
    pub fn New_SecureRandom__cordl_bool1(
        entropySource: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        predictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (entropySource, predictionResistant))?;
        Ok(__cordl_object.into())
    }
    pub fn SetEntropyBitsRequired(
        &mut self,
        entropyBitsRequired: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder,
                        >,
                        1usize,
                    >("SetEntropyBitsRequired")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetEntropyBitsRequired", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (entropyBitsRequired))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetPersonalizationString(
        &mut self,
        personalizationString: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<u8>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder,
                        >,
                        1usize,
                    >("SetPersonalizationString")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetPersonalizationString", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (personalizationString))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetSecurityStrength(
        &mut self,
        securityStrength: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder,
                        >,
                        1usize,
                    >("SetSecurityStrength")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetSecurityStrength", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (securityStrength))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IEntropySourceProvider2(
        &mut self,
        entropySourceProvider: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IEntropySourceProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::IEntropySourceProvider,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (entropySourceProvider))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SecureRandom__cordl_bool1(
        &mut self,
        entropySource: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        predictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Security::SecureRandom,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (entropySource, predictionResistant))?
        };
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+CtrDrbgProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct SP800SecureRandomBuilder_CtrDrbgProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mBlockCipher: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IBlockCipher,
    >,
    pub mKeySizeInBits: i32,
    pub mNonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub mPersonalizationString: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub mSecurityStrength: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+CtrDrbgProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_CtrDrbgProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Prng";
    const CLASS_NAME: &'static str = "SP800SecureRandomBuilder/CtrDrbgProvider";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+CtrDrbgProvider")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_CtrDrbgProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+CtrDrbgProvider")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_CtrDrbgProvider {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+CtrDrbgProvider")]
impl crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_CtrDrbgProvider {
    pub fn Get(
        &mut self,
        entropySource: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IEntropySource,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::IEntropySource,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg,
                        >,
                        1usize,
                    >("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Get",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (entropySource))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        blockCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBlockCipher,
        >,
        keySizeInBits: i32,
        nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        personalizationString: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        securityStrength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        blockCipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBlockCipher,
        >,
        keySizeInBits: i32,
        nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        personalizationString: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        securityStrength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::IBlockCipher,
                            >,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        blockCipher,
                        keySizeInBits,
                        nonce,
                        personalizationString,
                        securityStrength,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+CtrDrbgProvider")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Prng::IDrbgProvider>
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_CtrDrbgProvider {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Prng::IDrbgProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+CtrDrbgProvider")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Prng::IDrbgProvider>
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_CtrDrbgProvider {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::Prng::IDrbgProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HMacDrbgProvider"
)]
#[repr(C)]
#[derive(Debug)]
pub struct SP800SecureRandomBuilder_HMacDrbgProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mHMac: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
    pub mNonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub mPersonalizationString: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub mSecurityStrength: i32,
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HMacDrbgProvider"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HMacDrbgProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Prng";
    const CLASS_NAME: &'static str = "SP800SecureRandomBuilder/HMacDrbgProvider";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HMacDrbgProvider"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HMacDrbgProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HMacDrbgProvider"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HMacDrbgProvider {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HMacDrbgProvider"
)]
impl crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HMacDrbgProvider {
    pub fn Get(
        &mut self,
        entropySource: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IEntropySource,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::IEntropySource,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg,
                        >,
                        1usize,
                    >("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Get",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (entropySource))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        hMac: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
        nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        personalizationString: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        securityStrength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (hMac, nonce, personalizationString, securityStrength),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        hMac: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
        nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        personalizationString: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        securityStrength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::IMac,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (hMac, nonce, personalizationString, securityStrength),
                )?
        };
        Ok(__cordl_ret.into())
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
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HMacDrbgProvider"
)]
impl AsRef<crate::Org::BouncyCastle::Crypto::Prng::IDrbgProvider>
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HMacDrbgProvider {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Prng::IDrbgProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HMacDrbgProvider"
)]
impl AsMut<crate::Org::BouncyCastle::Crypto::Prng::IDrbgProvider>
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HMacDrbgProvider {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::Prng::IDrbgProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HashDrbgProvider"
)]
#[repr(C)]
#[derive(Debug)]
pub struct SP800SecureRandomBuilder_HashDrbgProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mDigest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
    pub mNonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub mPersonalizationString: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub mSecurityStrength: i32,
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HashDrbgProvider"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HashDrbgProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Prng";
    const CLASS_NAME: &'static str = "SP800SecureRandomBuilder/HashDrbgProvider";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HashDrbgProvider"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HashDrbgProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HashDrbgProvider"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HashDrbgProvider {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HashDrbgProvider"
)]
impl crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HashDrbgProvider {
    pub fn Get(
        &mut self,
        entropySource: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IEntropySource,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::IEntropySource,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg,
                        >,
                        1usize,
                    >("Get")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Get",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (entropySource))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        personalizationString: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        securityStrength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (digest, nonce, personalizationString, securityStrength),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        personalizationString: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        securityStrength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Crypto::IDigest,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (digest, nonce, personalizationString, securityStrength),
                )?
        };
        Ok(__cordl_ret.into())
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
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HashDrbgProvider"
)]
impl AsRef<crate::Org::BouncyCastle::Crypto::Prng::IDrbgProvider>
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HashDrbgProvider {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Prng::IDrbgProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+SP800SecureRandomBuilder+HashDrbgProvider"
)]
impl AsMut<crate::Org::BouncyCastle::Crypto::Prng::IDrbgProvider>
for crate::Org::BouncyCastle::Crypto::Prng::SP800SecureRandomBuilder_HashDrbgProvider {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::Prng::IDrbgProvider {
        unsafe { std::mem::transmute(self) }
    }
}
