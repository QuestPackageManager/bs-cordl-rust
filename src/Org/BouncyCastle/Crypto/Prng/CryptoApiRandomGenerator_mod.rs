#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiRandomGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct CryptoApiRandomGenerator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub rndProv: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::RandomNumberGenerator,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiRandomGenerator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Prng::CryptoApiRandomGenerator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Prng";
    const CLASS_NAME: &'static str = "CryptoApiRandomGenerator";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiRandomGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Prng::CryptoApiRandomGenerator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiRandomGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Prng::CryptoApiRandomGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiRandomGenerator")]
impl crate::Org::BouncyCastle::Crypto::Prng::CryptoApiRandomGenerator {
    pub fn AddSeedMaterial_Il2CppArray0(
        &mut self,
        seed: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Prng::CryptoApiRandomGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddSeedMaterial")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Prng::CryptoApiRandomGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "AddSeedMaterial", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (seed))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn AddSeedMaterial_i64_1(
        &mut self,
        seed: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Prng::CryptoApiRandomGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i64), quest_hook::libil2cpp::Void, 1usize>("AddSeedMaterial")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Prng::CryptoApiRandomGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "AddSeedMaterial", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (seed))?
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
    pub fn New_RandomNumberGenerator1(
        rng: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RandomNumberGenerator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (rng))?;
        Ok(__cordl_object.into())
    }
    pub fn NextBytes_Il2CppArray0(
        &mut self,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Prng::CryptoApiRandomGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("NextBytes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Prng::CryptoApiRandomGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "NextBytes", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bytes))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn NextBytes_i32_i32_1(
        &mut self,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        start: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Prng::CryptoApiRandomGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("NextBytes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Prng::CryptoApiRandomGenerator as
                    quest_hook::libil2cpp::Type > ::class(), "NextBytes", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bytes, start, len))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Prng::CryptoApiRandomGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Prng::CryptoApiRandomGenerator as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_RandomNumberGenerator1(
        &mut self,
        rng: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::RandomNumberGenerator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Prng::CryptoApiRandomGenerator as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Security::Cryptography::RandomNumberGenerator,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Prng::CryptoApiRandomGenerator as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (rng))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiRandomGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Prng::CryptoApiRandomGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiRandomGenerator")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Prng::IRandomGenerator>
for crate::Org::BouncyCastle::Crypto::Prng::CryptoApiRandomGenerator {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Prng::IRandomGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Prng+CryptoApiRandomGenerator")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Prng::IRandomGenerator>
for crate::Org::BouncyCastle::Crypto::Prng::CryptoApiRandomGenerator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::Prng::IRandomGenerator {
        unsafe { std::mem::transmute(self) }
    }
}
