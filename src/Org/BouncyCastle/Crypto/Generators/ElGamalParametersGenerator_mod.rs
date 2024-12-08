#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+ElGamalParametersGenerator")]
#[repr(C)]
#[derive(Debug)]
pub struct ElGamalParametersGenerator {
    __cordl_parent: crate::System::Object,
    pub _cordl_size: i32,
    pub certainty: i32,
    pub random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+ElGamalParametersGenerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Generators::ElGamalParametersGenerator =>
    "Org.BouncyCastle.Crypto.Generators"."ElGamalParametersGenerator"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+ElGamalParametersGenerator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Generators::ElGamalParametersGenerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+ElGamalParametersGenerator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Generators::ElGamalParametersGenerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+ElGamalParametersGenerator")]
impl crate::Org::BouncyCastle::Crypto::Generators::ElGamalParametersGenerator {
    pub fn GenerateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::Parameters::ElGamalParameters,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::Parameters::ElGamalParameters = __cordl_object
            .invoke("GenerateParameters", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Generators+ElGamalParametersGenerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Generators::ElGamalParametersGenerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}