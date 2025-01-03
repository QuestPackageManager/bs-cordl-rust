#[cfg(feature = "Mono+Math+Prime+PrimalityTests")]
#[repr(C)]
#[derive(Debug)]
pub struct PrimalityTests {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Math+Prime+PrimalityTests")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Math::Prime::PrimalityTests =>
    "Mono.Math.Prime"."PrimalityTests"
);
#[cfg(feature = "Mono+Math+Prime+PrimalityTests")]
impl std::ops::Deref for crate::Mono::Math::Prime::PrimalityTests {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+Prime+PrimalityTests")]
impl std::ops::DerefMut for crate::Mono::Math::Prime::PrimalityTests {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Math+Prime+PrimalityTests")]
impl crate::Mono::Math::Prime::PrimalityTests {
    pub fn GetSPPRounds(
        bi: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        confidence: crate::Mono::Math::Prime::ConfidenceFactor,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSPPRounds", (bi, confidence))?;
        Ok(__cordl_ret.into())
    }
    pub fn RabinMillerTest(
        n: quest_hook::libil2cpp::Gc<crate::Mono::Math::BigInteger>,
        confidence: crate::Mono::Math::Prime::ConfidenceFactor,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RabinMillerTest", (n, confidence))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Math+Prime+PrimalityTests")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Math::Prime::PrimalityTests {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
