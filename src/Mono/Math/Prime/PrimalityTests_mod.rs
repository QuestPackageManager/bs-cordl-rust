#[cfg(feature = "Mono+Math+Prime+PrimalityTests")]
#[repr(C)]
#[derive(Debug)]
pub struct PrimalityTests {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Mono+Math+Prime+PrimalityTests")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Math::Prime::PrimalityTests =>
    "Mono.Math.Prime"."PrimalityTests"
);
#[cfg(feature = "Mono+Math+Prime+PrimalityTests")]
impl std::ops::Deref for crate::Mono::Math::Prime::PrimalityTests {
    type Target = crate::System::Object;
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
impl crate::Mono::Math::Prime::PrimalityTests {}
#[cfg(feature = "Mono+Math+Prime+PrimalityTests")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Math::Prime::PrimalityTests {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}