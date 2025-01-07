#[cfg(
    feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternKeyGenerationParameters"
)]
#[repr(C)]
#[derive(Debug)]
pub struct NaccacheSternKeyGenerationParameters {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::KeyGenerationParameters,
    pub certainty: i32,
    pub countSmallPrimes: i32,
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternKeyGenerationParameters"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternKeyGenerationParameters {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Parameters";
    const CLASS_NAME: &'static str = "NaccacheSternKeyGenerationParameters";
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
    feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternKeyGenerationParameters"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternKeyGenerationParameters {
    type Target = crate::Org::BouncyCastle::Crypto::KeyGenerationParameters;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternKeyGenerationParameters"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternKeyGenerationParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternKeyGenerationParameters"
)]
impl crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternKeyGenerationParameters {
    pub fn New_SecureRandom_i32_i32_i32_0(
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        strength: i32,
        certainty: i32,
        countSmallPrimes: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (random, strength, certainty, countSmallPrimes))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool1(
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        strength: i32,
        certainty: i32,
        countSmallPrimes: i32,
        debug: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (random, strength, certainty, countSmallPrimes, debug),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_SecureRandom_i32_i32_i32_0(
        &mut self,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        strength: i32,
        certainty: i32,
        countSmallPrimes: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (random, strength, certainty, countSmallPrimes))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        strength: i32,
        certainty: i32,
        countSmallPrimes: i32,
        debug: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (random, strength, certainty, countSmallPrimes, debug))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Certainty(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Certainty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CountSmallPrimes(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_CountSmallPrimes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDebug(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDebug", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Crypto+Parameters+NaccacheSternKeyGenerationParameters"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::NaccacheSternKeyGenerationParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
