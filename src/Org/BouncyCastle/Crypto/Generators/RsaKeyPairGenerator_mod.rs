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
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Generators::RsaKeyPairGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Generators";
    const CLASS_NAME: &'static str = "RsaKeyPairGenerator";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
                ),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
                2usize,
            >("ChooseRandomPrime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ChooseRandomPrime", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = unsafe { method.invoke_unchecked(self, (bitlength, e)) };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateKeyPair(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
                >,
                0usize,
            >("GenerateKeyPair")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GenerateKeyPair", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::KeyGenerationParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Crypto::KeyGenerationParameters,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Init", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (parameters))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
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
