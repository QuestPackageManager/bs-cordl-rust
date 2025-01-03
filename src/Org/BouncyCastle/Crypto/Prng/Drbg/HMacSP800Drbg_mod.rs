#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+HMacSP800Drbg")]
#[repr(C)]
#[derive(Debug)]
pub struct HMacSP800Drbg {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mK: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub mV: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub mEntropySource: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IEntropySource,
    >,
    pub mHMac: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
    pub mSecurityStrength: i32,
    pub mReseedCounter: i64,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+HMacSP800Drbg")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Prng::Drbg::HMacSP800Drbg =>
    "Org.BouncyCastle.Crypto.Prng.Drbg"."HMacSP800Drbg"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+HMacSP800Drbg")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Prng::Drbg::HMacSP800Drbg {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+HMacSP800Drbg")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Prng::Drbg::HMacSP800Drbg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+HMacSP800Drbg")]
impl crate::Org::BouncyCastle::Crypto::Prng::Drbg::HMacSP800Drbg {
    pub fn Generate(
        &mut self,
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        additionalInput: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        predictionResistant: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Generate", (output, additionalInput, predictionResistant))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEntropy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("GetEntropy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        hMac: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
        securityStrength: i32,
        entropySource: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IEntropySource,
        >,
        personalizationString: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (hMac, securityStrength, entropySource, personalizationString, nonce),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn Reseed(
        &mut self,
        additionalInput: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reseed", (additionalInput))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        hMac: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
        securityStrength: i32,
        entropySource: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IEntropySource,
        >,
        personalizationString: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        nonce: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (hMac, securityStrength, entropySource, personalizationString, nonce),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_BlockSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn hmac_DRBG_Update(
        &mut self,
        seedMaterial: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("hmac_DRBG_Update", (seedMaterial))?;
        Ok(__cordl_ret.into())
    }
    pub fn hmac_DRBG_Update_Func(
        &mut self,
        seedMaterial: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        vValue: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("hmac_DRBG_Update_Func", (seedMaterial, vValue))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+HMacSP800Drbg")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Prng::Drbg::HMacSP800Drbg {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+HMacSP800Drbg")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg>
for crate::Org::BouncyCastle::Crypto::Prng::Drbg::HMacSP800Drbg {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+Drbg+HMacSP800Drbg")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg>
for crate::Org::BouncyCastle::Crypto::Prng::Drbg::HMacSP800Drbg {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::Prng::Drbg::ISP80090Drbg {
        unsafe { std::mem::transmute(self) }
    }
}
