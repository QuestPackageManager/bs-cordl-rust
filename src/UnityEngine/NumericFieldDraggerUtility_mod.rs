#[cfg(feature = "UnityEngine+NumericFieldDraggerUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct NumericFieldDraggerUtility {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+NumericFieldDraggerUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::NumericFieldDraggerUtility =>
    "UnityEngine"."NumericFieldDraggerUtility"
);
#[cfg(feature = "UnityEngine+NumericFieldDraggerUtility")]
impl std::ops::Deref for crate::UnityEngine::NumericFieldDraggerUtility {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+NumericFieldDraggerUtility")]
impl std::ops::DerefMut for crate::UnityEngine::NumericFieldDraggerUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+NumericFieldDraggerUtility")]
impl crate::UnityEngine::NumericFieldDraggerUtility {
    pub fn Acceleration(
        shiftPressed: bool,
        altPressed: bool,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Acceleration", (shiftPressed, altPressed))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateFloatDragSensitivity_f64_0(
        value: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateFloatDragSensitivity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateFloatDragSensitivity_f64_f64_1(
        value: f64,
        minValue: f64,
        maxValue: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateFloatDragSensitivity", (value, minValue, maxValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateIntDragSensitivity_f64_2(
        value: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateIntDragSensitivity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateIntDragSensitivity_i64_0(
        value: i64,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateIntDragSensitivity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateIntDragSensitivity_i64_i64_i64_3(
        value: i64,
        minValue: i64,
        maxValue: i64,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateIntDragSensitivity", (value, minValue, maxValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateIntDragSensitivity_u64_1(
        value: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateIntDragSensitivity", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn NiceDelta(
        deviceDelta: crate::UnityEngine::Vector2,
        acceleration: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NiceDelta", (deviceDelta, acceleration))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+NumericFieldDraggerUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::NumericFieldDraggerUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
