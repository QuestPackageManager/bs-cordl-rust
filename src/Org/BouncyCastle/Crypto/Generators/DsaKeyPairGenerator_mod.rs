#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+DsaKeyPairGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct DsaKeyPairGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub param: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Parameters::DsaKeyGenerationParameters,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+DsaKeyPairGenerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Generators::DsaKeyPairGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Generators";
    const CLASS_NAME: &'static str = "DsaKeyPairGenerator";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+DsaKeyPairGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Generators::DsaKeyPairGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+DsaKeyPairGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Generators::DsaKeyPairGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+DsaKeyPairGenerator")]
impl crate::Org::BouncyCastle::Crypto::Generators::DsaKeyPairGenerator {
    pub fn CalculatePublicKey(
        p: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        x: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculatePublicKey", (p, g, x))?;
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
    pub fn GeneratePrivateKey(
        q: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GeneratePrivateKey", (q, random))?;
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+DsaKeyPairGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Generators::DsaKeyPairGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+DsaKeyPairGenerator")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IAsymmetricCipherKeyPairGenerator>
for crate::Org::BouncyCastle::Crypto::Generators::DsaKeyPairGenerator {
    fn as_ref(
        &self,
    ) -> &crate::Org::BouncyCastle::Crypto::IAsymmetricCipherKeyPairGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+DsaKeyPairGenerator")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IAsymmetricCipherKeyPairGenerator>
for crate::Org::BouncyCastle::Crypto::Generators::DsaKeyPairGenerator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::IAsymmetricCipherKeyPairGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
