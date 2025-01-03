#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DsaParameterGenerationParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct DsaParameterGenerationParameters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub l: i32,
    pub n: i32,
    pub certainty: i32,
    pub random: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Security::SecureRandom,
    >,
    pub usageIndex: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DsaParameterGenerationParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::DsaParameterGenerationParameters =>
    "Org.BouncyCastle.Crypto.Parameters"."DsaParameterGenerationParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DsaParameterGenerationParameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::DsaParameterGenerationParameters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DsaParameterGenerationParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::DsaParameterGenerationParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DsaParameterGenerationParameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::DsaParameterGenerationParameters {
    pub const DigitalSignatureUsage: i32 = 1i32;
    pub const KeyEstablishmentUsage: i32 = 2i32;
    pub fn New_i32_1(
        L: i32,
        N: i32,
        certainty: i32,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        usageIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (L, N, certainty, random, usageIndex))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_i32_i32_SecureRandom0(
        L: i32,
        N: i32,
        certainty: i32,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (L, N, certainty, random))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        L: i32,
        N: i32,
        certainty: i32,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
        usageIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (L, N, certainty, random, usageIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_i32_SecureRandom0(
        &mut self,
        L: i32,
        N: i32,
        certainty: i32,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (L, N, certainty, random))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Certainty(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Certainty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_L(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_L", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_N(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_N", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Random(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Security::SecureRandom>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        > = __cordl_object.invoke("get_Random", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UsageIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_UsageIndex", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+DsaParameterGenerationParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::DsaParameterGenerationParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
