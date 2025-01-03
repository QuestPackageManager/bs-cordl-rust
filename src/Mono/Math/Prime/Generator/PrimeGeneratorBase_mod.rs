#[cfg(feature = "Mono+Math+Prime+Generator+PrimeGeneratorBase")]
#[repr(C)]
#[derive(Debug)]
pub struct PrimeGeneratorBase {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Math+Prime+Generator+PrimeGeneratorBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Math::Prime::Generator::PrimeGeneratorBase
    => "Mono.Math.Prime.Generator"."PrimeGeneratorBase"
);
#[cfg(feature = "Mono+Math+Prime+Generator+PrimeGeneratorBase")]
impl std::ops::Deref for crate::Mono::Math::Prime::Generator::PrimeGeneratorBase {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+Prime+Generator+PrimeGeneratorBase")]
impl std::ops::DerefMut for crate::Mono::Math::Prime::Generator::PrimeGeneratorBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+Prime+Generator+PrimeGeneratorBase")]
impl crate::Mono::Math::Prime::Generator::PrimeGeneratorBase {
    pub fn GenerateNewPrime(
        &mut self,
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger> = __cordl_object
            .invoke("GenerateNewPrime", (bits))?;
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
    pub fn get_Confidence(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Mono::Math::Prime::ConfidenceFactor> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Mono::Math::Prime::ConfidenceFactor = __cordl_object
            .invoke("get_Confidence", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PrimalityTest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Math::Prime::PrimalityTest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Math::Prime::PrimalityTest,
        > = __cordl_object.invoke("get_PrimalityTest", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TrialDivisionBounds(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_TrialDivisionBounds", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Math+Prime+Generator+PrimeGeneratorBase")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Math::Prime::Generator::PrimeGeneratorBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
