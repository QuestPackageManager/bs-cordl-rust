#[cfg(feature = "UnityEngine+InputSystem+Utilities+NumberHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct NumberHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+NumberHelpers")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::Utilities::NumberHelpers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Utilities";
    const CLASS_NAME: &'static str = "NumberHelpers";
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
#[cfg(feature = "UnityEngine+InputSystem+Utilities+NumberHelpers")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::Utilities::NumberHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+NumberHelpers")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::Utilities::NumberHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+NumberHelpers")]
impl crate::UnityEngine::InputSystem::Utilities::NumberHelpers {
    pub fn AlignToMultipleOf_i32_i32_0(
        number: i32,
        alignment: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AlignToMultipleOf", (number, alignment))?;
        Ok(__cordl_ret.into())
    }
    pub fn AlignToMultipleOf_i64_i64_1(
        number: i64,
        alignment: i64,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AlignToMultipleOf", (number, alignment))?;
        Ok(__cordl_ret.into())
    }
    pub fn AlignToMultipleOf_u32_u32_2(
        number: u32,
        alignment: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AlignToMultipleOf", (number, alignment))?;
        Ok(__cordl_ret.into())
    }
    pub fn Approximately(a: f64, b: f64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Approximately", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn IntToNormalizedFloat(
        value: i32,
        minValue: i32,
        maxValue: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IntToNormalizedFloat", (value, minValue, maxValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn NormalizedFloatToInt(
        value: f32,
        intMinValue: i32,
        intMaxValue: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NormalizedFloatToInt", (value, intMinValue, intMaxValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn NormalizedFloatToUInt(
        value: f32,
        uintMinValue: u32,
        uintMaxValue: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NormalizedFloatToUInt", (value, uintMinValue, uintMaxValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemapUIntBitsToNormalizeFloatToUIntBits(
        value: u32,
        inBitSize: u32,
        outBitSize: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "RemapUIntBitsToNormalizeFloatToUIntBits",
                (value, inBitSize, outBitSize),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UIntToNormalizedFloat(
        value: u32,
        minValue: u32,
        maxValue: u32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("UIntToNormalizedFloat", (value, minValue, maxValue))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Utilities+NumberHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Utilities::NumberHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
