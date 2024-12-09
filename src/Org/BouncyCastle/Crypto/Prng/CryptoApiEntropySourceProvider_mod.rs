#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiEntropySourceProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct CryptoApiEntropySourceProvider {
    __cordl_parent: crate::System::Object,
    pub mRng: *mut crate::System::Security::Cryptography::RandomNumberGenerator,
    pub mPredictionResistant: bool,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiEntropySourceProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Prng::CryptoApiEntropySourceProvider =>
    "Org.BouncyCastle.Crypto.Prng"."CryptoApiEntropySourceProvider"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiEntropySourceProvider")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Prng::CryptoApiEntropySourceProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiEntropySourceProvider")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Prng::CryptoApiEntropySourceProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiEntropySourceProvider")]
impl crate::Org::BouncyCastle::Crypto::Prng::CryptoApiEntropySourceProvider {
    #[cfg(
        feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiEntropySourceProvider+CryptoApiEntropySource"
    )]
    pub type CryptoApiEntropySource = crate::Org::BouncyCastle::Crypto::Prng::CryptoApiEntropySourceProvider_CryptoApiEntropySource;
    pub fn Get(
        &mut self,
        bitsRequired: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::IEntropySource,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IEntropySource = __cordl_object
            .invoke("Get", (bitsRequired))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_RandomNumberGenerator__cordl_bool1(
        rng: *mut crate::System::Security::Cryptography::RandomNumberGenerator,
        isPredictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rng, isPredictionResistant))?;
        Ok(__cordl_object)
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
    pub fn _ctor_RandomNumberGenerator__cordl_bool1(
        &mut self,
        rng: *mut crate::System::Security::Cryptography::RandomNumberGenerator,
        isPredictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rng, isPredictionResistant))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiEntropySourceProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Prng::CryptoApiEntropySourceProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiEntropySourceProvider+CryptoApiEntropySource"
)]
#[repr(C)]
#[derive(Debug)]
pub struct CryptoApiEntropySourceProvider_CryptoApiEntropySource {
    __cordl_parent: crate::System::Object,
    pub mRng: *mut crate::System::Security::Cryptography::RandomNumberGenerator,
    pub mPredictionResistant: bool,
    pub mEntropySize: i32,
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiEntropySourceProvider+CryptoApiEntropySource"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Prng::CryptoApiEntropySourceProvider_CryptoApiEntropySource
    => "Org.BouncyCastle.Crypto.Prng"
    ."CryptoApiEntropySourceProvider/CryptoApiEntropySource"
);
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiEntropySourceProvider+CryptoApiEntropySource"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Prng::CryptoApiEntropySourceProvider_CryptoApiEntropySource {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiEntropySourceProvider+CryptoApiEntropySource"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Prng::CryptoApiEntropySourceProvider_CryptoApiEntropySource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiEntropySourceProvider+CryptoApiEntropySource"
)]
impl crate::Org::BouncyCastle::Crypto::Prng::CryptoApiEntropySourceProvider_CryptoApiEntropySource {
    pub fn New(
        rng: *mut crate::System::Security::Cryptography::RandomNumberGenerator,
        predictionResistant: bool,
        entropySize: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rng, predictionResistant, entropySize))?;
        Ok(__cordl_object)
    }
    pub fn Org_BouncyCastle_Crypto_IEntropySource_GetEntropy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Org.BouncyCastle.Crypto.IEntropySource.GetEntropy", ())?;
        Ok(__cordl_ret)
    }
    pub fn Org_BouncyCastle_Crypto_IEntropySource_get_EntropySize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Org.BouncyCastle.Crypto.IEntropySource.get_EntropySize", ())?;
        Ok(__cordl_ret)
    }
    pub fn Org_BouncyCastle_Crypto_IEntropySource_get_IsPredictionResistant(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "Org.BouncyCastle.Crypto.IEntropySource.get_IsPredictionResistant",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        rng: *mut crate::System::Security::Cryptography::RandomNumberGenerator,
        predictionResistant: bool,
        entropySize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (rng, predictionResistant, entropySize))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiEntropySourceProvider+CryptoApiEntropySource"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Prng::CryptoApiEntropySourceProvider_CryptoApiEntropySource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
