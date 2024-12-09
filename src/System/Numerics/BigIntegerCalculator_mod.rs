#[cfg(feature = "System+Numerics+BigIntegerCalculator")]
#[repr(C)]
#[derive(Debug)]
pub struct BigIntegerCalculator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Numerics+BigIntegerCalculator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Numerics::BigIntegerCalculator =>
    "System.Numerics"."BigIntegerCalculator"
);
#[cfg(feature = "System+Numerics+BigIntegerCalculator")]
impl std::ops::Deref for crate::System::Numerics::BigIntegerCalculator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Numerics+BigIntegerCalculator")]
impl std::ops::DerefMut for crate::System::Numerics::BigIntegerCalculator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Numerics+BigIntegerCalculator")]
impl crate::System::Numerics::BigIntegerCalculator {}
#[cfg(feature = "System+Numerics+BigIntegerCalculator")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Numerics::BigIntegerCalculator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
