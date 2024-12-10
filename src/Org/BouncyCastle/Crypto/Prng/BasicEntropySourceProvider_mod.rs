#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct BasicEntropySourceProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mSecureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    pub mPredictionResistant: bool,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider =>
    "Org.BouncyCastle.Crypto.Prng"."BasicEntropySourceProvider"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider")]
impl crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider {
    #[cfg(
        feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider+BasicEntropySource"
    )]
    pub type BasicEntropySource = crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider_BasicEntropySource;
    pub fn Get(
        &mut self,
        bitsRequired: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IEntropySource>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IEntropySource,
        > = __cordl_object.invoke("Get", (bitsRequired))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        secureRandom: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        isPredictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (secureRandom, isPredictionResistant))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        secureRandom: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        isPredictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (secureRandom, isPredictionResistant))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IEntropySourceProvider>
for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IEntropySourceProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IEntropySourceProvider>
for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::IEntropySourceProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider+BasicEntropySource"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BasicEntropySourceProvider_BasicEntropySource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mSecureRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    pub mPredictionResistant: bool,
    pub mEntropySize: i32,
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider+BasicEntropySource"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider_BasicEntropySource =>
    "Org.BouncyCastle.Crypto.Prng"."BasicEntropySourceProvider/BasicEntropySource"
);
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider+BasicEntropySource"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider_BasicEntropySource {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider+BasicEntropySource"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider_BasicEntropySource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider+BasicEntropySource"
)]
impl crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider_BasicEntropySource {
    pub fn New(
        secureRandom: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        predictionResistant: bool,
        entropySize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (secureRandom, predictionResistant, entropySize))?;
        Ok(__cordl_object.into())
    }
    pub fn Org_BouncyCastle_Crypto_IEntropySource_GetEntropy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object
            .invoke("Org.BouncyCastle.Crypto.IEntropySource.GetEntropy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Org_BouncyCastle_Crypto_IEntropySource_get_EntropySize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Org.BouncyCastle.Crypto.IEntropySource.get_EntropySize", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        secureRandom: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        predictionResistant: bool,
        entropySize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (secureRandom, predictionResistant, entropySize))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider+BasicEntropySource"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider_BasicEntropySource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider+BasicEntropySource"
)]
impl AsRef<crate::Org::BouncyCastle::Crypto::IEntropySource>
for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider_BasicEntropySource {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IEntropySource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Prng+BasicEntropySourceProvider+BasicEntropySource"
)]
impl AsMut<crate::Org::BouncyCastle::Crypto::IEntropySource>
for crate::Org::BouncyCastle::Crypto::Prng::BasicEntropySourceProvider_BasicEntropySource {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IEntropySource {
        unsafe { std::mem::transmute(self) }
    }
}
