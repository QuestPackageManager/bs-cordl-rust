#[cfg(feature = "Mono+Math+Prime+Generator+SequentialSearchPrimeGeneratorBase")]
#[repr(C)]
#[derive(Debug)]
pub struct SequentialSearchPrimeGeneratorBase {
    __cordl_parent: crate::Mono::Math::Prime::Generator::PrimeGeneratorBase,
}
#[cfg(feature = "Mono+Math+Prime+Generator+SequentialSearchPrimeGeneratorBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Math::Prime::Generator::SequentialSearchPrimeGeneratorBase =>
    "Mono.Math.Prime.Generator"."SequentialSearchPrimeGeneratorBase"
);
#[cfg(feature = "Mono+Math+Prime+Generator+SequentialSearchPrimeGeneratorBase")]
impl std::ops::Deref
for crate::Mono::Math::Prime::Generator::SequentialSearchPrimeGeneratorBase {
    type Target = crate::Mono::Math::Prime::Generator::PrimeGeneratorBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+Prime+Generator+SequentialSearchPrimeGeneratorBase")]
impl std::ops::DerefMut
for crate::Mono::Math::Prime::Generator::SequentialSearchPrimeGeneratorBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+Prime+Generator+SequentialSearchPrimeGeneratorBase")]
impl crate::Mono::Math::Prime::Generator::SequentialSearchPrimeGeneratorBase {
    pub fn GenerateNewPrime_Il2CppObject1(
        &mut self,
        bits: i32,
        context: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Mono::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Math::BigInteger = __cordl_object
            .invoke("GenerateNewPrime", (bits, context))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateNewPrime_i32_0(
        &mut self,
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Mono::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Math::BigInteger = __cordl_object
            .invoke("GenerateNewPrime", (bits))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateSearchBase(
        &mut self,
        bits: i32,
        context: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Mono::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Math::BigInteger = __cordl_object
            .invoke("GenerateSearchBase", (bits, context))?;
        Ok(__cordl_ret)
    }
    pub fn IsPrimeAcceptable(
        &mut self,
        bi: *mut crate::Mono::Math::BigInteger,
        context: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsPrimeAcceptable", (bi, context))?;
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
#[cfg(feature = "Mono+Math+Prime+Generator+SequentialSearchPrimeGeneratorBase")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Math::Prime::Generator::SequentialSearchPrimeGeneratorBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
