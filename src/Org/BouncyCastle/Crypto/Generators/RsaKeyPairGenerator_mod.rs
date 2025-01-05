#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+RsaKeyPairGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct RsaKeyPairGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub parameters: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Parameters::RsaKeyGenerationParameters,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+RsaKeyPairGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Generators::RsaKeyPairGenerator =>
    "Org.BouncyCastle.Crypto.Generators"."RsaKeyPairGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+RsaKeyPairGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Generators::RsaKeyPairGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+RsaKeyPairGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Generators::RsaKeyPairGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+RsaKeyPairGenerator")]
impl crate::Org::BouncyCastle::Crypto::Generators::RsaKeyPairGenerator {
    pub const DefaultTests: i32 = 100i32;
    pub fn ChooseRandomPrime(
        &mut self,
        bitlength: i32,
        e: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("ChooseRandomPrime", (bitlength, e))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateKeyPair(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        > = __cordl_object.invoke("GenerateKeyPair", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::KeyGenerationParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+RsaKeyPairGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Generators::RsaKeyPairGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+RsaKeyPairGenerator")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IAsymmetricCipherKeyPairGenerator>
for crate::Org::BouncyCastle::Crypto::Generators::RsaKeyPairGenerator {
    fn as_ref(
        &self,
    ) -> &crate::Org::BouncyCastle::Crypto::IAsymmetricCipherKeyPairGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+RsaKeyPairGenerator")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IAsymmetricCipherKeyPairGenerator>
for crate::Org::BouncyCastle::Crypto::Generators::RsaKeyPairGenerator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::IAsymmetricCipherKeyPairGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
