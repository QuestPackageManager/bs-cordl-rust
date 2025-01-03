#[cfg(feature = "UnityEngine+Mathf")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Mathf {}
#[cfg(feature = "UnityEngine+Mathf")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Mathf => "UnityEngine"."Mathf"
);
#[cfg(feature = "UnityEngine+Mathf")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Mathf {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Mathf")]
impl crate::UnityEngine::Mathf {
    pub fn Abs_f32_0(f: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Abs", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn Abs_i32_1(value: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Abs", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Acos(f: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Acos", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn Approximately(a: f32, b: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Approximately", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Asin(f: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Asin", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn Atan(f: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Atan", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn Atan2(y: f32, x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Atan2", (y, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn Ceil(f: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Ceil", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn CeilToInt(f: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CeilToInt", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clamp01(value: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Clamp01", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClampToFloat(value: f64) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClampToFloat", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClampToInt(value: i64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClampToInt", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClampToUInt(value: i64) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClampToUInt", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clamp_f32_f32_f32_0(
        value: f32,
        min: f32,
        max: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Clamp", (value, min, max))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clamp_i32_i32_i32_1(
        value: i32,
        min: i32,
        max: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Clamp", (value, min, max))?;
        Ok(__cordl_ret.into())
    }
    pub fn CorrelatedColorTemperatureToRGB(
        kelvin: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_ret: crate::UnityEngine::Color = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CorrelatedColorTemperatureToRGB", (kelvin))?;
        Ok(__cordl_ret.into())
    }
    pub fn CorrelatedColorTemperatureToRGB_Injected(
        kelvin: f32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CorrelatedColorTemperatureToRGB_Injected", (kelvin, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn Cos(f: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Cos", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeltaAngle(current: f32, target: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeltaAngle", (current, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn DiscardLeastSignificantDecimal(v: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DiscardLeastSignificantDecimal", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn Floor(f: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Floor", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn FloorToInt(f: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FloorToInt", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn GammaToLinearSpace(value: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GammaToLinearSpace", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNumberOfDecimalsForMinimumDifference(
        minDifference: f64,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNumberOfDecimalsForMinimumDifference", (minDifference))?;
        Ok(__cordl_ret.into())
    }
    pub fn InverseLerp(
        a: f32,
        b: f32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InverseLerp", (a, b, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPowerOfTwo(value: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPowerOfTwo", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lerp(a: f32, b: f32, t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lerp", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn LerpUnclamped(a: f32, b: f32, t: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LerpUnclamped", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn LinearToGammaSpace(value: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LinearToGammaSpace", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Log10(f: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Log10", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn Log_f32_0(f: f32, p: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Log", (f, p))?;
        Ok(__cordl_ret.into())
    }
    pub fn Log_f32_1(f: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Log", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_Il2CppArray1(
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (values))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_f32_f32_0(a: f32, b: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max_i32_i32_2(a: i32, b: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_Il2CppArray1(
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (values))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_f32_f32_0(a: f32, b: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min_i32_i32_2(a: i32, b: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveTowards(
        current: f32,
        target: f32,
        maxDelta: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MoveTowards", (current, target, maxDelta))?;
        Ok(__cordl_ret.into())
    }
    pub fn NextPowerOfTwo(value: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NextPowerOfTwo", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Pow(f: f32, p: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Pow", (f, p))?;
        Ok(__cordl_ret.into())
    }
    pub fn Repeat(t: f32, length: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Repeat", (t, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Round(f: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Round", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn RoundBasedOnMinimumDifference(
        valueToRound: f64,
        minDifference: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RoundBasedOnMinimumDifference", (valueToRound, minDifference))?;
        Ok(__cordl_ret.into())
    }
    pub fn RoundToInt(f: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RoundToInt", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sign(f: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sign", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sin(f: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sin", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn SmoothDamp(
        current: f32,
        target: f32,
        currentVelocity: quest_hook::libil2cpp::ByRefMut<f32>,
        smoothTime: f32,
        maxSpeed: f32,
        deltaTime: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SmoothDamp",
                (current, target, currentVelocity, smoothTime, maxSpeed, deltaTime),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Sqrt(f: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sqrt", (f))?;
        Ok(__cordl_ret.into())
    }
    pub fn Tan(f: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Tan", (f))?;
        Ok(__cordl_ret.into())
    }
}
