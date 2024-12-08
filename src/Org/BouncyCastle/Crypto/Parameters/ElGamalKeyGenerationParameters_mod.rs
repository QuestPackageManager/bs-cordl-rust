#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ElGamalKeyGenerationParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct ElGamalKeyGenerationParameters {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::KeyGenerationParameters,
    pub parameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::ElGamalParameters,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ElGamalKeyGenerationParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::ElGamalKeyGenerationParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."ElGamalKeyGenerationParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ElGamalKeyGenerationParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::ElGamalKeyGenerationParameters {
    type Target = crate::Org::BouncyCastle::Crypto::KeyGenerationParameters;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ElGamalKeyGenerationParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::ElGamalKeyGenerationParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ElGamalKeyGenerationParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::ElGamalKeyGenerationParameters {
    pub fn New(
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
        parameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::ElGamalParameters,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (random, parameters))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
        parameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::ElGamalParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (random, parameters))?;
        Ok(__cordl_ret)
    }
    pub fn get_Parameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Parameters::ElGamalParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Parameters::ElGamalParameters = __cordl_object
            .invoke("get_Parameters", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ElGamalKeyGenerationParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::ElGamalKeyGenerationParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}