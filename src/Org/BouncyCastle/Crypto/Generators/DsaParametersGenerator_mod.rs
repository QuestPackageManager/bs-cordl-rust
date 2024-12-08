#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+DsaParametersGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct DsaParametersGenerator {
    __cordl_parent: crate::System::Object,
    pub digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    pub L: i32,
    pub N: i32,
    pub certainty: i32,
    pub random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    pub use186_3: bool,
    pub usageIndex: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+DsaParametersGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Generators::DsaParametersGenerator =>
    "Org.BouncyCastle.Crypto.Generators"."DsaParametersGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+DsaParametersGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Generators::DsaParametersGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+DsaParametersGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Generators::DsaParametersGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+DsaParametersGenerator")]
impl crate::Org::BouncyCastle::Crypto::Generators::DsaParametersGenerator {
    pub fn GenerateParameters_FIPS186_2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Parameters::DsaParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Parameters::DsaParameters = __cordl_object
            .invoke("GenerateParameters_FIPS186_2", ())?;
        Ok(__cordl_ret)
    }
    pub fn CalculateGenerator_FIPS186_3_Unverifiable(
        &mut self,
        p: *mut crate::Org::BouncyCastle::Math::BigInteger,
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        r: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("CalculateGenerator_FIPS186_3_Unverifiable", (p, q, r))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateGenerator_FIPS186_2(
        &mut self,
        p: *mut crate::Org::BouncyCastle::Math::BigInteger,
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        r: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("CalculateGenerator_FIPS186_2", (p, q, r))?;
        Ok(__cordl_ret)
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
    pub fn _ctor_IDigest1(
        &mut self,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (digest))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateGenerator_FIPS186_3_Verifiable(
        &mut self,
        d: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        p: *mut crate::Org::BouncyCastle::Math::BigInteger,
        q: *mut crate::Org::BouncyCastle::Math::BigInteger,
        seed: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("CalculateGenerator_FIPS186_3_Verifiable", (d, p, q, seed, index))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Parameters::DsaParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Parameters::DsaParameters = __cordl_object
            .invoke("GenerateParameters", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init_i32_i32_SecureRandom0(
        &mut self,
        _cordl_size: i32,
        certainty: i32,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (_cordl_size, certainty, random))?;
        Ok(__cordl_ret)
    }
    pub fn Init_DsaParameterGenerationParameters1(
        &mut self,
        parameters: *mut crate::Org::BouncyCastle::Crypto::Parameters::DsaParameterGenerationParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (parameters))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateParameters_FIPS186_3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Parameters::DsaParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Parameters::DsaParameters = __cordl_object
            .invoke("GenerateParameters_FIPS186_3", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_IDigest1(
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (digest))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+DsaParametersGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Generators::DsaParametersGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
