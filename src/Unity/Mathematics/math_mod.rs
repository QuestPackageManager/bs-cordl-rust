#[cfg(feature = "Unity+Mathematics+math")]
#[repr(C)]
#[derive(Debug)]
pub struct math {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Mathematics+math")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::math => "Unity.Mathematics"
    ."math"
);
#[cfg(feature = "Unity+Mathematics+math")]
impl std::ops::Deref for crate::Unity::Mathematics::math {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+math")]
impl std::ops::DerefMut for crate::Unity::Mathematics::math {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+math")]
impl crate::Unity::Mathematics::math {
    pub const DBL_MIN_NORMAL: f64 = 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000022250738585072014f64;
    pub const E: f32 = 2.7182817f32;
    pub const EPSILON: f32 = 0.00000011920929f32;
    pub const EPSILON_DBL: f64 = 0.0000000000000002220446049250313f64;
    pub const E_DBL: f64 = 2.718281828459045f64;
    pub const FLT_MIN_NORMAL: f32 = 0.000000000000000000000000000000000000011754944f32;
    pub const INFINITY_DBL: f64 = std::f64::INFINITY;
    pub const LN10: f32 = 2.3025851f32;
    pub const LN10_DBL: f64 = 2.302585092994046f64;
    pub const LN2: f32 = 0.6931472f32;
    pub const LN2_DBL: f64 = 0.6931471805599453f64;
    pub const LOG10E: f32 = 0.4342945f32;
    pub const LOG10E_DBL: f64 = 0.4342944819032518f64;
    pub const LOG2E: f32 = 1.442695f32;
    pub const LOG2E_DBL: f64 = 1.4426950408889634f64;
    pub const NAN_DBL: f64 = std::f64::NAN;
    pub const PI: f32 = 3.1415927f32;
    pub const PI_DBL: f64 = 3.141592653589793f64;
    pub const SQRT2: f32 = 1.4142135f32;
    pub const SQRT2_DBL: f64 = 1.4142135623730951f64;
    pub const _cordl_INFINITY: f32 = std::f32::INFINITY;
    pub const _cordl_NAN: f32 = std::f64::NAN;
    #[cfg(feature = "Unity+Mathematics+math+IntFloatUnion")]
    pub type IntFloatUnion = crate::Unity::Mathematics::math_IntFloatUnion;
    #[cfg(feature = "Unity+Mathematics+math+LongDoubleUnion")]
    pub type LongDoubleUnion = crate::Unity::Mathematics::math_LongDoubleUnion;
    #[cfg(feature = "Unity+Mathematics+math+RotationOrder")]
    pub type RotationOrder = crate::Unity::Mathematics::math_RotationOrder;
    #[cfg(feature = "Unity+Mathematics+math+ShuffleComponent")]
    pub type ShuffleComponent = crate::Unity::Mathematics::math_ShuffleComponent;
    pub fn RigidTransform_float3x3_float3_1(
        rotation: crate::Unity::Mathematics::float3x3,
        translation: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RigidTransform", (rotation, translation))?;
        Ok(__cordl_ret.into())
    }
    pub fn RigidTransform_float4x4_2(
        transform: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RigidTransform", (transform))?;
        Ok(__cordl_ret.into())
    }
    pub fn RigidTransform_quaternion_float3_0(
        rot: crate::Unity::Mathematics::quaternion,
        pos: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RigidTransform", (rot, pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn abs_double2_10(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("abs", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn abs_double3_11(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("abs", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn abs_double4_12(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("abs", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn abs_f32_5(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("abs", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn abs_f64_9(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("abs", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn abs_float2_6(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("abs", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn abs_float3_7(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("abs", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn abs_float4_8(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("abs", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn abs_i32_0(x: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("abs", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn abs_i64_4(x: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("abs", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn abs_int2_1(
        x: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("abs", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn abs_int3_2(
        x: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("abs", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn abs_int4_3(
        x: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("abs", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn acos_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("acos", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn acos_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("acos", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn acos_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("acos", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn acos_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("acos", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn acos_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("acos", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn acos_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("acos", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn acos_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("acos", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn acos_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("acos", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn all_bool2_0(
        x: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("all", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn all_bool3_1(
        x: crate::Unity::Mathematics::bool3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("all", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn all_bool4_2(
        x: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("all", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn all_double2_12(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("all", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn all_double3_13(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("all", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn all_double4_14(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("all", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn all_float2_9(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("all", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn all_float3_10(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("all", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn all_float4_11(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("all", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn all_int2_3(
        x: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("all", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn all_int3_4(
        x: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("all", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn all_int4_5(
        x: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("all", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn all_uint2_6(
        x: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("all", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn all_uint3_7(
        x: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("all", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn all_uint4_8(
        x: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("all", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn any_bool2_0(
        x: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("any", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn any_bool3_1(
        x: crate::Unity::Mathematics::bool3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("any", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn any_bool4_2(
        x: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("any", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn any_double2_12(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("any", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn any_double3_13(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("any", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn any_double4_14(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("any", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn any_float2_9(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("any", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn any_float3_10(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("any", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn any_float4_11(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("any", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn any_int2_3(
        x: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("any", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn any_int3_4(
        x: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("any", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn any_int4_5(
        x: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("any", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn any_uint2_6(
        x: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("any", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn any_uint3_7(
        x: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("any", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn any_uint4_8(
        x: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("any", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asdouble_i64_0(x: i64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asdouble", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asdouble_u64_1(x: u64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asdouble", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asfloat_i32_0(x: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asfloat", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asfloat_int2_1(
        x: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asfloat", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asfloat_int3_2(
        x: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asfloat", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asfloat_int4_3(
        x: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asfloat", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asfloat_u32_4(x: u32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asfloat", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asfloat_uint2_5(
        x: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asfloat", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asfloat_uint3_6(
        x: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asfloat", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asfloat_uint4_7(
        x: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asfloat", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asin_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asin_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asin_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asin_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asin_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asin_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asin_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asin_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asint_f32_4(x: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asint", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asint_float2_5(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asint", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asint_float3_6(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asint", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asint_float4_7(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asint", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asint_u32_0(x: u32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asint", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asint_uint2_1(
        x: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asint", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asint_uint3_2(
        x: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asint", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asint_uint4_3(
        x: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asint", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn aslong_f64_1(x: f64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("aslong", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn aslong_u64_0(x: u64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("aslong", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asuint_f32_4(x: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asuint", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asuint_float2_5(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asuint", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asuint_float3_6(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asuint", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asuint_float4_7(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asuint", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asuint_i32_0(x: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asuint", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asuint_int2_1(
        x: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asuint", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asuint_int3_2(
        x: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asuint", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asuint_int4_3(
        x: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asuint", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asulong_f64_1(x: f64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asulong", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn asulong_i64_0(x: i64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("asulong", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn atan2_double2_double2_5(
        y: crate::Unity::Mathematics::double2,
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("atan2", (y, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn atan2_double3_double3_6(
        y: crate::Unity::Mathematics::double3,
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("atan2", (y, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn atan2_double4_double4_7(
        y: crate::Unity::Mathematics::double4,
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("atan2", (y, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn atan2_f32_f32_0(y: f32, x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("atan2", (y, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn atan2_f64_f64_4(y: f64, x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("atan2", (y, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn atan2_float2_float2_1(
        y: crate::Unity::Mathematics::float2,
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("atan2", (y, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn atan2_float3_float3_2(
        y: crate::Unity::Mathematics::float3,
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("atan2", (y, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn atan2_float4_float4_3(
        y: crate::Unity::Mathematics::float4,
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("atan2", (y, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn atan_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("atan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn atan_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("atan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn atan_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("atan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn atan_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("atan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn atan_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("atan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn atan_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("atan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn atan_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("atan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn atan_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("atan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn back() -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("back", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn bitmask(
        value: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bitmask", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool2__cordl_bool2(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool2__cordl_bool__cordl_bool0(
        x: bool,
        y: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool2", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool2_bool2_1(
        xy: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool2", (xy))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool2x2__cordl_bool2(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool2x2__cordl_bool__cordl_bool__cordl_bool__cordl_bool1(
        m00: bool,
        m01: bool,
        m10: bool,
        m11: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool2x2", (m00, m01, m10, m11))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool2x2_bool2_bool2_0(
        c0: crate::Unity::Mathematics::bool2,
        c1: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool2x2", (c0, c1))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool2x3__cordl_bool2(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool2x3__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool1(
        m00: bool,
        m01: bool,
        m02: bool,
        m10: bool,
        m11: bool,
        m12: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool2x3", (m00, m01, m02, m10, m11, m12))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool2x3_bool2_bool2_bool2_0(
        c0: crate::Unity::Mathematics::bool2,
        c1: crate::Unity::Mathematics::bool2,
        c2: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool2x3", (c0, c1, c2))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool2x4__cordl_bool2(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool2x4__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool1(
        m00: bool,
        m01: bool,
        m02: bool,
        m03: bool,
        m10: bool,
        m11: bool,
        m12: bool,
        m13: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool2x4", (m00, m01, m02, m03, m10, m11, m12, m13))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool2x4_bool2_bool2_bool2_bool2_0(
        c0: crate::Unity::Mathematics::bool2,
        c1: crate::Unity::Mathematics::bool2,
        c2: crate::Unity::Mathematics::bool2,
        c3: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool2x4", (c0, c1, c2, c3))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool3__cordl_bool4(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool3__cordl_bool__cordl_bool__cordl_bool0(
        x: bool,
        y: bool,
        z: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool3", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool3__cordl_bool_bool2_1(
        x: bool,
        yz: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool3", (x, yz))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool3_bool2__cordl_bool2(
        xy: crate::Unity::Mathematics::bool2,
        z: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool3", (xy, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool3_bool3_3(
        xyz: crate::Unity::Mathematics::bool3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool3", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool3x2__cordl_bool2(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool3x2__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool1(
        m00: bool,
        m01: bool,
        m10: bool,
        m11: bool,
        m20: bool,
        m21: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool3x2", (m00, m01, m10, m11, m20, m21))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool3x2_bool3_bool3_0(
        c0: crate::Unity::Mathematics::bool3,
        c1: crate::Unity::Mathematics::bool3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool3x2", (c0, c1))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool3x3__cordl_bool2(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool3x3__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool1(
        m00: bool,
        m01: bool,
        m02: bool,
        m10: bool,
        m11: bool,
        m12: bool,
        m20: bool,
        m21: bool,
        m22: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool3x3", (m00, m01, m02, m10, m11, m12, m20, m21, m22))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool3x3_bool3_bool3_bool3_0(
        c0: crate::Unity::Mathematics::bool3,
        c1: crate::Unity::Mathematics::bool3,
        c2: crate::Unity::Mathematics::bool3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool3x3", (c0, c1, c2))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool3x4__cordl_bool2(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool3x4__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool1(
        m00: bool,
        m01: bool,
        m02: bool,
        m03: bool,
        m10: bool,
        m11: bool,
        m12: bool,
        m13: bool,
        m20: bool,
        m21: bool,
        m22: bool,
        m23: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "bool3x4",
                (m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn bool3x4_bool3_bool3_bool3_bool3_0(
        c0: crate::Unity::Mathematics::bool3,
        c1: crate::Unity::Mathematics::bool3,
        c2: crate::Unity::Mathematics::bool3,
        c3: crate::Unity::Mathematics::bool3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool3x4", (c0, c1, c2, c3))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool4__cordl_bool8(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool4__cordl_bool__cordl_bool__cordl_bool__cordl_bool0(
        x: bool,
        y: bool,
        z: bool,
        w: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool4", (x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool4__cordl_bool__cordl_bool_bool2_1(
        x: bool,
        y: bool,
        zw: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool4", (x, y, zw))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool4__cordl_bool_bool2__cordl_bool2(
        x: bool,
        yz: crate::Unity::Mathematics::bool2,
        w: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool4", (x, yz, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool4__cordl_bool_bool3_3(
        x: bool,
        yzw: crate::Unity::Mathematics::bool3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool4", (x, yzw))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool4_bool2__cordl_bool__cordl_bool4(
        xy: crate::Unity::Mathematics::bool2,
        z: bool,
        w: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool4", (xy, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool4_bool2_bool2_5(
        xy: crate::Unity::Mathematics::bool2,
        zw: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool4", (xy, zw))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool4_bool3__cordl_bool6(
        xyz: crate::Unity::Mathematics::bool3,
        w: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool4", (xyz, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool4_bool4_7(
        xyzw: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool4", (xyzw))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool4x2__cordl_bool2(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool4x2__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool1(
        m00: bool,
        m01: bool,
        m10: bool,
        m11: bool,
        m20: bool,
        m21: bool,
        m30: bool,
        m31: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool4x2", (m00, m01, m10, m11, m20, m21, m30, m31))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool4x2_bool4_bool4_0(
        c0: crate::Unity::Mathematics::bool4,
        c1: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool4x2", (c0, c1))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool4x3__cordl_bool2(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool4x3__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool1(
        m00: bool,
        m01: bool,
        m02: bool,
        m10: bool,
        m11: bool,
        m12: bool,
        m20: bool,
        m21: bool,
        m22: bool,
        m30: bool,
        m31: bool,
        m32: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "bool4x3",
                (m00, m01, m02, m10, m11, m12, m20, m21, m22, m30, m31, m32),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn bool4x3_bool4_bool4_bool4_0(
        c0: crate::Unity::Mathematics::bool4,
        c1: crate::Unity::Mathematics::bool4,
        c2: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool4x3", (c0, c1, c2))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool4x4__cordl_bool2(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn bool4x4__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool1(
        m00: bool,
        m01: bool,
        m02: bool,
        m03: bool,
        m10: bool,
        m11: bool,
        m12: bool,
        m13: bool,
        m20: bool,
        m21: bool,
        m22: bool,
        m23: bool,
        m30: bool,
        m31: bool,
        m32: bool,
        m33: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "bool4x4",
                (
                    m00,
                    m01,
                    m02,
                    m03,
                    m10,
                    m11,
                    m12,
                    m13,
                    m20,
                    m21,
                    m22,
                    m23,
                    m30,
                    m31,
                    m32,
                    m33,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn bool4x4_bool4_bool4_bool4_bool4_0(
        c0: crate::Unity::Mathematics::bool4,
        c1: crate::Unity::Mathematics::bool4,
        c2: crate::Unity::Mathematics::bool4,
        c3: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("bool4x4", (c0, c1, c2, c3))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceil_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceil", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceil_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceil", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceil_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceil", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceil_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceil", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceil_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceil", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceil_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceil", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceil_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceil", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceil_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceil", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceillog2_i32_0(x: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceillog2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceillog2_int2_1(
        x: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceillog2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceillog2_int3_2(
        x: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceillog2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceillog2_int4_3(
        x: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceillog2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceillog2_u32_4(x: u32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceillog2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceillog2_uint2_5(
        x: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceillog2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceillog2_uint3_6(
        x: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceillog2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceillog2_uint4_7(
        x: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceillog2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceilpow2_i32_0(x: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceilpow2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceilpow2_i64_8(x: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceilpow2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceilpow2_int2_1(
        x: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceilpow2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceilpow2_int3_2(
        x: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceilpow2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceilpow2_int4_3(
        x: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceilpow2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceilpow2_u32_4(x: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceilpow2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceilpow2_u64_9(x: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceilpow2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceilpow2_uint2_5(
        x: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceilpow2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceilpow2_uint3_6(
        x: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceilpow2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ceilpow2_uint4_7(
        x: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ceilpow2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn clamp_double2_double2_double2_15(
        x: crate::Unity::Mathematics::double2,
        a: crate::Unity::Mathematics::double2,
        b: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("clamp", (x, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn clamp_double3_double3_double3_16(
        x: crate::Unity::Mathematics::double3,
        a: crate::Unity::Mathematics::double3,
        b: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("clamp", (x, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn clamp_double4_double4_double4_17(
        x: crate::Unity::Mathematics::double4,
        a: crate::Unity::Mathematics::double4,
        b: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("clamp", (x, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn clamp_f32_f32_f32_10(
        x: f32,
        a: f32,
        b: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("clamp", (x, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn clamp_f64_f64_f64_14(
        x: f64,
        a: f64,
        b: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("clamp", (x, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn clamp_float2_float2_float2_11(
        x: crate::Unity::Mathematics::float2,
        a: crate::Unity::Mathematics::float2,
        b: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("clamp", (x, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn clamp_float3_float3_float3_12(
        x: crate::Unity::Mathematics::float3,
        a: crate::Unity::Mathematics::float3,
        b: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("clamp", (x, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn clamp_float4_float4_float4_13(
        x: crate::Unity::Mathematics::float4,
        a: crate::Unity::Mathematics::float4,
        b: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("clamp", (x, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn clamp_i32_i32_i32_0(
        x: i32,
        a: i32,
        b: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("clamp", (x, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn clamp_i64_i64_i64_8(
        x: i64,
        a: i64,
        b: i64,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("clamp", (x, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn clamp_int2_int2_int2_1(
        x: crate::Unity::Mathematics::int2,
        a: crate::Unity::Mathematics::int2,
        b: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("clamp", (x, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn clamp_int3_int3_int3_2(
        x: crate::Unity::Mathematics::int3,
        a: crate::Unity::Mathematics::int3,
        b: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("clamp", (x, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn clamp_int4_int4_int4_3(
        x: crate::Unity::Mathematics::int4,
        a: crate::Unity::Mathematics::int4,
        b: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("clamp", (x, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn clamp_u32_u32_u32_4(
        x: u32,
        a: u32,
        b: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("clamp", (x, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn clamp_u64_u64_u64_9(
        x: u64,
        a: u64,
        b: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("clamp", (x, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn clamp_uint2_uint2_uint2_5(
        x: crate::Unity::Mathematics::uint2,
        a: crate::Unity::Mathematics::uint2,
        b: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("clamp", (x, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn clamp_uint3_uint3_uint3_6(
        x: crate::Unity::Mathematics::uint3,
        a: crate::Unity::Mathematics::uint3,
        b: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("clamp", (x, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn clamp_uint4_uint4_uint4_7(
        x: crate::Unity::Mathematics::uint4,
        a: crate::Unity::Mathematics::uint4,
        b: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("clamp", (x, a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmax_double2_9(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmax", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmax_double3_10(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmax", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmax_double4_11(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmax", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmax_float2_6(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmax", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmax_float3_7(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmax", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmax_float4_8(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmax", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmax_int2_0(
        x: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmax", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmax_int3_1(
        x: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmax", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmax_int4_2(
        x: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmax", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmax_uint2_3(
        x: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmax", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmax_uint3_4(
        x: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmax", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmax_uint4_5(
        x: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmax", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmin_double2_9(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmin_double3_10(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmin_double4_11(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmin_float2_6(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmin_float3_7(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmin_float4_8(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmin_int2_0(
        x: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmin_int3_1(
        x: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmin_int4_2(
        x: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmin_uint2_3(
        x: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmin_uint3_4(
        x: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cmin_uint4_5(
        x: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cmin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn compress_float4_2(
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index: i32,
        val: crate::Unity::Mathematics::float4,
        mask: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("compress", (output, index, val, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn compress_int4_0(
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index: i32,
        val: crate::Unity::Mathematics::int4,
        mask: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("compress", (output, index, val, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn compress_uint4_1(
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index: i32,
        val: crate::Unity::Mathematics::uint4,
        mask: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("compress", (output, index, val, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn conjugate(
        q: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("conjugate", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn cos_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cos", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cos_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cos", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cos_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cos", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cos_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cos", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cos_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cos", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cos_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cos", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cos_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cos", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cos_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cos", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cosh_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cosh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cosh_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cosh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cosh_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cosh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cosh_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cosh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cosh_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cosh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cosh_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cosh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cosh_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cosh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cosh_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cosh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn countbits_i32_0(x: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("countbits", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn countbits_i64_9(x: i64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("countbits", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn countbits_int2_1(
        x: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("countbits", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn countbits_int3_2(
        x: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("countbits", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn countbits_int4_3(
        x: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("countbits", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn countbits_u32_4(x: u32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("countbits", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn countbits_u64_8(x: u64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("countbits", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn countbits_uint2_5(
        x: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("countbits", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn countbits_uint3_6(
        x: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("countbits", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn countbits_uint4_7(
        x: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("countbits", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn cross_double3_double3_1(
        x: crate::Unity::Mathematics::double3,
        y: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cross", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn cross_float3_float3_0(
        x: crate::Unity::Mathematics::float3,
        y: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cross", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn csum_double2_9(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("csum", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn csum_double3_10(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("csum", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn csum_double4_11(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("csum", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn csum_float2_6(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("csum", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn csum_float3_7(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("csum", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn csum_float4_8(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("csum", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn csum_int2_0(
        x: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("csum", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn csum_int3_1(
        x: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("csum", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn csum_int4_2(
        x: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("csum", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn csum_uint2_3(
        x: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("csum", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn csum_uint3_4(
        x: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("csum", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn csum_uint4_5(
        x: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("csum", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn degrees_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("degrees", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn degrees_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("degrees", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn degrees_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("degrees", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn degrees_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("degrees", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn degrees_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("degrees", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn degrees_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("degrees", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn degrees_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("degrees", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn degrees_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("degrees", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn determinant_double2x2_0(
        m: crate::Unity::Mathematics::double2x2,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("determinant", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn determinant_double3x3_1(
        m: crate::Unity::Mathematics::double3x3,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("determinant", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn determinant_double4x4_2(
        m: crate::Unity::Mathematics::double4x4,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("determinant", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn determinant_float2x2_3(
        m: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("determinant", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn determinant_float3x3_4(
        m: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("determinant", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn determinant_float4x4_5(
        m: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("determinant", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn determinant_int2x2_6(
        m: crate::Unity::Mathematics::int2x2,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("determinant", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn determinant_int3x3_7(
        m: crate::Unity::Mathematics::int3x3,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("determinant", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn determinant_int4x4_8(
        m: crate::Unity::Mathematics::int4x4,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("determinant", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn distance_double2_double2_5(
        x: crate::Unity::Mathematics::double2,
        y: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("distance", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn distance_double3_double3_6(
        x: crate::Unity::Mathematics::double3,
        y: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("distance", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn distance_double4_double4_7(
        x: crate::Unity::Mathematics::double4,
        y: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("distance", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn distance_f32_f32_0(x: f32, y: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("distance", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn distance_f64_f64_4(x: f64, y: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("distance", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn distance_float2_float2_1(
        x: crate::Unity::Mathematics::float2,
        y: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("distance", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn distance_float3_float3_2(
        x: crate::Unity::Mathematics::float3,
        y: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("distance", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn distance_float4_float4_3(
        x: crate::Unity::Mathematics::float4,
        y: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("distance", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn distancesq_double2_double2_5(
        x: crate::Unity::Mathematics::double2,
        y: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("distancesq", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn distancesq_double3_double3_6(
        x: crate::Unity::Mathematics::double3,
        y: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("distancesq", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn distancesq_double4_double4_7(
        x: crate::Unity::Mathematics::double4,
        y: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("distancesq", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn distancesq_f32_f32_0(x: f32, y: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("distancesq", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn distancesq_f64_f64_4(x: f64, y: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("distancesq", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn distancesq_float2_float2_1(
        x: crate::Unity::Mathematics::float2,
        y: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("distancesq", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn distancesq_float3_float3_2(
        x: crate::Unity::Mathematics::float3,
        y: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("distancesq", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn distancesq_float4_float4_3(
        x: crate::Unity::Mathematics::float4,
        y: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("distancesq", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn dot_double2_double2_13(
        x: crate::Unity::Mathematics::double2,
        y: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("dot", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn dot_double3_double3_14(
        x: crate::Unity::Mathematics::double3,
        y: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("dot", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn dot_double4_double4_15(
        x: crate::Unity::Mathematics::double4,
        y: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("dot", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn dot_f32_f32_8(x: f32, y: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("dot", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn dot_f64_f64_12(x: f64, y: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("dot", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn dot_float2_float2_9(
        x: crate::Unity::Mathematics::float2,
        y: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("dot", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn dot_float3_float3_10(
        x: crate::Unity::Mathematics::float3,
        y: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("dot", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn dot_float4_float4_11(
        x: crate::Unity::Mathematics::float4,
        y: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("dot", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn dot_i32_i32_0(x: i32, y: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("dot", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn dot_int2_int2_1(
        x: crate::Unity::Mathematics::int2,
        y: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("dot", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn dot_int3_int3_2(
        x: crate::Unity::Mathematics::int3,
        y: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("dot", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn dot_int4_int4_3(
        x: crate::Unity::Mathematics::int4,
        y: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("dot", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn dot_quaternion_quaternion16(
        a: crate::Unity::Mathematics::quaternion,
        b: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("dot", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn dot_u32_u32_4(x: u32, y: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("dot", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn dot_uint2_uint2_5(
        x: crate::Unity::Mathematics::uint2,
        y: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("dot", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn dot_uint3_uint3_6(
        x: crate::Unity::Mathematics::uint3,
        y: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("dot", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn dot_uint4_uint4_7(
        x: crate::Unity::Mathematics::uint4,
        y: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("dot", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2_bool2_4(
        v: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2_double2_1(
        xy: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2", (xy))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2_f32_11(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2_f64_2(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2_f64_f64_0(
        x: f64,
        y: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2_float2_12(
        v: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2_half2_10(
        v: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2_half9(
        v: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2_int2_6(
        v: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2_uint2_8(
        v: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x2__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x2> {
        let __cordl_ret: crate::Unity::Mathematics::double2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x2_bool2x2_4(
        v: crate::Unity::Mathematics::bool2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x2> {
        let __cordl_ret: crate::Unity::Mathematics::double2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x2_double2_double2_0(
        c0: crate::Unity::Mathematics::double2,
        c1: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x2> {
        let __cordl_ret: crate::Unity::Mathematics::double2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x2", (c0, c1))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x2_f32_9(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x2> {
        let __cordl_ret: crate::Unity::Mathematics::double2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x2_f64_2(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x2> {
        let __cordl_ret: crate::Unity::Mathematics::double2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x2_f64_f64_f64_f64_1(
        m00: f64,
        m01: f64,
        m10: f64,
        m11: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x2> {
        let __cordl_ret: crate::Unity::Mathematics::double2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x2", (m00, m01, m10, m11))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x2_float2x2_10(
        v: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x2> {
        let __cordl_ret: crate::Unity::Mathematics::double2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x2_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x2> {
        let __cordl_ret: crate::Unity::Mathematics::double2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x2_int2x2_6(
        v: crate::Unity::Mathematics::int2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x2> {
        let __cordl_ret: crate::Unity::Mathematics::double2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x2_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x2> {
        let __cordl_ret: crate::Unity::Mathematics::double2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x2_uint2x2_8(
        v: crate::Unity::Mathematics::uint2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x2> {
        let __cordl_ret: crate::Unity::Mathematics::double2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x3__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x3> {
        let __cordl_ret: crate::Unity::Mathematics::double2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x3_bool2x3_4(
        v: crate::Unity::Mathematics::bool2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x3> {
        let __cordl_ret: crate::Unity::Mathematics::double2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x3_double2_double2_double2_0(
        c0: crate::Unity::Mathematics::double2,
        c1: crate::Unity::Mathematics::double2,
        c2: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x3> {
        let __cordl_ret: crate::Unity::Mathematics::double2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x3", (c0, c1, c2))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x3_f32_9(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x3> {
        let __cordl_ret: crate::Unity::Mathematics::double2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x3_f64_2(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x3> {
        let __cordl_ret: crate::Unity::Mathematics::double2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x3_f64_f64_f64_f64_f64_f64_1(
        m00: f64,
        m01: f64,
        m02: f64,
        m10: f64,
        m11: f64,
        m12: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x3> {
        let __cordl_ret: crate::Unity::Mathematics::double2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x3", (m00, m01, m02, m10, m11, m12))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x3_float2x3_10(
        v: crate::Unity::Mathematics::float2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x3> {
        let __cordl_ret: crate::Unity::Mathematics::double2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x3_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x3> {
        let __cordl_ret: crate::Unity::Mathematics::double2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x3_int2x3_6(
        v: crate::Unity::Mathematics::int2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x3> {
        let __cordl_ret: crate::Unity::Mathematics::double2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x3_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x3> {
        let __cordl_ret: crate::Unity::Mathematics::double2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x3_uint2x3_8(
        v: crate::Unity::Mathematics::uint2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x3> {
        let __cordl_ret: crate::Unity::Mathematics::double2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x4__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x4> {
        let __cordl_ret: crate::Unity::Mathematics::double2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x4_bool2x4_4(
        v: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x4> {
        let __cordl_ret: crate::Unity::Mathematics::double2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x4_double2_double2_double2_double2_0(
        c0: crate::Unity::Mathematics::double2,
        c1: crate::Unity::Mathematics::double2,
        c2: crate::Unity::Mathematics::double2,
        c3: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x4> {
        let __cordl_ret: crate::Unity::Mathematics::double2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x4", (c0, c1, c2, c3))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x4_f32_9(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x4> {
        let __cordl_ret: crate::Unity::Mathematics::double2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x4_f64_2(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x4> {
        let __cordl_ret: crate::Unity::Mathematics::double2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x4_f64_f64_f64_f64_f64_f64_f64_f64_1(
        m00: f64,
        m01: f64,
        m02: f64,
        m03: f64,
        m10: f64,
        m11: f64,
        m12: f64,
        m13: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x4> {
        let __cordl_ret: crate::Unity::Mathematics::double2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x4", (m00, m01, m02, m03, m10, m11, m12, m13))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x4_float2x4_10(
        v: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x4> {
        let __cordl_ret: crate::Unity::Mathematics::double2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x4_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x4> {
        let __cordl_ret: crate::Unity::Mathematics::double2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x4_int2x4_6(
        v: crate::Unity::Mathematics::int2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x4> {
        let __cordl_ret: crate::Unity::Mathematics::double2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x4_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x4> {
        let __cordl_ret: crate::Unity::Mathematics::double2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double2x4_uint2x4_8(
        v: crate::Unity::Mathematics::uint2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x4> {
        let __cordl_ret: crate::Unity::Mathematics::double2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3__cordl_bool5(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3_bool3_6(
        v: crate::Unity::Mathematics::bool3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3_double2_f64_2(
        xy: crate::Unity::Mathematics::double2,
        z: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3", (xy, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3_double3_3(
        xyz: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3_f32_13(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3_f64_4(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3_f64_double2_1(
        x: f64,
        yz: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3", (x, yz))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3_f64_f64_f64_0(
        x: f64,
        y: f64,
        z: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3_float3_14(
        v: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3_half11(
        v: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3_half3_12(
        v: crate::Unity::Mathematics::half3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3_i32_7(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3_int3_8(
        v: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3_u32_9(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3_uint3_10(
        v: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x2__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x2> {
        let __cordl_ret: crate::Unity::Mathematics::double3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x2_bool3x2_4(
        v: crate::Unity::Mathematics::bool3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x2> {
        let __cordl_ret: crate::Unity::Mathematics::double3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x2_double3_double3_0(
        c0: crate::Unity::Mathematics::double3,
        c1: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x2> {
        let __cordl_ret: crate::Unity::Mathematics::double3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x2", (c0, c1))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x2_f32_9(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x2> {
        let __cordl_ret: crate::Unity::Mathematics::double3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x2_f64_2(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x2> {
        let __cordl_ret: crate::Unity::Mathematics::double3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x2_f64_f64_f64_f64_f64_f64_1(
        m00: f64,
        m01: f64,
        m10: f64,
        m11: f64,
        m20: f64,
        m21: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x2> {
        let __cordl_ret: crate::Unity::Mathematics::double3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x2", (m00, m01, m10, m11, m20, m21))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x2_float3x2_10(
        v: crate::Unity::Mathematics::float3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x2> {
        let __cordl_ret: crate::Unity::Mathematics::double3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x2_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x2> {
        let __cordl_ret: crate::Unity::Mathematics::double3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x2_int3x2_6(
        v: crate::Unity::Mathematics::int3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x2> {
        let __cordl_ret: crate::Unity::Mathematics::double3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x2_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x2> {
        let __cordl_ret: crate::Unity::Mathematics::double3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x2_uint3x2_8(
        v: crate::Unity::Mathematics::uint3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x2> {
        let __cordl_ret: crate::Unity::Mathematics::double3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x3__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x3> {
        let __cordl_ret: crate::Unity::Mathematics::double3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x3_bool3x3_4(
        v: crate::Unity::Mathematics::bool3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x3> {
        let __cordl_ret: crate::Unity::Mathematics::double3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x3_double3_double3_double3_0(
        c0: crate::Unity::Mathematics::double3,
        c1: crate::Unity::Mathematics::double3,
        c2: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x3> {
        let __cordl_ret: crate::Unity::Mathematics::double3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x3", (c0, c1, c2))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x3_f32_9(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x3> {
        let __cordl_ret: crate::Unity::Mathematics::double3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x3_f64_2(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x3> {
        let __cordl_ret: crate::Unity::Mathematics::double3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x3_f64_f64_f64_f64_f64_f64_f64_f64_f64_1(
        m00: f64,
        m01: f64,
        m02: f64,
        m10: f64,
        m11: f64,
        m12: f64,
        m20: f64,
        m21: f64,
        m22: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x3> {
        let __cordl_ret: crate::Unity::Mathematics::double3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x3", (m00, m01, m02, m10, m11, m12, m20, m21, m22))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x3_float3x3_10(
        v: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x3> {
        let __cordl_ret: crate::Unity::Mathematics::double3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x3_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x3> {
        let __cordl_ret: crate::Unity::Mathematics::double3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x3_int3x3_6(
        v: crate::Unity::Mathematics::int3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x3> {
        let __cordl_ret: crate::Unity::Mathematics::double3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x3_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x3> {
        let __cordl_ret: crate::Unity::Mathematics::double3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x3_uint3x3_8(
        v: crate::Unity::Mathematics::uint3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x3> {
        let __cordl_ret: crate::Unity::Mathematics::double3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x4__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x4> {
        let __cordl_ret: crate::Unity::Mathematics::double3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x4_bool3x4_4(
        v: crate::Unity::Mathematics::bool3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x4> {
        let __cordl_ret: crate::Unity::Mathematics::double3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x4_double3_double3_double3_double3_0(
        c0: crate::Unity::Mathematics::double3,
        c1: crate::Unity::Mathematics::double3,
        c2: crate::Unity::Mathematics::double3,
        c3: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x4> {
        let __cordl_ret: crate::Unity::Mathematics::double3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x4", (c0, c1, c2, c3))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x4_f32_9(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x4> {
        let __cordl_ret: crate::Unity::Mathematics::double3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x4_f64_2(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x4> {
        let __cordl_ret: crate::Unity::Mathematics::double3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x4_f64_f64_f64_f64_f64_f64_f64_f64_f64_f64_f64_f64_1(
        m00: f64,
        m01: f64,
        m02: f64,
        m03: f64,
        m10: f64,
        m11: f64,
        m12: f64,
        m13: f64,
        m20: f64,
        m21: f64,
        m22: f64,
        m23: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x4> {
        let __cordl_ret: crate::Unity::Mathematics::double3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "double3x4",
                (m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x4_float3x4_10(
        v: crate::Unity::Mathematics::float3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x4> {
        let __cordl_ret: crate::Unity::Mathematics::double3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x4_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x4> {
        let __cordl_ret: crate::Unity::Mathematics::double3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x4_int3x4_6(
        v: crate::Unity::Mathematics::int3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x4> {
        let __cordl_ret: crate::Unity::Mathematics::double3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x4_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x4> {
        let __cordl_ret: crate::Unity::Mathematics::double3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double3x4_uint3x4_8(
        v: crate::Unity::Mathematics::uint3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x4> {
        let __cordl_ret: crate::Unity::Mathematics::double3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4__cordl_bool9(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4_bool4_10(
        v: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4_double2_double2_5(
        xy: crate::Unity::Mathematics::double2,
        zw: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4", (xy, zw))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4_double2_f64_f64_4(
        xy: crate::Unity::Mathematics::double2,
        z: f64,
        w: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4", (xy, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4_double3_f64_6(
        xyz: crate::Unity::Mathematics::double3,
        w: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4", (xyz, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4_double4_7(
        xyzw: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4", (xyzw))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4_f32_17(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4_f64_8(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4_f64_double2_f64_2(
        x: f64,
        yz: crate::Unity::Mathematics::double2,
        w: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4", (x, yz, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4_f64_double3_3(
        x: f64,
        yzw: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4", (x, yzw))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4_f64_f64_double2_1(
        x: f64,
        y: f64,
        zw: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4", (x, y, zw))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4_f64_f64_f64_f64_0(
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4", (x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4_float4_18(
        v: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4_half15(
        v: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4_half4_16(
        v: crate::Unity::Mathematics::half4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4_i32_11(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4_int4_12(
        v: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4_u32_13(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4_uint4_14(
        v: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x2__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x2> {
        let __cordl_ret: crate::Unity::Mathematics::double4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x2_bool4x2_4(
        v: crate::Unity::Mathematics::bool4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x2> {
        let __cordl_ret: crate::Unity::Mathematics::double4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x2_double4_double4_0(
        c0: crate::Unity::Mathematics::double4,
        c1: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x2> {
        let __cordl_ret: crate::Unity::Mathematics::double4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x2", (c0, c1))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x2_f32_9(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x2> {
        let __cordl_ret: crate::Unity::Mathematics::double4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x2_f64_2(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x2> {
        let __cordl_ret: crate::Unity::Mathematics::double4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x2_f64_f64_f64_f64_f64_f64_f64_f64_1(
        m00: f64,
        m01: f64,
        m10: f64,
        m11: f64,
        m20: f64,
        m21: f64,
        m30: f64,
        m31: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x2> {
        let __cordl_ret: crate::Unity::Mathematics::double4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x2", (m00, m01, m10, m11, m20, m21, m30, m31))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x2_float4x2_10(
        v: crate::Unity::Mathematics::float4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x2> {
        let __cordl_ret: crate::Unity::Mathematics::double4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x2_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x2> {
        let __cordl_ret: crate::Unity::Mathematics::double4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x2_int4x2_6(
        v: crate::Unity::Mathematics::int4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x2> {
        let __cordl_ret: crate::Unity::Mathematics::double4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x2_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x2> {
        let __cordl_ret: crate::Unity::Mathematics::double4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x2_uint4x2_8(
        v: crate::Unity::Mathematics::uint4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x2> {
        let __cordl_ret: crate::Unity::Mathematics::double4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x3__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x3_bool4x3_4(
        v: crate::Unity::Mathematics::bool4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x3_double4_double4_double4_0(
        c0: crate::Unity::Mathematics::double4,
        c1: crate::Unity::Mathematics::double4,
        c2: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x3", (c0, c1, c2))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x3_f32_9(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x3_f64_2(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x3_f64_f64_f64_f64_f64_f64_f64_f64_f64_f64_f64_f64_1(
        m00: f64,
        m01: f64,
        m02: f64,
        m10: f64,
        m11: f64,
        m12: f64,
        m20: f64,
        m21: f64,
        m22: f64,
        m30: f64,
        m31: f64,
        m32: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "double4x3",
                (m00, m01, m02, m10, m11, m12, m20, m21, m22, m30, m31, m32),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x3_float4x3_10(
        v: crate::Unity::Mathematics::float4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x3_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x3_int4x3_6(
        v: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x3_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x3_uint4x3_8(
        v: crate::Unity::Mathematics::uint4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x4__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x4> {
        let __cordl_ret: crate::Unity::Mathematics::double4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x4_bool4x4_4(
        v: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x4> {
        let __cordl_ret: crate::Unity::Mathematics::double4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x4_double4_double4_double4_double4_0(
        c0: crate::Unity::Mathematics::double4,
        c1: crate::Unity::Mathematics::double4,
        c2: crate::Unity::Mathematics::double4,
        c3: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x4> {
        let __cordl_ret: crate::Unity::Mathematics::double4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x4", (c0, c1, c2, c3))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x4_f32_9(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x4> {
        let __cordl_ret: crate::Unity::Mathematics::double4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x4_f64_2(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x4> {
        let __cordl_ret: crate::Unity::Mathematics::double4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x4_f64_f64_f64_f64_f64_f64_f64_f64_f64_f64_f64_f64_f64_f64_f64_f64_1(
        m00: f64,
        m01: f64,
        m02: f64,
        m03: f64,
        m10: f64,
        m11: f64,
        m12: f64,
        m13: f64,
        m20: f64,
        m21: f64,
        m22: f64,
        m23: f64,
        m30: f64,
        m31: f64,
        m32: f64,
        m33: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x4> {
        let __cordl_ret: crate::Unity::Mathematics::double4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "double4x4",
                (
                    m00,
                    m01,
                    m02,
                    m03,
                    m10,
                    m11,
                    m12,
                    m13,
                    m20,
                    m21,
                    m22,
                    m23,
                    m30,
                    m31,
                    m32,
                    m33,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x4_float4x4_10(
        v: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x4> {
        let __cordl_ret: crate::Unity::Mathematics::double4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x4_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x4> {
        let __cordl_ret: crate::Unity::Mathematics::double4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x4_int4x4_6(
        v: crate::Unity::Mathematics::int4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x4> {
        let __cordl_ret: crate::Unity::Mathematics::double4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x4_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x4> {
        let __cordl_ret: crate::Unity::Mathematics::double4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn double4x4_uint4x4_8(
        v: crate::Unity::Mathematics::uint4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x4> {
        let __cordl_ret: crate::Unity::Mathematics::double4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("double4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn down() -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("down", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn exp10_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp10", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp10_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp10", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp10_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp10", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp10_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp10", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp10_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp10", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp10_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp10", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp10_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp10", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp10_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp10", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp2_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp2_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp2_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp2_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp2_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp2_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp2_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp2_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn exp_quaternion8(
        q: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("exp", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn f16tof32_u32_0(x: u32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("f16tof32", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn f16tof32_uint2_1(
        x: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("f16tof32", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn f16tof32_uint3_2(
        x: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("f16tof32", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn f16tof32_uint4_3(
        x: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("f16tof32", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn f32tof16_f32_0(x: f32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("f32tof16", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn f32tof16_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("f32tof16", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn f32tof16_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("f32tof16", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn f32tof16_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("f32tof16", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn faceforward_double2_double2_double2_3(
        n: crate::Unity::Mathematics::double2,
        i: crate::Unity::Mathematics::double2,
        ng: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("faceforward", (n, i, ng))?;
        Ok(__cordl_ret.into())
    }
    pub fn faceforward_double3_double3_double3_4(
        n: crate::Unity::Mathematics::double3,
        i: crate::Unity::Mathematics::double3,
        ng: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("faceforward", (n, i, ng))?;
        Ok(__cordl_ret.into())
    }
    pub fn faceforward_double4_double4_double4_5(
        n: crate::Unity::Mathematics::double4,
        i: crate::Unity::Mathematics::double4,
        ng: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("faceforward", (n, i, ng))?;
        Ok(__cordl_ret.into())
    }
    pub fn faceforward_float2_float2_float2_0(
        n: crate::Unity::Mathematics::float2,
        i: crate::Unity::Mathematics::float2,
        ng: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("faceforward", (n, i, ng))?;
        Ok(__cordl_ret.into())
    }
    pub fn faceforward_float3_float3_float3_1(
        n: crate::Unity::Mathematics::float3,
        i: crate::Unity::Mathematics::float3,
        ng: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("faceforward", (n, i, ng))?;
        Ok(__cordl_ret.into())
    }
    pub fn faceforward_float4_float4_float4_2(
        n: crate::Unity::Mathematics::float4,
        i: crate::Unity::Mathematics::float4,
        ng: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("faceforward", (n, i, ng))?;
        Ok(__cordl_ret.into())
    }
    pub fn fastinverse_double3x4_0(
        m: crate::Unity::Mathematics::double3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x4> {
        let __cordl_ret: crate::Unity::Mathematics::double3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fastinverse", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn fastinverse_double4x4_1(
        m: crate::Unity::Mathematics::double4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x4> {
        let __cordl_ret: crate::Unity::Mathematics::double4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fastinverse", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn fastinverse_float3x4_2(
        m: crate::Unity::Mathematics::float3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x4> {
        let __cordl_ret: crate::Unity::Mathematics::float3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fastinverse", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn fastinverse_float4x4_3(
        m: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fastinverse", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2_bool2_4(
        v: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2_double2_12(
        v: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2_f32_2(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2_f32_f32_0(
        x: f32,
        y: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2_f64_11(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2_float2_1(
        xy: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2", (xy))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2_half2_10(
        v: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2_half9(
        v: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2_int2_6(
        v: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2_uint2_8(
        v: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x2__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x2_bool2x2_4(
        v: crate::Unity::Mathematics::bool2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x2_double2x2_10(
        v: crate::Unity::Mathematics::double2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x2_f32_2(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x2_f32_f32_f32_f32_1(
        m00: f32,
        m01: f32,
        m10: f32,
        m11: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x2", (m00, m01, m10, m11))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x2_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x2_float2_float2_0(
        c0: crate::Unity::Mathematics::float2,
        c1: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x2", (c0, c1))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x2_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x2_int2x2_6(
        v: crate::Unity::Mathematics::int2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x2_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x2_uint2x2_8(
        v: crate::Unity::Mathematics::uint2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x3__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x3> {
        let __cordl_ret: crate::Unity::Mathematics::float2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x3_bool2x3_4(
        v: crate::Unity::Mathematics::bool2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x3> {
        let __cordl_ret: crate::Unity::Mathematics::float2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x3_double2x3_10(
        v: crate::Unity::Mathematics::double2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x3> {
        let __cordl_ret: crate::Unity::Mathematics::float2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x3_f32_2(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x3> {
        let __cordl_ret: crate::Unity::Mathematics::float2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x3_f32_f32_f32_f32_f32_f32_1(
        m00: f32,
        m01: f32,
        m02: f32,
        m10: f32,
        m11: f32,
        m12: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x3> {
        let __cordl_ret: crate::Unity::Mathematics::float2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x3", (m00, m01, m02, m10, m11, m12))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x3_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x3> {
        let __cordl_ret: crate::Unity::Mathematics::float2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x3_float2_float2_float2_0(
        c0: crate::Unity::Mathematics::float2,
        c1: crate::Unity::Mathematics::float2,
        c2: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x3> {
        let __cordl_ret: crate::Unity::Mathematics::float2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x3", (c0, c1, c2))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x3_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x3> {
        let __cordl_ret: crate::Unity::Mathematics::float2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x3_int2x3_6(
        v: crate::Unity::Mathematics::int2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x3> {
        let __cordl_ret: crate::Unity::Mathematics::float2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x3_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x3> {
        let __cordl_ret: crate::Unity::Mathematics::float2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x3_uint2x3_8(
        v: crate::Unity::Mathematics::uint2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x3> {
        let __cordl_ret: crate::Unity::Mathematics::float2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x4__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x4_bool2x4_4(
        v: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x4_double2x4_10(
        v: crate::Unity::Mathematics::double2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x4_f32_2(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x4_f32_f32_f32_f32_f32_f32_f32_f32_1(
        m00: f32,
        m01: f32,
        m02: f32,
        m03: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m13: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x4", (m00, m01, m02, m03, m10, m11, m12, m13))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x4_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x4_float2_float2_float2_float2_0(
        c0: crate::Unity::Mathematics::float2,
        c1: crate::Unity::Mathematics::float2,
        c2: crate::Unity::Mathematics::float2,
        c3: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x4", (c0, c1, c2, c3))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x4_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x4_int2x4_6(
        v: crate::Unity::Mathematics::int2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x4_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float2x4_uint2x4_8(
        v: crate::Unity::Mathematics::uint2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3__cordl_bool5(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3_bool3_6(
        v: crate::Unity::Mathematics::bool3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3_double3_14(
        v: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3_f32_4(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3_f32_f32_f32_0(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3_f32_float2_1(
        x: f32,
        yz: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3", (x, yz))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3_f64_13(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3_float2_f32_2(
        xy: crate::Unity::Mathematics::float2,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3", (xy, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3_float3_3(
        xyz: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3_half11(
        v: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3_half3_12(
        v: crate::Unity::Mathematics::half3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3_i32_7(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3_int3_8(
        v: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3_u32_9(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3_uint3_10(
        v: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x2__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x2> {
        let __cordl_ret: crate::Unity::Mathematics::float3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x2_bool3x2_4(
        v: crate::Unity::Mathematics::bool3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x2> {
        let __cordl_ret: crate::Unity::Mathematics::float3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x2_double3x2_10(
        v: crate::Unity::Mathematics::double3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x2> {
        let __cordl_ret: crate::Unity::Mathematics::float3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x2_f32_2(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x2> {
        let __cordl_ret: crate::Unity::Mathematics::float3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x2_f32_f32_f32_f32_f32_f32_1(
        m00: f32,
        m01: f32,
        m10: f32,
        m11: f32,
        m20: f32,
        m21: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x2> {
        let __cordl_ret: crate::Unity::Mathematics::float3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x2", (m00, m01, m10, m11, m20, m21))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x2_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x2> {
        let __cordl_ret: crate::Unity::Mathematics::float3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x2_float3_float3_0(
        c0: crate::Unity::Mathematics::float3,
        c1: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x2> {
        let __cordl_ret: crate::Unity::Mathematics::float3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x2", (c0, c1))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x2_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x2> {
        let __cordl_ret: crate::Unity::Mathematics::float3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x2_int3x2_6(
        v: crate::Unity::Mathematics::int3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x2> {
        let __cordl_ret: crate::Unity::Mathematics::float3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x2_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x2> {
        let __cordl_ret: crate::Unity::Mathematics::float3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x2_uint3x2_8(
        v: crate::Unity::Mathematics::uint3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x2> {
        let __cordl_ret: crate::Unity::Mathematics::float3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x3__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x3_bool3x3_4(
        v: crate::Unity::Mathematics::bool3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x3_double3x3_10(
        v: crate::Unity::Mathematics::double3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x3_f32_2(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x3_f32_f32_f32_f32_f32_f32_f32_f32_f32_1(
        m00: f32,
        m01: f32,
        m02: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m20: f32,
        m21: f32,
        m22: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x3", (m00, m01, m02, m10, m11, m12, m20, m21, m22))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x3_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x3_float3_float3_float3_0(
        c0: crate::Unity::Mathematics::float3,
        c1: crate::Unity::Mathematics::float3,
        c2: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x3", (c0, c1, c2))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x3_float4x4_11(
        f4x4: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x3", (f4x4))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x3_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x3_int3x3_6(
        v: crate::Unity::Mathematics::int3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x3_quaternion12(
        rotation: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x3", (rotation))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x3_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x3_uint3x3_8(
        v: crate::Unity::Mathematics::uint3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x4__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x4> {
        let __cordl_ret: crate::Unity::Mathematics::float3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x4_bool3x4_4(
        v: crate::Unity::Mathematics::bool3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x4> {
        let __cordl_ret: crate::Unity::Mathematics::float3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x4_double3x4_10(
        v: crate::Unity::Mathematics::double3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x4> {
        let __cordl_ret: crate::Unity::Mathematics::float3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x4_f32_2(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x4> {
        let __cordl_ret: crate::Unity::Mathematics::float3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x4_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_1(
        m00: f32,
        m01: f32,
        m02: f32,
        m03: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m13: f32,
        m20: f32,
        m21: f32,
        m22: f32,
        m23: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x4> {
        let __cordl_ret: crate::Unity::Mathematics::float3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "float3x4",
                (m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x4_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x4> {
        let __cordl_ret: crate::Unity::Mathematics::float3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x4_float3_float3_float3_float3_0(
        c0: crate::Unity::Mathematics::float3,
        c1: crate::Unity::Mathematics::float3,
        c2: crate::Unity::Mathematics::float3,
        c3: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x4> {
        let __cordl_ret: crate::Unity::Mathematics::float3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x4", (c0, c1, c2, c3))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x4_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x4> {
        let __cordl_ret: crate::Unity::Mathematics::float3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x4_int3x4_6(
        v: crate::Unity::Mathematics::int3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x4> {
        let __cordl_ret: crate::Unity::Mathematics::float3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x4_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x4> {
        let __cordl_ret: crate::Unity::Mathematics::float3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float3x4_uint3x4_8(
        v: crate::Unity::Mathematics::uint3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x4> {
        let __cordl_ret: crate::Unity::Mathematics::float3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4__cordl_bool9(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4_bool4_10(
        v: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4_double4_18(
        v: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4_f32_8(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4_f32_f32_f32_f32_0(
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4", (x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4_f32_f32_float2_1(
        x: f32,
        y: f32,
        zw: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4", (x, y, zw))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4_f32_float2_f32_2(
        x: f32,
        yz: crate::Unity::Mathematics::float2,
        w: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4", (x, yz, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4_f32_float3_3(
        x: f32,
        yzw: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4", (x, yzw))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4_f64_17(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4_float2_f32_f32_4(
        xy: crate::Unity::Mathematics::float2,
        z: f32,
        w: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4", (xy, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4_float2_float2_5(
        xy: crate::Unity::Mathematics::float2,
        zw: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4", (xy, zw))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4_float3_f32_6(
        xyz: crate::Unity::Mathematics::float3,
        w: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4", (xyz, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4_float4_7(
        xyzw: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4", (xyzw))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4_half15(
        v: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4_half4_16(
        v: crate::Unity::Mathematics::half4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4_i32_11(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4_int4_12(
        v: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4_u32_13(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4_uint4_14(
        v: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x2__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x2> {
        let __cordl_ret: crate::Unity::Mathematics::float4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x2_bool4x2_4(
        v: crate::Unity::Mathematics::bool4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x2> {
        let __cordl_ret: crate::Unity::Mathematics::float4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x2_double4x2_10(
        v: crate::Unity::Mathematics::double4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x2> {
        let __cordl_ret: crate::Unity::Mathematics::float4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x2_f32_2(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x2> {
        let __cordl_ret: crate::Unity::Mathematics::float4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x2_f32_f32_f32_f32_f32_f32_f32_f32_1(
        m00: f32,
        m01: f32,
        m10: f32,
        m11: f32,
        m20: f32,
        m21: f32,
        m30: f32,
        m31: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x2> {
        let __cordl_ret: crate::Unity::Mathematics::float4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x2", (m00, m01, m10, m11, m20, m21, m30, m31))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x2_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x2> {
        let __cordl_ret: crate::Unity::Mathematics::float4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x2_float4_float4_0(
        c0: crate::Unity::Mathematics::float4,
        c1: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x2> {
        let __cordl_ret: crate::Unity::Mathematics::float4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x2", (c0, c1))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x2_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x2> {
        let __cordl_ret: crate::Unity::Mathematics::float4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x2_int4x2_6(
        v: crate::Unity::Mathematics::int4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x2> {
        let __cordl_ret: crate::Unity::Mathematics::float4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x2_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x2> {
        let __cordl_ret: crate::Unity::Mathematics::float4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x2_uint4x2_8(
        v: crate::Unity::Mathematics::uint4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x2> {
        let __cordl_ret: crate::Unity::Mathematics::float4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x3__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x3> {
        let __cordl_ret: crate::Unity::Mathematics::float4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x3_bool4x3_4(
        v: crate::Unity::Mathematics::bool4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x3> {
        let __cordl_ret: crate::Unity::Mathematics::float4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x3_double4x3_10(
        v: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x3> {
        let __cordl_ret: crate::Unity::Mathematics::float4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x3_f32_2(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x3> {
        let __cordl_ret: crate::Unity::Mathematics::float4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x3_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_1(
        m00: f32,
        m01: f32,
        m02: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m20: f32,
        m21: f32,
        m22: f32,
        m30: f32,
        m31: f32,
        m32: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x3> {
        let __cordl_ret: crate::Unity::Mathematics::float4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "float4x3",
                (m00, m01, m02, m10, m11, m12, m20, m21, m22, m30, m31, m32),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x3_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x3> {
        let __cordl_ret: crate::Unity::Mathematics::float4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x3_float4_float4_float4_0(
        c0: crate::Unity::Mathematics::float4,
        c1: crate::Unity::Mathematics::float4,
        c2: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x3> {
        let __cordl_ret: crate::Unity::Mathematics::float4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x3", (c0, c1, c2))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x3_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x3> {
        let __cordl_ret: crate::Unity::Mathematics::float4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x3_int4x3_6(
        v: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x3> {
        let __cordl_ret: crate::Unity::Mathematics::float4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x3_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x3> {
        let __cordl_ret: crate::Unity::Mathematics::float4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x3_uint4x3_8(
        v: crate::Unity::Mathematics::uint4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x3> {
        let __cordl_ret: crate::Unity::Mathematics::float4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x4_RigidTransform13(
        transform: crate::Unity::Mathematics::RigidTransform,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x4", (transform))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x4__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x4_bool4x4_4(
        v: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x4_double4x4_10(
        v: crate::Unity::Mathematics::double4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x4_f32_2(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x4_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_f32_1(
        m00: f32,
        m01: f32,
        m02: f32,
        m03: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m13: f32,
        m20: f32,
        m21: f32,
        m22: f32,
        m23: f32,
        m30: f32,
        m31: f32,
        m32: f32,
        m33: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "float4x4",
                (
                    m00,
                    m01,
                    m02,
                    m03,
                    m10,
                    m11,
                    m12,
                    m13,
                    m20,
                    m21,
                    m22,
                    m23,
                    m30,
                    m31,
                    m32,
                    m33,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x4_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x4_float3x3_float3_11(
        rotation: crate::Unity::Mathematics::float3x3,
        translation: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x4", (rotation, translation))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x4_float4_float4_float4_float4_0(
        c0: crate::Unity::Mathematics::float4,
        c1: crate::Unity::Mathematics::float4,
        c2: crate::Unity::Mathematics::float4,
        c3: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x4", (c0, c1, c2, c3))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x4_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x4_int4x4_6(
        v: crate::Unity::Mathematics::int4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x4_quaternion_float3_12(
        rotation: crate::Unity::Mathematics::quaternion,
        translation: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x4", (rotation, translation))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x4_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn float4x4_uint4x4_8(
        v: crate::Unity::Mathematics::uint4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("float4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn floor_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("floor", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn floor_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("floor", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn floor_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("floor", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn floor_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("floor", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn floor_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("floor", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn floor_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("floor", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn floor_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("floor", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn floor_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("floor", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn floorlog2_i32_0(x: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("floorlog2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn floorlog2_int2_1(
        x: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("floorlog2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn floorlog2_int3_2(
        x: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("floorlog2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn floorlog2_int4_3(
        x: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("floorlog2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn floorlog2_u32_4(x: u32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("floorlog2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn floorlog2_uint2_5(
        x: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("floorlog2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn floorlog2_uint3_6(
        x: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("floorlog2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn floorlog2_uint4_7(
        x: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("floorlog2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn fmod_double2_double2_5(
        x: crate::Unity::Mathematics::double2,
        y: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fmod", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn fmod_double3_double3_6(
        x: crate::Unity::Mathematics::double3,
        y: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fmod", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn fmod_double4_double4_7(
        x: crate::Unity::Mathematics::double4,
        y: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fmod", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn fmod_f32_f32_0(x: f32, y: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fmod", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn fmod_f64_f64_4(x: f64, y: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fmod", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn fmod_float2_float2_1(
        x: crate::Unity::Mathematics::float2,
        y: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fmod", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn fmod_float3_float3_2(
        x: crate::Unity::Mathematics::float3,
        y: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fmod", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn fmod_float4_float4_3(
        x: crate::Unity::Mathematics::float4,
        y: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fmod", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn fold_to_uint_double2_1(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fold_to_uint", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn fold_to_uint_double3_2(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fold_to_uint", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn fold_to_uint_double4_3(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fold_to_uint", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn fold_to_uint_f64_0(x: f64) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fold_to_uint", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn forward_0() -> quest_hook::libil2cpp::Result<
        crate::Unity::Mathematics::float3,
    > {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("forward", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn forward_quaternion1(
        q: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("forward", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn frac_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("frac", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn frac_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("frac", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn frac_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("frac", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn frac_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("frac", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn frac_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("frac", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn frac_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("frac", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn frac_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("frac", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn frac_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("frac", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn half2_double2_6(
        v: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn half2_f32_3(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn half2_f64_5(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn half2_float2_4(
        v: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn half2_half2(
        v: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn half2_half2_1(
        xy: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half2", (xy))?;
        Ok(__cordl_ret.into())
    }
    pub fn half2_half_half0(
        x: crate::Unity::Mathematics::half,
        y: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half2> {
        let __cordl_ret: crate::Unity::Mathematics::half2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half2", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn half3_double3_8(
        v: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn half3_f32_5(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn half3_f64_7(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn half3_float3_6(
        v: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn half3_half2_half2(
        xy: crate::Unity::Mathematics::half2,
        z: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half3", (xy, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn half3_half3_3(
        xyz: crate::Unity::Mathematics::half3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half3", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn half3_half4(
        v: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn half3_half_half2_1(
        x: crate::Unity::Mathematics::half,
        yz: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half3", (x, yz))?;
        Ok(__cordl_ret.into())
    }
    pub fn half3_half_half_half0(
        x: crate::Unity::Mathematics::half,
        y: crate::Unity::Mathematics::half,
        z: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half3> {
        let __cordl_ret: crate::Unity::Mathematics::half3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half3", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn half4_double4_12(
        v: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn half4_f32_9(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn half4_f64_11(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn half4_float4_10(
        v: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn half4_half2_half2_5(
        xy: crate::Unity::Mathematics::half2,
        zw: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half4", (xy, zw))?;
        Ok(__cordl_ret.into())
    }
    pub fn half4_half2_half_half4(
        xy: crate::Unity::Mathematics::half2,
        z: crate::Unity::Mathematics::half,
        w: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half4", (xy, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn half4_half3_half6(
        xyz: crate::Unity::Mathematics::half3,
        w: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half4", (xyz, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn half4_half4_7(
        xyzw: crate::Unity::Mathematics::half4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half4", (xyzw))?;
        Ok(__cordl_ret.into())
    }
    pub fn half4_half8(
        v: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn half4_half_half2_half2(
        x: crate::Unity::Mathematics::half,
        yz: crate::Unity::Mathematics::half2,
        w: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half4", (x, yz, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn half4_half_half3_3(
        x: crate::Unity::Mathematics::half,
        yzw: crate::Unity::Mathematics::half3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half4", (x, yzw))?;
        Ok(__cordl_ret.into())
    }
    pub fn half4_half_half_half2_1(
        x: crate::Unity::Mathematics::half,
        y: crate::Unity::Mathematics::half,
        zw: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half4", (x, y, zw))?;
        Ok(__cordl_ret.into())
    }
    pub fn half4_half_half_half_half0(
        x: crate::Unity::Mathematics::half,
        y: crate::Unity::Mathematics::half,
        z: crate::Unity::Mathematics::half,
        w: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half4> {
        let __cordl_ret: crate::Unity::Mathematics::half4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half4", (x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn half_f32_1(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half> {
        let __cordl_ret: crate::Unity::Mathematics::half = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn half_f64_2(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half> {
        let __cordl_ret: crate::Unity::Mathematics::half = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn half_half0(
        x: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half> {
        let __cordl_ret: crate::Unity::Mathematics::half = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("half", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_Il2CppObject_i32_u32_52(
        pBuffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        numBytes: i32,
        seed: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (pBuffer, numBytes, seed))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_RigidTransform54(
        t: crate::Unity::Mathematics::RigidTransform,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_bool2_0(
        v: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_bool2x2_1(
        v: crate::Unity::Mathematics::bool2x2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_bool2x3_2(
        v: crate::Unity::Mathematics::bool2x3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_bool2x4_3(
        v: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_bool3_4(
        v: crate::Unity::Mathematics::bool3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_bool3x2_5(
        v: crate::Unity::Mathematics::bool3x2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_bool3x3_6(
        v: crate::Unity::Mathematics::bool3x3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_bool3x4_7(
        v: crate::Unity::Mathematics::bool3x4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_bool4_8(
        v: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_bool4x2_9(
        v: crate::Unity::Mathematics::bool4x2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_bool4x3_10(
        v: crate::Unity::Mathematics::bool4x3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_bool4x4_11(
        v: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_double2_12(
        v: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_double2x2_13(
        v: crate::Unity::Mathematics::double2x2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_double2x3_14(
        v: crate::Unity::Mathematics::double2x3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_double2x4_15(
        v: crate::Unity::Mathematics::double2x4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_double3_16(
        v: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_double3x2_17(
        v: crate::Unity::Mathematics::double3x2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_double3x3_18(
        v: crate::Unity::Mathematics::double3x3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_double3x4_19(
        v: crate::Unity::Mathematics::double3x4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_double4_20(
        v: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_double4x2_21(
        v: crate::Unity::Mathematics::double4x2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_double4x3_22(
        v: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_double4x4_23(
        v: crate::Unity::Mathematics::double4x4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_float2_24(
        v: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_float2x2_25(
        v: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_float2x3_26(
        v: crate::Unity::Mathematics::float2x3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_float2x4_27(
        v: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_float3_28(
        v: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_float3x2_29(
        v: crate::Unity::Mathematics::float3x2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_float3x3_30(
        v: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_float3x4_31(
        v: crate::Unity::Mathematics::float3x4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_float4_32(
        v: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_float4x2_33(
        v: crate::Unity::Mathematics::float4x2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_float4x3_34(
        v: crate::Unity::Mathematics::float4x3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_float4x4_35(
        v: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_half2_37(
        v: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_half36(
        v: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_half3_38(
        v: crate::Unity::Mathematics::half3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_half4_39(
        v: crate::Unity::Mathematics::half4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_int2_40(
        v: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_int2x2_41(
        v: crate::Unity::Mathematics::int2x2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_int2x3_42(
        v: crate::Unity::Mathematics::int2x3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_int2x4_43(
        v: crate::Unity::Mathematics::int2x4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_int3_44(
        v: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_int3x2_45(
        v: crate::Unity::Mathematics::int3x2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_int3x3_46(
        v: crate::Unity::Mathematics::int3x3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_int3x4_47(
        v: crate::Unity::Mathematics::int3x4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_int4_48(
        v: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_int4x2_49(
        v: crate::Unity::Mathematics::int4x2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_int4x3_50(
        v: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_int4x4_51(
        v: crate::Unity::Mathematics::int4x4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_quaternion53(
        q: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_uint2_55(
        v: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_uint2x2_56(
        v: crate::Unity::Mathematics::uint2x2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_uint2x3_57(
        v: crate::Unity::Mathematics::uint2x3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_uint2x4_58(
        v: crate::Unity::Mathematics::uint2x4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_uint3_59(
        v: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_uint3x2_60(
        v: crate::Unity::Mathematics::uint3x2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_uint3x3_61(
        v: crate::Unity::Mathematics::uint3x3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_uint3x4_62(
        v: crate::Unity::Mathematics::uint3x4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_uint4_63(
        v: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_uint4x2_64(
        v: crate::Unity::Mathematics::uint4x2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_uint4x3_65(
        v: crate::Unity::Mathematics::uint4x3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hash_uint4x4_66(
        v: crate::Unity::Mathematics::uint4x4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hash", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_RigidTransform52(
        t: crate::Unity::Mathematics::RigidTransform,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_bool2_0(
        v: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_bool2x2_1(
        v: crate::Unity::Mathematics::bool2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_bool2x3_2(
        v: crate::Unity::Mathematics::bool2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_bool2x4_3(
        v: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_bool3_4(
        v: crate::Unity::Mathematics::bool3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_bool3x2_5(
        v: crate::Unity::Mathematics::bool3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_bool3x3_6(
        v: crate::Unity::Mathematics::bool3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_bool3x4_7(
        v: crate::Unity::Mathematics::bool3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_bool4_8(
        v: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_bool4x2_9(
        v: crate::Unity::Mathematics::bool4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_bool4x3_10(
        v: crate::Unity::Mathematics::bool4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_bool4x4_11(
        v: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_double2_12(
        v: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_double2x2_13(
        v: crate::Unity::Mathematics::double2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_double2x3_14(
        v: crate::Unity::Mathematics::double2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_double2x4_15(
        v: crate::Unity::Mathematics::double2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_double3_16(
        v: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_double3x2_17(
        v: crate::Unity::Mathematics::double3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_double3x3_18(
        v: crate::Unity::Mathematics::double3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_double3x4_19(
        v: crate::Unity::Mathematics::double3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_double4_20(
        v: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_double4x2_21(
        v: crate::Unity::Mathematics::double4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_double4x3_22(
        v: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_double4x4_23(
        v: crate::Unity::Mathematics::double4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_float2_24(
        v: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_float2x2_25(
        v: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_float2x3_26(
        v: crate::Unity::Mathematics::float2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_float2x4_27(
        v: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_float3_28(
        v: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_float3x2_29(
        v: crate::Unity::Mathematics::float3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_float3x3_30(
        v: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_float3x4_31(
        v: crate::Unity::Mathematics::float3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_float4_32(
        v: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_float4x2_33(
        v: crate::Unity::Mathematics::float4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_float4x3_34(
        v: crate::Unity::Mathematics::float4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_float4x4_35(
        v: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_half2_36(
        v: crate::Unity::Mathematics::half2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_half3_37(
        v: crate::Unity::Mathematics::half3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_half4_38(
        v: crate::Unity::Mathematics::half4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_int2_39(
        v: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_int2x2_40(
        v: crate::Unity::Mathematics::int2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_int2x3_41(
        v: crate::Unity::Mathematics::int2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_int2x4_42(
        v: crate::Unity::Mathematics::int2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_int3_43(
        v: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_int3x2_44(
        v: crate::Unity::Mathematics::int3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_int3x3_45(
        v: crate::Unity::Mathematics::int3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_int3x4_46(
        v: crate::Unity::Mathematics::int3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_int4_47(
        v: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_int4x2_48(
        v: crate::Unity::Mathematics::int4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_int4x3_49(
        v: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_int4x4_50(
        v: crate::Unity::Mathematics::int4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_quaternion51(
        q: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_uint2_53(
        v: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_uint2x2_54(
        v: crate::Unity::Mathematics::uint2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_uint2x3_55(
        v: crate::Unity::Mathematics::uint2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_uint2x4_56(
        v: crate::Unity::Mathematics::uint2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_uint3_57(
        v: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_uint3x2_58(
        v: crate::Unity::Mathematics::uint3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_uint3x3_59(
        v: crate::Unity::Mathematics::uint3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_uint3x4_60(
        v: crate::Unity::Mathematics::uint3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_uint4_61(
        v: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_uint4x2_62(
        v: crate::Unity::Mathematics::uint4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_uint4x3_63(
        v: crate::Unity::Mathematics::uint4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn hashwide_uint4x4_64(
        v: crate::Unity::Mathematics::uint4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("hashwide", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2_bool2_4(
        v: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2_double2_10(
        v: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2_float2_8(
        v: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2_i32_2(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2_i32_i32_0(
        x: i32,
        y: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2_int2_1(
        xy: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2", (xy))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2_u32_5(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2_uint2_6(
        v: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x2__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x2> {
        let __cordl_ret: crate::Unity::Mathematics::int2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x2_bool2x2_4(
        v: crate::Unity::Mathematics::bool2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x2> {
        let __cordl_ret: crate::Unity::Mathematics::int2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x2_double2x2_10(
        v: crate::Unity::Mathematics::double2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x2> {
        let __cordl_ret: crate::Unity::Mathematics::int2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x2_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x2> {
        let __cordl_ret: crate::Unity::Mathematics::int2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x2_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x2> {
        let __cordl_ret: crate::Unity::Mathematics::int2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x2_float2x2_8(
        v: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x2> {
        let __cordl_ret: crate::Unity::Mathematics::int2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x2_i32_2(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x2> {
        let __cordl_ret: crate::Unity::Mathematics::int2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x2_i32_i32_i32_i32_1(
        m00: i32,
        m01: i32,
        m10: i32,
        m11: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x2> {
        let __cordl_ret: crate::Unity::Mathematics::int2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x2", (m00, m01, m10, m11))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x2_int2_int2_0(
        c0: crate::Unity::Mathematics::int2,
        c1: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x2> {
        let __cordl_ret: crate::Unity::Mathematics::int2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x2", (c0, c1))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x2_u32_5(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x2> {
        let __cordl_ret: crate::Unity::Mathematics::int2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x2_uint2x2_6(
        v: crate::Unity::Mathematics::uint2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x2> {
        let __cordl_ret: crate::Unity::Mathematics::int2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x3__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x3> {
        let __cordl_ret: crate::Unity::Mathematics::int2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x3_bool2x3_4(
        v: crate::Unity::Mathematics::bool2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x3> {
        let __cordl_ret: crate::Unity::Mathematics::int2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x3_double2x3_10(
        v: crate::Unity::Mathematics::double2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x3> {
        let __cordl_ret: crate::Unity::Mathematics::int2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x3_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x3> {
        let __cordl_ret: crate::Unity::Mathematics::int2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x3_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x3> {
        let __cordl_ret: crate::Unity::Mathematics::int2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x3_float2x3_8(
        v: crate::Unity::Mathematics::float2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x3> {
        let __cordl_ret: crate::Unity::Mathematics::int2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x3_i32_2(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x3> {
        let __cordl_ret: crate::Unity::Mathematics::int2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x3_i32_i32_i32_i32_i32_i32_1(
        m00: i32,
        m01: i32,
        m02: i32,
        m10: i32,
        m11: i32,
        m12: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x3> {
        let __cordl_ret: crate::Unity::Mathematics::int2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x3", (m00, m01, m02, m10, m11, m12))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x3_int2_int2_int2_0(
        c0: crate::Unity::Mathematics::int2,
        c1: crate::Unity::Mathematics::int2,
        c2: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x3> {
        let __cordl_ret: crate::Unity::Mathematics::int2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x3", (c0, c1, c2))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x3_u32_5(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x3> {
        let __cordl_ret: crate::Unity::Mathematics::int2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x3_uint2x3_6(
        v: crate::Unity::Mathematics::uint2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x3> {
        let __cordl_ret: crate::Unity::Mathematics::int2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x4__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x4> {
        let __cordl_ret: crate::Unity::Mathematics::int2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x4_bool2x4_4(
        v: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x4> {
        let __cordl_ret: crate::Unity::Mathematics::int2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x4_double2x4_10(
        v: crate::Unity::Mathematics::double2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x4> {
        let __cordl_ret: crate::Unity::Mathematics::int2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x4_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x4> {
        let __cordl_ret: crate::Unity::Mathematics::int2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x4_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x4> {
        let __cordl_ret: crate::Unity::Mathematics::int2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x4_float2x4_8(
        v: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x4> {
        let __cordl_ret: crate::Unity::Mathematics::int2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x4_i32_2(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x4> {
        let __cordl_ret: crate::Unity::Mathematics::int2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x4_i32_i32_i32_i32_i32_i32_i32_i32_1(
        m00: i32,
        m01: i32,
        m02: i32,
        m03: i32,
        m10: i32,
        m11: i32,
        m12: i32,
        m13: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x4> {
        let __cordl_ret: crate::Unity::Mathematics::int2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x4", (m00, m01, m02, m03, m10, m11, m12, m13))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x4_int2_int2_int2_int2_0(
        c0: crate::Unity::Mathematics::int2,
        c1: crate::Unity::Mathematics::int2,
        c2: crate::Unity::Mathematics::int2,
        c3: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x4> {
        let __cordl_ret: crate::Unity::Mathematics::int2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x4", (c0, c1, c2, c3))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x4_u32_5(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x4> {
        let __cordl_ret: crate::Unity::Mathematics::int2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int2x4_uint2x4_6(
        v: crate::Unity::Mathematics::uint2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x4> {
        let __cordl_ret: crate::Unity::Mathematics::int2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3__cordl_bool5(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3_bool3_6(
        v: crate::Unity::Mathematics::bool3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3_double3_12(
        v: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3_f32_9(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3_f64_11(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3_float3_10(
        v: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3_i32_4(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3_i32_i32_i32_0(
        x: i32,
        y: i32,
        z: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3_i32_int2_1(
        x: i32,
        yz: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3", (x, yz))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3_int2_i32_2(
        xy: crate::Unity::Mathematics::int2,
        z: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3", (xy, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3_int3_3(
        xyz: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3_u32_7(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3_uint3_8(
        v: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x2__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x2> {
        let __cordl_ret: crate::Unity::Mathematics::int3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x2_bool3x2_4(
        v: crate::Unity::Mathematics::bool3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x2> {
        let __cordl_ret: crate::Unity::Mathematics::int3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x2_double3x2_10(
        v: crate::Unity::Mathematics::double3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x2> {
        let __cordl_ret: crate::Unity::Mathematics::int3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x2_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x2> {
        let __cordl_ret: crate::Unity::Mathematics::int3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x2_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x2> {
        let __cordl_ret: crate::Unity::Mathematics::int3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x2_float3x2_8(
        v: crate::Unity::Mathematics::float3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x2> {
        let __cordl_ret: crate::Unity::Mathematics::int3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x2_i32_2(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x2> {
        let __cordl_ret: crate::Unity::Mathematics::int3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x2_i32_i32_i32_i32_i32_i32_1(
        m00: i32,
        m01: i32,
        m10: i32,
        m11: i32,
        m20: i32,
        m21: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x2> {
        let __cordl_ret: crate::Unity::Mathematics::int3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x2", (m00, m01, m10, m11, m20, m21))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x2_int3_int3_0(
        c0: crate::Unity::Mathematics::int3,
        c1: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x2> {
        let __cordl_ret: crate::Unity::Mathematics::int3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x2", (c0, c1))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x2_u32_5(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x2> {
        let __cordl_ret: crate::Unity::Mathematics::int3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x2_uint3x2_6(
        v: crate::Unity::Mathematics::uint3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x2> {
        let __cordl_ret: crate::Unity::Mathematics::int3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x3__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x3> {
        let __cordl_ret: crate::Unity::Mathematics::int3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x3_bool3x3_4(
        v: crate::Unity::Mathematics::bool3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x3> {
        let __cordl_ret: crate::Unity::Mathematics::int3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x3_double3x3_10(
        v: crate::Unity::Mathematics::double3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x3> {
        let __cordl_ret: crate::Unity::Mathematics::int3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x3_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x3> {
        let __cordl_ret: crate::Unity::Mathematics::int3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x3_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x3> {
        let __cordl_ret: crate::Unity::Mathematics::int3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x3_float3x3_8(
        v: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x3> {
        let __cordl_ret: crate::Unity::Mathematics::int3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x3_i32_2(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x3> {
        let __cordl_ret: crate::Unity::Mathematics::int3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x3_i32_i32_i32_i32_i32_i32_i32_i32_i32_1(
        m00: i32,
        m01: i32,
        m02: i32,
        m10: i32,
        m11: i32,
        m12: i32,
        m20: i32,
        m21: i32,
        m22: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x3> {
        let __cordl_ret: crate::Unity::Mathematics::int3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x3", (m00, m01, m02, m10, m11, m12, m20, m21, m22))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x3_int3_int3_int3_0(
        c0: crate::Unity::Mathematics::int3,
        c1: crate::Unity::Mathematics::int3,
        c2: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x3> {
        let __cordl_ret: crate::Unity::Mathematics::int3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x3", (c0, c1, c2))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x3_u32_5(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x3> {
        let __cordl_ret: crate::Unity::Mathematics::int3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x3_uint3x3_6(
        v: crate::Unity::Mathematics::uint3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x3> {
        let __cordl_ret: crate::Unity::Mathematics::int3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x4__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x4> {
        let __cordl_ret: crate::Unity::Mathematics::int3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x4_bool3x4_4(
        v: crate::Unity::Mathematics::bool3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x4> {
        let __cordl_ret: crate::Unity::Mathematics::int3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x4_double3x4_10(
        v: crate::Unity::Mathematics::double3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x4> {
        let __cordl_ret: crate::Unity::Mathematics::int3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x4_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x4> {
        let __cordl_ret: crate::Unity::Mathematics::int3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x4_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x4> {
        let __cordl_ret: crate::Unity::Mathematics::int3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x4_float3x4_8(
        v: crate::Unity::Mathematics::float3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x4> {
        let __cordl_ret: crate::Unity::Mathematics::int3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x4_i32_2(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x4> {
        let __cordl_ret: crate::Unity::Mathematics::int3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x4_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_1(
        m00: i32,
        m01: i32,
        m02: i32,
        m03: i32,
        m10: i32,
        m11: i32,
        m12: i32,
        m13: i32,
        m20: i32,
        m21: i32,
        m22: i32,
        m23: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x4> {
        let __cordl_ret: crate::Unity::Mathematics::int3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "int3x4",
                (m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x4_int3_int3_int3_int3_0(
        c0: crate::Unity::Mathematics::int3,
        c1: crate::Unity::Mathematics::int3,
        c2: crate::Unity::Mathematics::int3,
        c3: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x4> {
        let __cordl_ret: crate::Unity::Mathematics::int3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x4", (c0, c1, c2, c3))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x4_u32_5(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x4> {
        let __cordl_ret: crate::Unity::Mathematics::int3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int3x4_uint3x4_6(
        v: crate::Unity::Mathematics::uint3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x4> {
        let __cordl_ret: crate::Unity::Mathematics::int3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4__cordl_bool9(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4_bool4_10(
        v: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4_double4_16(
        v: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4_f32_13(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4_f64_15(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4_float4_14(
        v: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4_i32_8(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4_i32_i32_i32_i32_0(
        x: i32,
        y: i32,
        z: i32,
        w: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4", (x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4_i32_i32_int2_1(
        x: i32,
        y: i32,
        zw: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4", (x, y, zw))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4_i32_int2_i32_2(
        x: i32,
        yz: crate::Unity::Mathematics::int2,
        w: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4", (x, yz, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4_i32_int3_3(
        x: i32,
        yzw: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4", (x, yzw))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4_int2_i32_i32_4(
        xy: crate::Unity::Mathematics::int2,
        z: i32,
        w: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4", (xy, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4_int2_int2_5(
        xy: crate::Unity::Mathematics::int2,
        zw: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4", (xy, zw))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4_int3_i32_6(
        xyz: crate::Unity::Mathematics::int3,
        w: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4", (xyz, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4_int4_7(
        xyzw: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4", (xyzw))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4_u32_11(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4_uint4_12(
        v: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x2__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x2> {
        let __cordl_ret: crate::Unity::Mathematics::int4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x2_bool4x2_4(
        v: crate::Unity::Mathematics::bool4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x2> {
        let __cordl_ret: crate::Unity::Mathematics::int4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x2_double4x2_10(
        v: crate::Unity::Mathematics::double4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x2> {
        let __cordl_ret: crate::Unity::Mathematics::int4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x2_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x2> {
        let __cordl_ret: crate::Unity::Mathematics::int4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x2_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x2> {
        let __cordl_ret: crate::Unity::Mathematics::int4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x2_float4x2_8(
        v: crate::Unity::Mathematics::float4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x2> {
        let __cordl_ret: crate::Unity::Mathematics::int4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x2_i32_2(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x2> {
        let __cordl_ret: crate::Unity::Mathematics::int4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x2_i32_i32_i32_i32_i32_i32_i32_i32_1(
        m00: i32,
        m01: i32,
        m10: i32,
        m11: i32,
        m20: i32,
        m21: i32,
        m30: i32,
        m31: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x2> {
        let __cordl_ret: crate::Unity::Mathematics::int4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x2", (m00, m01, m10, m11, m20, m21, m30, m31))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x2_int4_int4_0(
        c0: crate::Unity::Mathematics::int4,
        c1: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x2> {
        let __cordl_ret: crate::Unity::Mathematics::int4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x2", (c0, c1))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x2_u32_5(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x2> {
        let __cordl_ret: crate::Unity::Mathematics::int4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x2_uint4x2_6(
        v: crate::Unity::Mathematics::uint4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x2> {
        let __cordl_ret: crate::Unity::Mathematics::int4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x3__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x3_bool4x3_4(
        v: crate::Unity::Mathematics::bool4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x3_double4x3_10(
        v: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x3_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x3_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x3_float4x3_8(
        v: crate::Unity::Mathematics::float4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x3_i32_2(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x3_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_1(
        m00: i32,
        m01: i32,
        m02: i32,
        m10: i32,
        m11: i32,
        m12: i32,
        m20: i32,
        m21: i32,
        m22: i32,
        m30: i32,
        m31: i32,
        m32: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "int4x3",
                (m00, m01, m02, m10, m11, m12, m20, m21, m22, m30, m31, m32),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x3_int4_int4_int4_0(
        c0: crate::Unity::Mathematics::int4,
        c1: crate::Unity::Mathematics::int4,
        c2: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x3", (c0, c1, c2))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x3_u32_5(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x3_uint4x3_6(
        v: crate::Unity::Mathematics::uint4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x4__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x4> {
        let __cordl_ret: crate::Unity::Mathematics::int4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x4_bool4x4_4(
        v: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x4> {
        let __cordl_ret: crate::Unity::Mathematics::int4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x4_double4x4_10(
        v: crate::Unity::Mathematics::double4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x4> {
        let __cordl_ret: crate::Unity::Mathematics::int4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x4_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x4> {
        let __cordl_ret: crate::Unity::Mathematics::int4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x4_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x4> {
        let __cordl_ret: crate::Unity::Mathematics::int4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x4_float4x4_8(
        v: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x4> {
        let __cordl_ret: crate::Unity::Mathematics::int4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x4_i32_2(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x4> {
        let __cordl_ret: crate::Unity::Mathematics::int4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x4_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_i32_1(
        m00: i32,
        m01: i32,
        m02: i32,
        m03: i32,
        m10: i32,
        m11: i32,
        m12: i32,
        m13: i32,
        m20: i32,
        m21: i32,
        m22: i32,
        m23: i32,
        m30: i32,
        m31: i32,
        m32: i32,
        m33: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x4> {
        let __cordl_ret: crate::Unity::Mathematics::int4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "int4x4",
                (
                    m00,
                    m01,
                    m02,
                    m03,
                    m10,
                    m11,
                    m12,
                    m13,
                    m20,
                    m21,
                    m22,
                    m23,
                    m30,
                    m31,
                    m32,
                    m33,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x4_int4_int4_int4_int4_0(
        c0: crate::Unity::Mathematics::int4,
        c1: crate::Unity::Mathematics::int4,
        c2: crate::Unity::Mathematics::int4,
        c3: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x4> {
        let __cordl_ret: crate::Unity::Mathematics::int4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x4", (c0, c1, c2, c3))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x4_u32_5(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x4> {
        let __cordl_ret: crate::Unity::Mathematics::int4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn int4x4_uint4x4_6(
        v: crate::Unity::Mathematics::uint4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x4> {
        let __cordl_ret: crate::Unity::Mathematics::int4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("int4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn inverse_RigidTransform7(
        t: crate::Unity::Mathematics::RigidTransform,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("inverse", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn inverse_double2x2_0(
        m: crate::Unity::Mathematics::double2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x2> {
        let __cordl_ret: crate::Unity::Mathematics::double2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("inverse", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn inverse_double3x3_1(
        m: crate::Unity::Mathematics::double3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x3> {
        let __cordl_ret: crate::Unity::Mathematics::double3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("inverse", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn inverse_double4x4_2(
        m: crate::Unity::Mathematics::double4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x4> {
        let __cordl_ret: crate::Unity::Mathematics::double4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("inverse", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn inverse_float2x2_3(
        m: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("inverse", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn inverse_float3x3_4(
        m: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("inverse", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn inverse_float4x4_5(
        m: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("inverse", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn inverse_quaternion6(
        q: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("inverse", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn isfinite_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isfinite", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isfinite_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isfinite", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isfinite_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isfinite", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isfinite_f32_0(x: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isfinite", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isfinite_f64_4(x: f64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isfinite", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isfinite_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isfinite", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isfinite_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isfinite", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isfinite_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isfinite", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isinf_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isinf", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isinf_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isinf", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isinf_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isinf", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isinf_f32_0(x: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isinf", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isinf_f64_4(x: f64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isinf", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isinf_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isinf", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isinf_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isinf", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isinf_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isinf", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isnan_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isnan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isnan_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isnan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isnan_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isnan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isnan_f32_0(x: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isnan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isnan_f64_4(x: f64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isnan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isnan_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isnan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isnan_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isnan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn isnan_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("isnan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ispow2_i32_0(x: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ispow2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ispow2_int2_1(
        x: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ispow2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ispow2_int3_2(
        x: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ispow2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ispow2_int4_3(
        x: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ispow2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ispow2_u32_4(x: u32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ispow2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ispow2_uint2_5(
        x: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ispow2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ispow2_uint3_6(
        x: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ispow2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn ispow2_uint4_7(
        x: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ispow2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn left() -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("left", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn length_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("length", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn length_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("length", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn length_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("length", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn length_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("length", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn length_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("length", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn length_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("length", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn length_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("length", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn length_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("length", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn length_quaternion8(
        q: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("length", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn lengthsq_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lengthsq", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn lengthsq_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lengthsq", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn lengthsq_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lengthsq", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn lengthsq_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lengthsq", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn lengthsq_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lengthsq", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn lengthsq_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lengthsq", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn lengthsq_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lengthsq", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn lengthsq_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lengthsq", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn lengthsq_quaternion8(
        q: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lengthsq", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn lerp_double2_double2_double2_11(
        x: crate::Unity::Mathematics::double2,
        y: crate::Unity::Mathematics::double2,
        s: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lerp", (x, y, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn lerp_double2_double2_f64_8(
        x: crate::Unity::Mathematics::double2,
        y: crate::Unity::Mathematics::double2,
        s: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lerp", (x, y, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn lerp_double3_double3_double3_12(
        x: crate::Unity::Mathematics::double3,
        y: crate::Unity::Mathematics::double3,
        s: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lerp", (x, y, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn lerp_double3_double3_f64_9(
        x: crate::Unity::Mathematics::double3,
        y: crate::Unity::Mathematics::double3,
        s: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lerp", (x, y, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn lerp_double4_double4_double4_13(
        x: crate::Unity::Mathematics::double4,
        y: crate::Unity::Mathematics::double4,
        s: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lerp", (x, y, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn lerp_double4_double4_f64_10(
        x: crate::Unity::Mathematics::double4,
        y: crate::Unity::Mathematics::double4,
        s: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lerp", (x, y, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn lerp_f32_f32_f32_0(
        x: f32,
        y: f32,
        s: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lerp", (x, y, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn lerp_f64_f64_f64_7(
        x: f64,
        y: f64,
        s: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lerp", (x, y, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn lerp_float2_float2_f32_1(
        x: crate::Unity::Mathematics::float2,
        y: crate::Unity::Mathematics::float2,
        s: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lerp", (x, y, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn lerp_float2_float2_float2_4(
        x: crate::Unity::Mathematics::float2,
        y: crate::Unity::Mathematics::float2,
        s: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lerp", (x, y, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn lerp_float3_float3_f32_2(
        x: crate::Unity::Mathematics::float3,
        y: crate::Unity::Mathematics::float3,
        s: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lerp", (x, y, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn lerp_float3_float3_float3_5(
        x: crate::Unity::Mathematics::float3,
        y: crate::Unity::Mathematics::float3,
        s: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lerp", (x, y, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn lerp_float4_float4_f32_3(
        x: crate::Unity::Mathematics::float4,
        y: crate::Unity::Mathematics::float4,
        s: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lerp", (x, y, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn lerp_float4_float4_float4_6(
        x: crate::Unity::Mathematics::float4,
        y: crate::Unity::Mathematics::float4,
        s: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lerp", (x, y, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn log10_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log10", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log10_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log10", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log10_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log10", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log10_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log10", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log10_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log10", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log10_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log10", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log10_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log10", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log10_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log10", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log2_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log2_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log2_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log2_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log2_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log2_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log2_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log2_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log2", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn log_quaternion8(
        q: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("log", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn lzcnt_i32_0(x: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lzcnt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn lzcnt_i64_8(x: i64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lzcnt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn lzcnt_int2_1(
        x: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lzcnt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn lzcnt_int3_2(
        x: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lzcnt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn lzcnt_int4_3(
        x: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lzcnt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn lzcnt_u32_4(x: u32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lzcnt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn lzcnt_u64_9(x: u64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lzcnt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn lzcnt_uint2_5(
        x: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lzcnt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn lzcnt_uint3_6(
        x: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lzcnt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn lzcnt_uint4_7(
        x: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("lzcnt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn mad_double2_double2_double2_15(
        a: crate::Unity::Mathematics::double2,
        b: crate::Unity::Mathematics::double2,
        c: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mad", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mad_double3_double3_double3_16(
        a: crate::Unity::Mathematics::double3,
        b: crate::Unity::Mathematics::double3,
        c: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mad", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mad_double4_double4_double4_17(
        a: crate::Unity::Mathematics::double4,
        b: crate::Unity::Mathematics::double4,
        c: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mad", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mad_f32_f32_f32_10(
        a: f32,
        b: f32,
        c: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mad", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mad_f64_f64_f64_14(
        a: f64,
        b: f64,
        c: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mad", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mad_float2_float2_float2_11(
        a: crate::Unity::Mathematics::float2,
        b: crate::Unity::Mathematics::float2,
        c: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mad", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mad_float3_float3_float3_12(
        a: crate::Unity::Mathematics::float3,
        b: crate::Unity::Mathematics::float3,
        c: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mad", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mad_float4_float4_float4_13(
        a: crate::Unity::Mathematics::float4,
        b: crate::Unity::Mathematics::float4,
        c: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mad", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mad_i32_i32_i32_0(
        a: i32,
        b: i32,
        c: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mad", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mad_i64_i64_i64_8(
        a: i64,
        b: i64,
        c: i64,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mad", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mad_int2_int2_int2_1(
        a: crate::Unity::Mathematics::int2,
        b: crate::Unity::Mathematics::int2,
        c: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mad", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mad_int3_int3_int3_2(
        a: crate::Unity::Mathematics::int3,
        b: crate::Unity::Mathematics::int3,
        c: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mad", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mad_int4_int4_int4_3(
        a: crate::Unity::Mathematics::int4,
        b: crate::Unity::Mathematics::int4,
        c: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mad", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mad_u32_u32_u32_4(
        a: u32,
        b: u32,
        c: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mad", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mad_u64_u64_u64_9(
        a: u64,
        b: u64,
        c: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mad", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mad_uint2_uint2_uint2_5(
        a: crate::Unity::Mathematics::uint2,
        b: crate::Unity::Mathematics::uint2,
        c: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mad", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mad_uint3_uint3_uint3_6(
        a: crate::Unity::Mathematics::uint3,
        b: crate::Unity::Mathematics::uint3,
        c: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mad", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn mad_uint4_uint4_uint4_7(
        a: crate::Unity::Mathematics::uint4,
        b: crate::Unity::Mathematics::uint4,
        c: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mad", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_double2_double2_15(
        x: crate::Unity::Mathematics::double2,
        y: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_double3_double3_16(
        x: crate::Unity::Mathematics::double3,
        y: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_double4_double4_17(
        x: crate::Unity::Mathematics::double4,
        y: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_f32_f32_10(x: f32, y: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_f64_f64_14(x: f64, y: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_float2_float2_11(
        x: crate::Unity::Mathematics::float2,
        y: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_float3_float3_12(
        x: crate::Unity::Mathematics::float3,
        y: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_float4_float4_13(
        x: crate::Unity::Mathematics::float4,
        y: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_i32_i32_0(x: i32, y: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_i64_i64_8(x: i64, y: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_int2_int2_1(
        x: crate::Unity::Mathematics::int2,
        y: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_int3_int3_2(
        x: crate::Unity::Mathematics::int3,
        y: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_int4_int4_3(
        x: crate::Unity::Mathematics::int4,
        y: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_u32_u32_4(x: u32, y: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_u64_u64_9(x: u64, y: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_uint2_uint2_5(
        x: crate::Unity::Mathematics::uint2,
        y: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_uint3_uint3_6(
        x: crate::Unity::Mathematics::uint3,
        y: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn max_uint4_uint4_7(
        x: crate::Unity::Mathematics::uint4,
        y: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("max", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_double2_double2_15(
        x: crate::Unity::Mathematics::double2,
        y: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_double3_double3_16(
        x: crate::Unity::Mathematics::double3,
        y: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_double4_double4_17(
        x: crate::Unity::Mathematics::double4,
        y: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_f32_f32_10(x: f32, y: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_f64_f64_14(x: f64, y: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_float2_float2_11(
        x: crate::Unity::Mathematics::float2,
        y: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_float3_float3_12(
        x: crate::Unity::Mathematics::float3,
        y: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_float4_float4_13(
        x: crate::Unity::Mathematics::float4,
        y: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_i32_i32_0(x: i32, y: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_i64_i64_8(x: i64, y: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_int2_int2_1(
        x: crate::Unity::Mathematics::int2,
        y: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_int3_int3_2(
        x: crate::Unity::Mathematics::int3,
        y: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_int4_int4_3(
        x: crate::Unity::Mathematics::int4,
        y: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_u32_u32_4(x: u32, y: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_u64_u64_9(x: u64, y: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_uint2_uint2_5(
        x: crate::Unity::Mathematics::uint2,
        y: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_uint3_uint3_6(
        x: crate::Unity::Mathematics::uint3,
        y: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn min_uint4_uint4_7(
        x: crate::Unity::Mathematics::uint4,
        y: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("min", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn modf_double2_5(
        x: crate::Unity::Mathematics::double2,
        i: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::double2>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("modf", (x, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn modf_double3_6(
        x: crate::Unity::Mathematics::double3,
        i: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::double3>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("modf", (x, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn modf_double4_7(
        x: crate::Unity::Mathematics::double4,
        i: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::double4>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("modf", (x, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn modf_f32_0(
        x: f32,
        i: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("modf", (x, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn modf_f64_4(
        x: f64,
        i: quest_hook::libil2cpp::ByRefMut<f64>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("modf", (x, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn modf_float2_1(
        x: crate::Unity::Mathematics::float2,
        i: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float2>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("modf", (x, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn modf_float3_2(
        x: crate::Unity::Mathematics::float3,
        i: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float3>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("modf", (x, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn modf_float4_3(
        x: crate::Unity::Mathematics::float4,
        i: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float4>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("modf", (x, i))?;
        Ok(__cordl_ret.into())
    }
    pub fn movehl_double4_double4_1(
        a: crate::Unity::Mathematics::double4,
        b: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("movehl", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn movehl_float4_float4_0(
        a: crate::Unity::Mathematics::float4,
        b: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("movehl", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn movelh_double4_double4_1(
        a: crate::Unity::Mathematics::double4,
        b: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("movelh", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn movelh_float4_float4_0(
        a: crate::Unity::Mathematics::float4,
        b: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("movelh", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_RigidTransform_RigidTransform198(
        a: crate::Unity::Mathematics::RigidTransform,
        b: crate::Unity::Mathematics::RigidTransform,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::RigidTransform> {
        let __cordl_ret: crate::Unity::Mathematics::RigidTransform = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_RigidTransform_float4_199(
        a: crate::Unity::Mathematics::RigidTransform,
        pos: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double2_double2_50(
        a: crate::Unity::Mathematics::double2,
        b: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double2_double2x2_51(
        a: crate::Unity::Mathematics::double2,
        b: crate::Unity::Mathematics::double2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double2_double2x3_52(
        a: crate::Unity::Mathematics::double2,
        b: crate::Unity::Mathematics::double2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double2_double2x4_53(
        a: crate::Unity::Mathematics::double2,
        b: crate::Unity::Mathematics::double2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double2x2_double2_62(
        a: crate::Unity::Mathematics::double2x2,
        b: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double2x2_double2x2_63(
        a: crate::Unity::Mathematics::double2x2,
        b: crate::Unity::Mathematics::double2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x2> {
        let __cordl_ret: crate::Unity::Mathematics::double2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double2x2_double2x3_64(
        a: crate::Unity::Mathematics::double2x2,
        b: crate::Unity::Mathematics::double2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x3> {
        let __cordl_ret: crate::Unity::Mathematics::double2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double2x2_double2x4_65(
        a: crate::Unity::Mathematics::double2x2,
        b: crate::Unity::Mathematics::double2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x4> {
        let __cordl_ret: crate::Unity::Mathematics::double2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double2x3_double3_66(
        a: crate::Unity::Mathematics::double2x3,
        b: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double2x3_double3x2_67(
        a: crate::Unity::Mathematics::double2x3,
        b: crate::Unity::Mathematics::double3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x2> {
        let __cordl_ret: crate::Unity::Mathematics::double2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double2x3_double3x3_68(
        a: crate::Unity::Mathematics::double2x3,
        b: crate::Unity::Mathematics::double3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x3> {
        let __cordl_ret: crate::Unity::Mathematics::double2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double2x3_double3x4_69(
        a: crate::Unity::Mathematics::double2x3,
        b: crate::Unity::Mathematics::double3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x4> {
        let __cordl_ret: crate::Unity::Mathematics::double2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double2x4_double4_70(
        a: crate::Unity::Mathematics::double2x4,
        b: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double2x4_double4x2_71(
        a: crate::Unity::Mathematics::double2x4,
        b: crate::Unity::Mathematics::double4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x2> {
        let __cordl_ret: crate::Unity::Mathematics::double2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double2x4_double4x3_72(
        a: crate::Unity::Mathematics::double2x4,
        b: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x3> {
        let __cordl_ret: crate::Unity::Mathematics::double2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double2x4_double4x4_73(
        a: crate::Unity::Mathematics::double2x4,
        b: crate::Unity::Mathematics::double4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x4> {
        let __cordl_ret: crate::Unity::Mathematics::double2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double3_double3_54(
        a: crate::Unity::Mathematics::double3,
        b: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double3_double3x2_55(
        a: crate::Unity::Mathematics::double3,
        b: crate::Unity::Mathematics::double3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double3_double3x3_56(
        a: crate::Unity::Mathematics::double3,
        b: crate::Unity::Mathematics::double3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double3_double3x4_57(
        a: crate::Unity::Mathematics::double3,
        b: crate::Unity::Mathematics::double3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double3x2_double2_74(
        a: crate::Unity::Mathematics::double3x2,
        b: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double3x2_double2x2_75(
        a: crate::Unity::Mathematics::double3x2,
        b: crate::Unity::Mathematics::double2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x2> {
        let __cordl_ret: crate::Unity::Mathematics::double3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double3x2_double2x3_76(
        a: crate::Unity::Mathematics::double3x2,
        b: crate::Unity::Mathematics::double2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x3> {
        let __cordl_ret: crate::Unity::Mathematics::double3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double3x2_double2x4_77(
        a: crate::Unity::Mathematics::double3x2,
        b: crate::Unity::Mathematics::double2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x4> {
        let __cordl_ret: crate::Unity::Mathematics::double3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double3x3_double3_78(
        a: crate::Unity::Mathematics::double3x3,
        b: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double3x3_double3x2_79(
        a: crate::Unity::Mathematics::double3x3,
        b: crate::Unity::Mathematics::double3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x2> {
        let __cordl_ret: crate::Unity::Mathematics::double3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double3x3_double3x3_80(
        a: crate::Unity::Mathematics::double3x3,
        b: crate::Unity::Mathematics::double3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x3> {
        let __cordl_ret: crate::Unity::Mathematics::double3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double3x3_double3x4_81(
        a: crate::Unity::Mathematics::double3x3,
        b: crate::Unity::Mathematics::double3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x4> {
        let __cordl_ret: crate::Unity::Mathematics::double3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double3x4_double4_82(
        a: crate::Unity::Mathematics::double3x4,
        b: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double3x4_double4x2_83(
        a: crate::Unity::Mathematics::double3x4,
        b: crate::Unity::Mathematics::double4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x2> {
        let __cordl_ret: crate::Unity::Mathematics::double3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double3x4_double4x3_84(
        a: crate::Unity::Mathematics::double3x4,
        b: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x3> {
        let __cordl_ret: crate::Unity::Mathematics::double3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double3x4_double4x4_85(
        a: crate::Unity::Mathematics::double3x4,
        b: crate::Unity::Mathematics::double4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x4> {
        let __cordl_ret: crate::Unity::Mathematics::double3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double4_double4_58(
        a: crate::Unity::Mathematics::double4,
        b: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double4_double4x2_59(
        a: crate::Unity::Mathematics::double4,
        b: crate::Unity::Mathematics::double4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double4_double4x3_60(
        a: crate::Unity::Mathematics::double4,
        b: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double4_double4x4_61(
        a: crate::Unity::Mathematics::double4,
        b: crate::Unity::Mathematics::double4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double4x2_double2_86(
        a: crate::Unity::Mathematics::double4x2,
        b: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double4x2_double2x2_87(
        a: crate::Unity::Mathematics::double4x2,
        b: crate::Unity::Mathematics::double2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x2> {
        let __cordl_ret: crate::Unity::Mathematics::double4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double4x2_double2x3_88(
        a: crate::Unity::Mathematics::double4x2,
        b: crate::Unity::Mathematics::double2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double4x2_double2x4_89(
        a: crate::Unity::Mathematics::double4x2,
        b: crate::Unity::Mathematics::double2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x4> {
        let __cordl_ret: crate::Unity::Mathematics::double4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double4x3_double3_90(
        a: crate::Unity::Mathematics::double4x3,
        b: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double4x3_double3x2_91(
        a: crate::Unity::Mathematics::double4x3,
        b: crate::Unity::Mathematics::double3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x2> {
        let __cordl_ret: crate::Unity::Mathematics::double4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double4x3_double3x3_92(
        a: crate::Unity::Mathematics::double4x3,
        b: crate::Unity::Mathematics::double3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double4x3_double3x4_93(
        a: crate::Unity::Mathematics::double4x3,
        b: crate::Unity::Mathematics::double3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x4> {
        let __cordl_ret: crate::Unity::Mathematics::double4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double4x4_double4_94(
        a: crate::Unity::Mathematics::double4x4,
        b: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double4x4_double4x2_95(
        a: crate::Unity::Mathematics::double4x4,
        b: crate::Unity::Mathematics::double4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x2> {
        let __cordl_ret: crate::Unity::Mathematics::double4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double4x4_double4x3_96(
        a: crate::Unity::Mathematics::double4x4,
        b: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_double4x4_double4x4_97(
        a: crate::Unity::Mathematics::double4x4,
        b: crate::Unity::Mathematics::double4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x4> {
        let __cordl_ret: crate::Unity::Mathematics::double4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_f32_f32_0(a: f32, b: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_f64_f64_49(a: f64, b: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float2_float2_1(
        a: crate::Unity::Mathematics::float2,
        b: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float2_float2x2_2(
        a: crate::Unity::Mathematics::float2,
        b: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float2_float2x3_3(
        a: crate::Unity::Mathematics::float2,
        b: crate::Unity::Mathematics::float2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float2_float2x4_4(
        a: crate::Unity::Mathematics::float2,
        b: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float2x2_float2_13(
        a: crate::Unity::Mathematics::float2x2,
        b: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float2x2_float2x2_14(
        a: crate::Unity::Mathematics::float2x2,
        b: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float2x2_float2x3_15(
        a: crate::Unity::Mathematics::float2x2,
        b: crate::Unity::Mathematics::float2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x3> {
        let __cordl_ret: crate::Unity::Mathematics::float2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float2x2_float2x4_16(
        a: crate::Unity::Mathematics::float2x2,
        b: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float2x3_float3_17(
        a: crate::Unity::Mathematics::float2x3,
        b: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float2x3_float3x2_18(
        a: crate::Unity::Mathematics::float2x3,
        b: crate::Unity::Mathematics::float3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float2x3_float3x3_19(
        a: crate::Unity::Mathematics::float2x3,
        b: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x3> {
        let __cordl_ret: crate::Unity::Mathematics::float2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float2x3_float3x4_20(
        a: crate::Unity::Mathematics::float2x3,
        b: crate::Unity::Mathematics::float3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float2x4_float4_21(
        a: crate::Unity::Mathematics::float2x4,
        b: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float2x4_float4x2_22(
        a: crate::Unity::Mathematics::float2x4,
        b: crate::Unity::Mathematics::float4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float2x4_float4x3_23(
        a: crate::Unity::Mathematics::float2x4,
        b: crate::Unity::Mathematics::float4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x3> {
        let __cordl_ret: crate::Unity::Mathematics::float2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float2x4_float4x4_24(
        a: crate::Unity::Mathematics::float2x4,
        b: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float3_float3_5(
        a: crate::Unity::Mathematics::float3,
        b: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float3_float3x2_6(
        a: crate::Unity::Mathematics::float3,
        b: crate::Unity::Mathematics::float3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float3_float3x3_7(
        a: crate::Unity::Mathematics::float3,
        b: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float3_float3x4_8(
        a: crate::Unity::Mathematics::float3,
        b: crate::Unity::Mathematics::float3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float3x2_float2_25(
        a: crate::Unity::Mathematics::float3x2,
        b: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float3x2_float2x2_26(
        a: crate::Unity::Mathematics::float3x2,
        b: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x2> {
        let __cordl_ret: crate::Unity::Mathematics::float3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float3x2_float2x3_27(
        a: crate::Unity::Mathematics::float3x2,
        b: crate::Unity::Mathematics::float2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float3x2_float2x4_28(
        a: crate::Unity::Mathematics::float3x2,
        b: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x4> {
        let __cordl_ret: crate::Unity::Mathematics::float3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float3x3_float3_29(
        a: crate::Unity::Mathematics::float3x3,
        b: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float3x3_float3x2_30(
        a: crate::Unity::Mathematics::float3x3,
        b: crate::Unity::Mathematics::float3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x2> {
        let __cordl_ret: crate::Unity::Mathematics::float3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float3x3_float3x3_31(
        a: crate::Unity::Mathematics::float3x3,
        b: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float3x3_float3x4_32(
        a: crate::Unity::Mathematics::float3x3,
        b: crate::Unity::Mathematics::float3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x4> {
        let __cordl_ret: crate::Unity::Mathematics::float3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float3x4_float4_33(
        a: crate::Unity::Mathematics::float3x4,
        b: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float3x4_float4x2_34(
        a: crate::Unity::Mathematics::float3x4,
        b: crate::Unity::Mathematics::float4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x2> {
        let __cordl_ret: crate::Unity::Mathematics::float3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float3x4_float4x3_35(
        a: crate::Unity::Mathematics::float3x4,
        b: crate::Unity::Mathematics::float4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float3x4_float4x4_36(
        a: crate::Unity::Mathematics::float3x4,
        b: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x4> {
        let __cordl_ret: crate::Unity::Mathematics::float3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float4_float4_9(
        a: crate::Unity::Mathematics::float4,
        b: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float4_float4x2_10(
        a: crate::Unity::Mathematics::float4,
        b: crate::Unity::Mathematics::float4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float4_float4x3_11(
        a: crate::Unity::Mathematics::float4,
        b: crate::Unity::Mathematics::float4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float4_float4x4_12(
        a: crate::Unity::Mathematics::float4,
        b: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float4x2_float2_37(
        a: crate::Unity::Mathematics::float4x2,
        b: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float4x2_float2x2_38(
        a: crate::Unity::Mathematics::float4x2,
        b: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x2> {
        let __cordl_ret: crate::Unity::Mathematics::float4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float4x2_float2x3_39(
        a: crate::Unity::Mathematics::float4x2,
        b: crate::Unity::Mathematics::float2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x3> {
        let __cordl_ret: crate::Unity::Mathematics::float4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float4x2_float2x4_40(
        a: crate::Unity::Mathematics::float4x2,
        b: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float4x3_float3_41(
        a: crate::Unity::Mathematics::float4x3,
        b: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float4x3_float3x2_42(
        a: crate::Unity::Mathematics::float4x3,
        b: crate::Unity::Mathematics::float3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x2> {
        let __cordl_ret: crate::Unity::Mathematics::float4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float4x3_float3x3_43(
        a: crate::Unity::Mathematics::float4x3,
        b: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x3> {
        let __cordl_ret: crate::Unity::Mathematics::float4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float4x3_float3x4_44(
        a: crate::Unity::Mathematics::float4x3,
        b: crate::Unity::Mathematics::float3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float4x4_float4_45(
        a: crate::Unity::Mathematics::float4x4,
        b: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float4x4_float4x2_46(
        a: crate::Unity::Mathematics::float4x4,
        b: crate::Unity::Mathematics::float4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x2> {
        let __cordl_ret: crate::Unity::Mathematics::float4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float4x4_float4x3_47(
        a: crate::Unity::Mathematics::float4x4,
        b: crate::Unity::Mathematics::float4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x3> {
        let __cordl_ret: crate::Unity::Mathematics::float4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_float4x4_float4x4_48(
        a: crate::Unity::Mathematics::float4x4,
        b: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_i32_i32_98(a: i32, b: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int2_int2_99(
        a: crate::Unity::Mathematics::int2,
        b: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int2_int2x2_100(
        a: crate::Unity::Mathematics::int2,
        b: crate::Unity::Mathematics::int2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int2_int2x3_101(
        a: crate::Unity::Mathematics::int2,
        b: crate::Unity::Mathematics::int2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int2_int2x4_102(
        a: crate::Unity::Mathematics::int2,
        b: crate::Unity::Mathematics::int2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int2x2_int2_111(
        a: crate::Unity::Mathematics::int2x2,
        b: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int2x2_int2x2_112(
        a: crate::Unity::Mathematics::int2x2,
        b: crate::Unity::Mathematics::int2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x2> {
        let __cordl_ret: crate::Unity::Mathematics::int2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int2x2_int2x3_113(
        a: crate::Unity::Mathematics::int2x2,
        b: crate::Unity::Mathematics::int2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x3> {
        let __cordl_ret: crate::Unity::Mathematics::int2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int2x2_int2x4_114(
        a: crate::Unity::Mathematics::int2x2,
        b: crate::Unity::Mathematics::int2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x4> {
        let __cordl_ret: crate::Unity::Mathematics::int2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int2x3_int3_115(
        a: crate::Unity::Mathematics::int2x3,
        b: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int2x3_int3x2_116(
        a: crate::Unity::Mathematics::int2x3,
        b: crate::Unity::Mathematics::int3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x2> {
        let __cordl_ret: crate::Unity::Mathematics::int2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int2x3_int3x3_117(
        a: crate::Unity::Mathematics::int2x3,
        b: crate::Unity::Mathematics::int3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x3> {
        let __cordl_ret: crate::Unity::Mathematics::int2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int2x3_int3x4_118(
        a: crate::Unity::Mathematics::int2x3,
        b: crate::Unity::Mathematics::int3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x4> {
        let __cordl_ret: crate::Unity::Mathematics::int2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int2x4_int4_119(
        a: crate::Unity::Mathematics::int2x4,
        b: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int2x4_int4x2_120(
        a: crate::Unity::Mathematics::int2x4,
        b: crate::Unity::Mathematics::int4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x2> {
        let __cordl_ret: crate::Unity::Mathematics::int2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int2x4_int4x3_121(
        a: crate::Unity::Mathematics::int2x4,
        b: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x3> {
        let __cordl_ret: crate::Unity::Mathematics::int2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int2x4_int4x4_122(
        a: crate::Unity::Mathematics::int2x4,
        b: crate::Unity::Mathematics::int4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x4> {
        let __cordl_ret: crate::Unity::Mathematics::int2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int3_int3_103(
        a: crate::Unity::Mathematics::int3,
        b: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int3_int3x2_104(
        a: crate::Unity::Mathematics::int3,
        b: crate::Unity::Mathematics::int3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int3_int3x3_105(
        a: crate::Unity::Mathematics::int3,
        b: crate::Unity::Mathematics::int3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int3_int3x4_106(
        a: crate::Unity::Mathematics::int3,
        b: crate::Unity::Mathematics::int3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int3x2_int2_123(
        a: crate::Unity::Mathematics::int3x2,
        b: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int3x2_int2x2_124(
        a: crate::Unity::Mathematics::int3x2,
        b: crate::Unity::Mathematics::int2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x2> {
        let __cordl_ret: crate::Unity::Mathematics::int3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int3x2_int2x3_125(
        a: crate::Unity::Mathematics::int3x2,
        b: crate::Unity::Mathematics::int2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x3> {
        let __cordl_ret: crate::Unity::Mathematics::int3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int3x2_int2x4_126(
        a: crate::Unity::Mathematics::int3x2,
        b: crate::Unity::Mathematics::int2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x4> {
        let __cordl_ret: crate::Unity::Mathematics::int3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int3x3_int3_127(
        a: crate::Unity::Mathematics::int3x3,
        b: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int3x3_int3x2_128(
        a: crate::Unity::Mathematics::int3x3,
        b: crate::Unity::Mathematics::int3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x2> {
        let __cordl_ret: crate::Unity::Mathematics::int3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int3x3_int3x3_129(
        a: crate::Unity::Mathematics::int3x3,
        b: crate::Unity::Mathematics::int3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x3> {
        let __cordl_ret: crate::Unity::Mathematics::int3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int3x3_int3x4_130(
        a: crate::Unity::Mathematics::int3x3,
        b: crate::Unity::Mathematics::int3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x4> {
        let __cordl_ret: crate::Unity::Mathematics::int3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int3x4_int4_131(
        a: crate::Unity::Mathematics::int3x4,
        b: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int3x4_int4x2_132(
        a: crate::Unity::Mathematics::int3x4,
        b: crate::Unity::Mathematics::int4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x2> {
        let __cordl_ret: crate::Unity::Mathematics::int3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int3x4_int4x3_133(
        a: crate::Unity::Mathematics::int3x4,
        b: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x3> {
        let __cordl_ret: crate::Unity::Mathematics::int3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int3x4_int4x4_134(
        a: crate::Unity::Mathematics::int3x4,
        b: crate::Unity::Mathematics::int4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x4> {
        let __cordl_ret: crate::Unity::Mathematics::int3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int4_int4_107(
        a: crate::Unity::Mathematics::int4,
        b: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int4_int4x2_108(
        a: crate::Unity::Mathematics::int4,
        b: crate::Unity::Mathematics::int4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int4_int4x3_109(
        a: crate::Unity::Mathematics::int4,
        b: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int4_int4x4_110(
        a: crate::Unity::Mathematics::int4,
        b: crate::Unity::Mathematics::int4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int4x2_int2_135(
        a: crate::Unity::Mathematics::int4x2,
        b: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int4x2_int2x2_136(
        a: crate::Unity::Mathematics::int4x2,
        b: crate::Unity::Mathematics::int2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x2> {
        let __cordl_ret: crate::Unity::Mathematics::int4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int4x2_int2x3_137(
        a: crate::Unity::Mathematics::int4x2,
        b: crate::Unity::Mathematics::int2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int4x2_int2x4_138(
        a: crate::Unity::Mathematics::int4x2,
        b: crate::Unity::Mathematics::int2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x4> {
        let __cordl_ret: crate::Unity::Mathematics::int4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int4x3_int3_139(
        a: crate::Unity::Mathematics::int4x3,
        b: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int4x3_int3x2_140(
        a: crate::Unity::Mathematics::int4x3,
        b: crate::Unity::Mathematics::int3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x2> {
        let __cordl_ret: crate::Unity::Mathematics::int4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int4x3_int3x3_141(
        a: crate::Unity::Mathematics::int4x3,
        b: crate::Unity::Mathematics::int3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int4x3_int3x4_142(
        a: crate::Unity::Mathematics::int4x3,
        b: crate::Unity::Mathematics::int3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x4> {
        let __cordl_ret: crate::Unity::Mathematics::int4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int4x4_int4_143(
        a: crate::Unity::Mathematics::int4x4,
        b: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int4x4_int4x2_144(
        a: crate::Unity::Mathematics::int4x4,
        b: crate::Unity::Mathematics::int4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x2> {
        let __cordl_ret: crate::Unity::Mathematics::int4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int4x4_int4x3_145(
        a: crate::Unity::Mathematics::int4x4,
        b: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_int4x4_int4x4_146(
        a: crate::Unity::Mathematics::int4x4,
        b: crate::Unity::Mathematics::int4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x4> {
        let __cordl_ret: crate::Unity::Mathematics::int4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_quaternion_float3_197(
        q: crate::Unity::Mathematics::quaternion,
        v: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (q, v))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_quaternion_quaternion196(
        a: crate::Unity::Mathematics::quaternion,
        b: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_u32_u32_147(a: u32, b: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint2_uint2_148(
        a: crate::Unity::Mathematics::uint2,
        b: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint2_uint2x2_149(
        a: crate::Unity::Mathematics::uint2,
        b: crate::Unity::Mathematics::uint2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint2_uint2x3_150(
        a: crate::Unity::Mathematics::uint2,
        b: crate::Unity::Mathematics::uint2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint2_uint2x4_151(
        a: crate::Unity::Mathematics::uint2,
        b: crate::Unity::Mathematics::uint2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint2x2_uint2_160(
        a: crate::Unity::Mathematics::uint2x2,
        b: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint2x2_uint2x2_161(
        a: crate::Unity::Mathematics::uint2x2,
        b: crate::Unity::Mathematics::uint2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint2x2_uint2x3_162(
        a: crate::Unity::Mathematics::uint2x2,
        b: crate::Unity::Mathematics::uint2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint2x2_uint2x4_163(
        a: crate::Unity::Mathematics::uint2x2,
        b: crate::Unity::Mathematics::uint2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint2x3_uint3_164(
        a: crate::Unity::Mathematics::uint2x3,
        b: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint2x3_uint3x2_165(
        a: crate::Unity::Mathematics::uint2x3,
        b: crate::Unity::Mathematics::uint3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint2x3_uint3x3_166(
        a: crate::Unity::Mathematics::uint2x3,
        b: crate::Unity::Mathematics::uint3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint2x3_uint3x4_167(
        a: crate::Unity::Mathematics::uint2x3,
        b: crate::Unity::Mathematics::uint3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint2x4_uint4_168(
        a: crate::Unity::Mathematics::uint2x4,
        b: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint2x4_uint4x2_169(
        a: crate::Unity::Mathematics::uint2x4,
        b: crate::Unity::Mathematics::uint4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint2x4_uint4x3_170(
        a: crate::Unity::Mathematics::uint2x4,
        b: crate::Unity::Mathematics::uint4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint2x4_uint4x4_171(
        a: crate::Unity::Mathematics::uint2x4,
        b: crate::Unity::Mathematics::uint4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint3_uint3_152(
        a: crate::Unity::Mathematics::uint3,
        b: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint3_uint3x2_153(
        a: crate::Unity::Mathematics::uint3,
        b: crate::Unity::Mathematics::uint3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint3_uint3x3_154(
        a: crate::Unity::Mathematics::uint3,
        b: crate::Unity::Mathematics::uint3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint3_uint3x4_155(
        a: crate::Unity::Mathematics::uint3,
        b: crate::Unity::Mathematics::uint3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint3x2_uint2_172(
        a: crate::Unity::Mathematics::uint3x2,
        b: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint3x2_uint2x2_173(
        a: crate::Unity::Mathematics::uint3x2,
        b: crate::Unity::Mathematics::uint2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint3x2_uint2x3_174(
        a: crate::Unity::Mathematics::uint3x2,
        b: crate::Unity::Mathematics::uint2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint3x2_uint2x4_175(
        a: crate::Unity::Mathematics::uint3x2,
        b: crate::Unity::Mathematics::uint2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint3x3_uint3_176(
        a: crate::Unity::Mathematics::uint3x3,
        b: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint3x3_uint3x2_177(
        a: crate::Unity::Mathematics::uint3x3,
        b: crate::Unity::Mathematics::uint3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint3x3_uint3x3_178(
        a: crate::Unity::Mathematics::uint3x3,
        b: crate::Unity::Mathematics::uint3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint3x3_uint3x4_179(
        a: crate::Unity::Mathematics::uint3x3,
        b: crate::Unity::Mathematics::uint3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint3x4_uint4_180(
        a: crate::Unity::Mathematics::uint3x4,
        b: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint3x4_uint4x2_181(
        a: crate::Unity::Mathematics::uint3x4,
        b: crate::Unity::Mathematics::uint4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint3x4_uint4x3_182(
        a: crate::Unity::Mathematics::uint3x4,
        b: crate::Unity::Mathematics::uint4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint3x4_uint4x4_183(
        a: crate::Unity::Mathematics::uint3x4,
        b: crate::Unity::Mathematics::uint4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint4_uint4_156(
        a: crate::Unity::Mathematics::uint4,
        b: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint4_uint4x2_157(
        a: crate::Unity::Mathematics::uint4,
        b: crate::Unity::Mathematics::uint4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint4_uint4x3_158(
        a: crate::Unity::Mathematics::uint4,
        b: crate::Unity::Mathematics::uint4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint4_uint4x4_159(
        a: crate::Unity::Mathematics::uint4,
        b: crate::Unity::Mathematics::uint4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint4x2_uint2_184(
        a: crate::Unity::Mathematics::uint4x2,
        b: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint4x2_uint2x2_185(
        a: crate::Unity::Mathematics::uint4x2,
        b: crate::Unity::Mathematics::uint2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint4x2_uint2x3_186(
        a: crate::Unity::Mathematics::uint4x2,
        b: crate::Unity::Mathematics::uint2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint4x2_uint2x4_187(
        a: crate::Unity::Mathematics::uint4x2,
        b: crate::Unity::Mathematics::uint2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint4x3_uint3_188(
        a: crate::Unity::Mathematics::uint4x3,
        b: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint4x3_uint3x2_189(
        a: crate::Unity::Mathematics::uint4x3,
        b: crate::Unity::Mathematics::uint3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint4x3_uint3x3_190(
        a: crate::Unity::Mathematics::uint4x3,
        b: crate::Unity::Mathematics::uint3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint4x3_uint3x4_191(
        a: crate::Unity::Mathematics::uint4x3,
        b: crate::Unity::Mathematics::uint3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint4x4_uint4_192(
        a: crate::Unity::Mathematics::uint4x4,
        b: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint4x4_uint4x2_193(
        a: crate::Unity::Mathematics::uint4x4,
        b: crate::Unity::Mathematics::uint4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint4x4_uint4x3_194(
        a: crate::Unity::Mathematics::uint4x4,
        b: crate::Unity::Mathematics::uint4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn mul_uint4x4_uint4x4_195(
        a: crate::Unity::Mathematics::uint4x4,
        b: crate::Unity::Mathematics::uint4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mul", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn nlerp(
        q1: crate::Unity::Mathematics::quaternion,
        q2: crate::Unity::Mathematics::quaternion,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("nlerp", (q1, q2, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn normalize_double2_3(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("normalize", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn normalize_double3_4(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("normalize", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn normalize_double4_5(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("normalize", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn normalize_float2_0(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("normalize", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn normalize_float3_1(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("normalize", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn normalize_float4_2(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("normalize", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn normalize_quaternion6(
        q: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("normalize", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn normalizesafe_double2_double2_3(
        x: crate::Unity::Mathematics::double2,
        defaultvalue: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("normalizesafe", (x, defaultvalue))?;
        Ok(__cordl_ret.into())
    }
    pub fn normalizesafe_double3_double3_4(
        x: crate::Unity::Mathematics::double3,
        defaultvalue: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("normalizesafe", (x, defaultvalue))?;
        Ok(__cordl_ret.into())
    }
    pub fn normalizesafe_double4_double4_5(
        x: crate::Unity::Mathematics::double4,
        defaultvalue: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("normalizesafe", (x, defaultvalue))?;
        Ok(__cordl_ret.into())
    }
    pub fn normalizesafe_float2_float2_0(
        x: crate::Unity::Mathematics::float2,
        defaultvalue: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("normalizesafe", (x, defaultvalue))?;
        Ok(__cordl_ret.into())
    }
    pub fn normalizesafe_float3_float3_1(
        x: crate::Unity::Mathematics::float3,
        defaultvalue: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("normalizesafe", (x, defaultvalue))?;
        Ok(__cordl_ret.into())
    }
    pub fn normalizesafe_float4_float4_2(
        x: crate::Unity::Mathematics::float4,
        defaultvalue: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("normalizesafe", (x, defaultvalue))?;
        Ok(__cordl_ret.into())
    }
    pub fn normalizesafe_quaternion6(
        q: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("normalizesafe", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn normalizesafe_quaternion_quaternion7(
        q: crate::Unity::Mathematics::quaternion,
        defaultvalue: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("normalizesafe", (q, defaultvalue))?;
        Ok(__cordl_ret.into())
    }
    pub fn orthonormalize(
        i: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("orthonormalize", (i))?;
        Ok(__cordl_ret.into())
    }
    pub fn pow_double2_double2_5(
        x: crate::Unity::Mathematics::double2,
        y: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("pow", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn pow_double3_double3_6(
        x: crate::Unity::Mathematics::double3,
        y: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("pow", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn pow_double4_double4_7(
        x: crate::Unity::Mathematics::double4,
        y: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("pow", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn pow_f32_f32_0(x: f32, y: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("pow", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn pow_f64_f64_4(x: f64, y: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("pow", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn pow_float2_float2_1(
        x: crate::Unity::Mathematics::float2,
        y: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("pow", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn pow_float3_float3_2(
        x: crate::Unity::Mathematics::float3,
        y: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("pow", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn pow_float4_float4_3(
        x: crate::Unity::Mathematics::float4,
        y: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("pow", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn project_double2_double2_3(
        a: crate::Unity::Mathematics::double2,
        b: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("project", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn project_double3_double3_4(
        a: crate::Unity::Mathematics::double3,
        b: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("project", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn project_double4_double4_5(
        a: crate::Unity::Mathematics::double4,
        b: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("project", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn project_float2_float2_0(
        a: crate::Unity::Mathematics::float2,
        b: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("project", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn project_float3_float3_1(
        a: crate::Unity::Mathematics::float3,
        b: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("project", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn project_float4_float4_2(
        a: crate::Unity::Mathematics::float4,
        b: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("project", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn projectsafe_double2_double2_double2_3(
        a: crate::Unity::Mathematics::double2,
        b: crate::Unity::Mathematics::double2,
        defaultValue: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("projectsafe", (a, b, defaultValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn projectsafe_double3_double3_double3_4(
        a: crate::Unity::Mathematics::double3,
        b: crate::Unity::Mathematics::double3,
        defaultValue: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("projectsafe", (a, b, defaultValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn projectsafe_double4_double4_double4_5(
        a: crate::Unity::Mathematics::double4,
        b: crate::Unity::Mathematics::double4,
        defaultValue: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("projectsafe", (a, b, defaultValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn projectsafe_float2_float2_float2_0(
        a: crate::Unity::Mathematics::float2,
        b: crate::Unity::Mathematics::float2,
        defaultValue: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("projectsafe", (a, b, defaultValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn projectsafe_float3_float3_float3_1(
        a: crate::Unity::Mathematics::float3,
        b: crate::Unity::Mathematics::float3,
        defaultValue: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("projectsafe", (a, b, defaultValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn projectsafe_float4_float4_float4_2(
        a: crate::Unity::Mathematics::float4,
        b: crate::Unity::Mathematics::float4,
        defaultValue: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("projectsafe", (a, b, defaultValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn quaternion_f32_f32_f32_f32_0(
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("quaternion", (x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn quaternion_float3x3_2(
        m: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("quaternion", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn quaternion_float4_1(
        value: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("quaternion", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn quaternion_float4x4_3(
        m: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("quaternion", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn radians_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("radians", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn radians_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("radians", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn radians_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("radians", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn radians_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("radians", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn radians_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("radians", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn radians_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("radians", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn radians_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("radians", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn radians_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("radians", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn rcp_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rcp", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn rcp_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rcp", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn rcp_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rcp", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn rcp_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rcp", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn rcp_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rcp", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn rcp_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rcp", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn rcp_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rcp", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn rcp_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rcp", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn reflect_double2_double2_3(
        i: crate::Unity::Mathematics::double2,
        n: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("reflect", (i, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn reflect_double3_double3_4(
        i: crate::Unity::Mathematics::double3,
        n: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("reflect", (i, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn reflect_double4_double4_5(
        i: crate::Unity::Mathematics::double4,
        n: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("reflect", (i, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn reflect_float2_float2_0(
        i: crate::Unity::Mathematics::float2,
        n: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("reflect", (i, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn reflect_float3_float3_1(
        i: crate::Unity::Mathematics::float3,
        n: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("reflect", (i, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn reflect_float4_float4_2(
        i: crate::Unity::Mathematics::float4,
        n: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("reflect", (i, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn refract_double2_double2_f64_3(
        i: crate::Unity::Mathematics::double2,
        n: crate::Unity::Mathematics::double2,
        eta: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("refract", (i, n, eta))?;
        Ok(__cordl_ret.into())
    }
    pub fn refract_double3_double3_f64_4(
        i: crate::Unity::Mathematics::double3,
        n: crate::Unity::Mathematics::double3,
        eta: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("refract", (i, n, eta))?;
        Ok(__cordl_ret.into())
    }
    pub fn refract_double4_double4_f64_5(
        i: crate::Unity::Mathematics::double4,
        n: crate::Unity::Mathematics::double4,
        eta: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("refract", (i, n, eta))?;
        Ok(__cordl_ret.into())
    }
    pub fn refract_float2_float2_f32_0(
        i: crate::Unity::Mathematics::float2,
        n: crate::Unity::Mathematics::float2,
        eta: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("refract", (i, n, eta))?;
        Ok(__cordl_ret.into())
    }
    pub fn refract_float3_float3_f32_1(
        i: crate::Unity::Mathematics::float3,
        n: crate::Unity::Mathematics::float3,
        eta: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("refract", (i, n, eta))?;
        Ok(__cordl_ret.into())
    }
    pub fn refract_float4_float4_f32_2(
        i: crate::Unity::Mathematics::float4,
        n: crate::Unity::Mathematics::float4,
        eta: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("refract", (i, n, eta))?;
        Ok(__cordl_ret.into())
    }
    pub fn remap_double2_double2_double2_double2_double2_5(
        a: crate::Unity::Mathematics::double2,
        b: crate::Unity::Mathematics::double2,
        c: crate::Unity::Mathematics::double2,
        d: crate::Unity::Mathematics::double2,
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remap", (a, b, c, d, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn remap_double3_double3_double3_double3_double3_6(
        a: crate::Unity::Mathematics::double3,
        b: crate::Unity::Mathematics::double3,
        c: crate::Unity::Mathematics::double3,
        d: crate::Unity::Mathematics::double3,
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remap", (a, b, c, d, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn remap_double4_double4_double4_double4_double4_7(
        a: crate::Unity::Mathematics::double4,
        b: crate::Unity::Mathematics::double4,
        c: crate::Unity::Mathematics::double4,
        d: crate::Unity::Mathematics::double4,
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remap", (a, b, c, d, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn remap_f32_f32_f32_f32_f32_0(
        a: f32,
        b: f32,
        c: f32,
        d: f32,
        x: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remap", (a, b, c, d, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn remap_f64_f64_f64_f64_f64_4(
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        x: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remap", (a, b, c, d, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn remap_float2_float2_float2_float2_float2_1(
        a: crate::Unity::Mathematics::float2,
        b: crate::Unity::Mathematics::float2,
        c: crate::Unity::Mathematics::float2,
        d: crate::Unity::Mathematics::float2,
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remap", (a, b, c, d, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn remap_float3_float3_float3_float3_float3_2(
        a: crate::Unity::Mathematics::float3,
        b: crate::Unity::Mathematics::float3,
        c: crate::Unity::Mathematics::float3,
        d: crate::Unity::Mathematics::float3,
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remap", (a, b, c, d, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn remap_float4_float4_float4_float4_float4_3(
        a: crate::Unity::Mathematics::float4,
        b: crate::Unity::Mathematics::float4,
        c: crate::Unity::Mathematics::float4,
        d: crate::Unity::Mathematics::float4,
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("remap", (a, b, c, d, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn reversebits_i32_0(x: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("reversebits", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn reversebits_i64_8(x: i64) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("reversebits", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn reversebits_int2_1(
        x: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("reversebits", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn reversebits_int3_2(
        x: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("reversebits", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn reversebits_int4_3(
        x: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("reversebits", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn reversebits_u32_4(x: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("reversebits", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn reversebits_u64_9(x: u64) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("reversebits", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn reversebits_uint2_5(
        x: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("reversebits", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn reversebits_uint3_6(
        x: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("reversebits", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn reversebits_uint4_7(
        x: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("reversebits", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn right() -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("right", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn rol_i32_0(x: i32, n: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rol", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn rol_i64_8(x: i64, n: i32) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rol", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn rol_int2_1(
        x: crate::Unity::Mathematics::int2,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rol", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn rol_int3_2(
        x: crate::Unity::Mathematics::int3,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rol", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn rol_int4_3(
        x: crate::Unity::Mathematics::int4,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rol", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn rol_u32_4(x: u32, n: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rol", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn rol_u64_9(x: u64, n: i32) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rol", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn rol_uint2_5(
        x: crate::Unity::Mathematics::uint2,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rol", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn rol_uint3_6(
        x: crate::Unity::Mathematics::uint3,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rol", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn rol_uint4_7(
        x: crate::Unity::Mathematics::uint4,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rol", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn ror_i32_0(x: i32, n: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ror", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn ror_i64_8(x: i64, n: i32) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ror", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn ror_int2_1(
        x: crate::Unity::Mathematics::int2,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ror", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn ror_int3_2(
        x: crate::Unity::Mathematics::int3,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ror", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn ror_int4_3(
        x: crate::Unity::Mathematics::int4,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ror", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn ror_u32_4(x: u32, n: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ror", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn ror_u64_9(x: u64, n: i32) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ror", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn ror_uint2_5(
        x: crate::Unity::Mathematics::uint2,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ror", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn ror_uint3_6(
        x: crate::Unity::Mathematics::uint3,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ror", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn ror_uint4_7(
        x: crate::Unity::Mathematics::uint4,
        n: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ror", (x, n))?;
        Ok(__cordl_ret.into())
    }
    pub fn rotate_RigidTransform_float3_3(
        a: crate::Unity::Mathematics::RigidTransform,
        dir: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rotate", (a, dir))?;
        Ok(__cordl_ret.into())
    }
    pub fn rotate_double4x4_double3_0(
        a: crate::Unity::Mathematics::double4x4,
        b: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rotate", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn rotate_float4x4_float3_1(
        a: crate::Unity::Mathematics::float4x4,
        b: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rotate", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn rotate_quaternion_float3_2(
        q: crate::Unity::Mathematics::quaternion,
        v: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rotate", (q, v))?;
        Ok(__cordl_ret.into())
    }
    pub fn round_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("round", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn round_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("round", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn round_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("round", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn round_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("round", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn round_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("round", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn round_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("round", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn round_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("round", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn round_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("round", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn rsqrt_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rsqrt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn rsqrt_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rsqrt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn rsqrt_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rsqrt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn rsqrt_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rsqrt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn rsqrt_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rsqrt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn rsqrt_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rsqrt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn rsqrt_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rsqrt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn rsqrt_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rsqrt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn saturate_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("saturate", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn saturate_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("saturate", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn saturate_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("saturate", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn saturate_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("saturate", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn saturate_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("saturate", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn saturate_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("saturate", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn saturate_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("saturate", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn saturate_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("saturate", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_double2_double2__cordl_bool24(
        a: crate::Unity::Mathematics::double2,
        b: crate::Unity::Mathematics::double2,
        c: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_double2_double2_bool2_27(
        a: crate::Unity::Mathematics::double2,
        b: crate::Unity::Mathematics::double2,
        c: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_double3_double3__cordl_bool25(
        a: crate::Unity::Mathematics::double3,
        b: crate::Unity::Mathematics::double3,
        c: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_double3_double3_bool3_28(
        a: crate::Unity::Mathematics::double3,
        b: crate::Unity::Mathematics::double3,
        c: crate::Unity::Mathematics::bool3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_double4_double4__cordl_bool26(
        a: crate::Unity::Mathematics::double4,
        b: crate::Unity::Mathematics::double4,
        c: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_double4_double4_bool4_29(
        a: crate::Unity::Mathematics::double4,
        b: crate::Unity::Mathematics::double4,
        c: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_f32_f32__cordl_bool16(
        a: f32,
        b: f32,
        c: bool,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_f64_f64__cordl_bool23(
        a: f64,
        b: f64,
        c: bool,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_float2_float2__cordl_bool17(
        a: crate::Unity::Mathematics::float2,
        b: crate::Unity::Mathematics::float2,
        c: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_float2_float2_bool2_20(
        a: crate::Unity::Mathematics::float2,
        b: crate::Unity::Mathematics::float2,
        c: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_float3_float3__cordl_bool18(
        a: crate::Unity::Mathematics::float3,
        b: crate::Unity::Mathematics::float3,
        c: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_float3_float3_bool3_21(
        a: crate::Unity::Mathematics::float3,
        b: crate::Unity::Mathematics::float3,
        c: crate::Unity::Mathematics::bool3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_float4_float4__cordl_bool19(
        a: crate::Unity::Mathematics::float4,
        b: crate::Unity::Mathematics::float4,
        c: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_float4_float4_bool4_22(
        a: crate::Unity::Mathematics::float4,
        b: crate::Unity::Mathematics::float4,
        c: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_i32_i32__cordl_bool0(
        a: i32,
        b: i32,
        c: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_i64_i64__cordl_bool14(
        a: i64,
        b: i64,
        c: bool,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_int2_int2__cordl_bool1(
        a: crate::Unity::Mathematics::int2,
        b: crate::Unity::Mathematics::int2,
        c: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_int2_int2_bool2_4(
        a: crate::Unity::Mathematics::int2,
        b: crate::Unity::Mathematics::int2,
        c: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_int3_int3__cordl_bool2(
        a: crate::Unity::Mathematics::int3,
        b: crate::Unity::Mathematics::int3,
        c: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_int3_int3_bool3_5(
        a: crate::Unity::Mathematics::int3,
        b: crate::Unity::Mathematics::int3,
        c: crate::Unity::Mathematics::bool3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_int4_int4__cordl_bool3(
        a: crate::Unity::Mathematics::int4,
        b: crate::Unity::Mathematics::int4,
        c: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_int4_int4_bool4_6(
        a: crate::Unity::Mathematics::int4,
        b: crate::Unity::Mathematics::int4,
        c: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_shuffle_component_bool2_bool2_0(
        a: crate::Unity::Mathematics::bool2,
        b: crate::Unity::Mathematics::bool2,
        component: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select_shuffle_component", (a, b, component))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_shuffle_component_bool3_bool3_1(
        a: crate::Unity::Mathematics::bool3,
        b: crate::Unity::Mathematics::bool3,
        component: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select_shuffle_component", (a, b, component))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_shuffle_component_bool4_bool4_2(
        a: crate::Unity::Mathematics::bool4,
        b: crate::Unity::Mathematics::bool4,
        component: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select_shuffle_component", (a, b, component))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_shuffle_component_double2_double2_3(
        a: crate::Unity::Mathematics::double2,
        b: crate::Unity::Mathematics::double2,
        component: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select_shuffle_component", (a, b, component))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_shuffle_component_double3_double3_4(
        a: crate::Unity::Mathematics::double3,
        b: crate::Unity::Mathematics::double3,
        component: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select_shuffle_component", (a, b, component))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_shuffle_component_double4_double4_5(
        a: crate::Unity::Mathematics::double4,
        b: crate::Unity::Mathematics::double4,
        component: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select_shuffle_component", (a, b, component))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_shuffle_component_float2_float2_6(
        a: crate::Unity::Mathematics::float2,
        b: crate::Unity::Mathematics::float2,
        component: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select_shuffle_component", (a, b, component))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_shuffle_component_float3_float3_7(
        a: crate::Unity::Mathematics::float3,
        b: crate::Unity::Mathematics::float3,
        component: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select_shuffle_component", (a, b, component))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_shuffle_component_float4_float4_8(
        a: crate::Unity::Mathematics::float4,
        b: crate::Unity::Mathematics::float4,
        component: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select_shuffle_component", (a, b, component))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_shuffle_component_int2_int2_9(
        a: crate::Unity::Mathematics::int2,
        b: crate::Unity::Mathematics::int2,
        component: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select_shuffle_component", (a, b, component))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_shuffle_component_int3_int3_10(
        a: crate::Unity::Mathematics::int3,
        b: crate::Unity::Mathematics::int3,
        component: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select_shuffle_component", (a, b, component))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_shuffle_component_int4_int4_11(
        a: crate::Unity::Mathematics::int4,
        b: crate::Unity::Mathematics::int4,
        component: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select_shuffle_component", (a, b, component))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_shuffle_component_uint2_uint2_12(
        a: crate::Unity::Mathematics::uint2,
        b: crate::Unity::Mathematics::uint2,
        component: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select_shuffle_component", (a, b, component))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_shuffle_component_uint3_uint3_13(
        a: crate::Unity::Mathematics::uint3,
        b: crate::Unity::Mathematics::uint3,
        component: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select_shuffle_component", (a, b, component))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_shuffle_component_uint4_uint4_14(
        a: crate::Unity::Mathematics::uint4,
        b: crate::Unity::Mathematics::uint4,
        component: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select_shuffle_component", (a, b, component))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_u32_u32__cordl_bool7(
        a: u32,
        b: u32,
        c: bool,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_u64_u64__cordl_bool15(
        a: u64,
        b: u64,
        c: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_uint2_uint2__cordl_bool8(
        a: crate::Unity::Mathematics::uint2,
        b: crate::Unity::Mathematics::uint2,
        c: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_uint2_uint2_bool2_11(
        a: crate::Unity::Mathematics::uint2,
        b: crate::Unity::Mathematics::uint2,
        c: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_uint3_uint3__cordl_bool9(
        a: crate::Unity::Mathematics::uint3,
        b: crate::Unity::Mathematics::uint3,
        c: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_uint3_uint3_bool3_12(
        a: crate::Unity::Mathematics::uint3,
        b: crate::Unity::Mathematics::uint3,
        c: crate::Unity::Mathematics::bool3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_uint4_uint4__cordl_bool10(
        a: crate::Unity::Mathematics::uint4,
        b: crate::Unity::Mathematics::uint4,
        c: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn select_uint4_uint4_bool4_13(
        a: crate::Unity::Mathematics::uint4,
        b: crate::Unity::Mathematics::uint4,
        c: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("select", (a, b, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_bool2_bool2_0(
        left: crate::Unity::Mathematics::bool2,
        right: crate::Unity::Mathematics::bool2,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_bool2_bool2_math_ShuffleComponent1(
        left: crate::Unity::Mathematics::bool2,
        right: crate::Unity::Mathematics::bool2,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_bool2_bool2_math_ShuffleComponent_math_ShuffleComponent2(
        left: crate::Unity::Mathematics::bool2,
        right: crate::Unity::Mathematics::bool2,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_bool2_bool2_math_ShuffleComponent_math_ShuffleComponent_math_ShuffleComponent3(
        left: crate::Unity::Mathematics::bool2,
        right: crate::Unity::Mathematics::bool2,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
        w: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_bool3_bool3_4(
        left: crate::Unity::Mathematics::bool3,
        right: crate::Unity::Mathematics::bool3,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_bool3_bool3_math_ShuffleComponent5(
        left: crate::Unity::Mathematics::bool3,
        right: crate::Unity::Mathematics::bool3,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_bool3_bool3_math_ShuffleComponent_math_ShuffleComponent6(
        left: crate::Unity::Mathematics::bool3,
        right: crate::Unity::Mathematics::bool3,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_bool3_bool3_math_ShuffleComponent_math_ShuffleComponent_math_ShuffleComponent7(
        left: crate::Unity::Mathematics::bool3,
        right: crate::Unity::Mathematics::bool3,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
        w: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_bool4_bool4_8(
        left: crate::Unity::Mathematics::bool4,
        right: crate::Unity::Mathematics::bool4,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_bool4_bool4_math_ShuffleComponent9(
        left: crate::Unity::Mathematics::bool4,
        right: crate::Unity::Mathematics::bool4,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_bool4_bool4_math_ShuffleComponent_math_ShuffleComponent10(
        left: crate::Unity::Mathematics::bool4,
        right: crate::Unity::Mathematics::bool4,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_bool4_bool4_math_ShuffleComponent_math_ShuffleComponent_math_ShuffleComponent11(
        left: crate::Unity::Mathematics::bool4,
        right: crate::Unity::Mathematics::bool4,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
        w: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_double2_double2_12(
        left: crate::Unity::Mathematics::double2,
        right: crate::Unity::Mathematics::double2,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_double2_double2_math_ShuffleComponent13(
        left: crate::Unity::Mathematics::double2,
        right: crate::Unity::Mathematics::double2,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_double2_double2_math_ShuffleComponent_math_ShuffleComponent14(
        left: crate::Unity::Mathematics::double2,
        right: crate::Unity::Mathematics::double2,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_double2_double2_math_ShuffleComponent_math_ShuffleComponent_math_ShuffleComponent15(
        left: crate::Unity::Mathematics::double2,
        right: crate::Unity::Mathematics::double2,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
        w: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_double3_double3_16(
        left: crate::Unity::Mathematics::double3,
        right: crate::Unity::Mathematics::double3,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_double3_double3_math_ShuffleComponent17(
        left: crate::Unity::Mathematics::double3,
        right: crate::Unity::Mathematics::double3,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_double3_double3_math_ShuffleComponent_math_ShuffleComponent18(
        left: crate::Unity::Mathematics::double3,
        right: crate::Unity::Mathematics::double3,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_double3_double3_math_ShuffleComponent_math_ShuffleComponent_math_ShuffleComponent19(
        left: crate::Unity::Mathematics::double3,
        right: crate::Unity::Mathematics::double3,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
        w: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_double4_double4_20(
        left: crate::Unity::Mathematics::double4,
        right: crate::Unity::Mathematics::double4,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_double4_double4_math_ShuffleComponent21(
        left: crate::Unity::Mathematics::double4,
        right: crate::Unity::Mathematics::double4,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_double4_double4_math_ShuffleComponent_math_ShuffleComponent22(
        left: crate::Unity::Mathematics::double4,
        right: crate::Unity::Mathematics::double4,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_double4_double4_math_ShuffleComponent_math_ShuffleComponent_math_ShuffleComponent23(
        left: crate::Unity::Mathematics::double4,
        right: crate::Unity::Mathematics::double4,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
        w: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_float2_float2_24(
        left: crate::Unity::Mathematics::float2,
        right: crate::Unity::Mathematics::float2,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_float2_float2_math_ShuffleComponent25(
        left: crate::Unity::Mathematics::float2,
        right: crate::Unity::Mathematics::float2,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_float2_float2_math_ShuffleComponent_math_ShuffleComponent26(
        left: crate::Unity::Mathematics::float2,
        right: crate::Unity::Mathematics::float2,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_float2_float2_math_ShuffleComponent_math_ShuffleComponent_math_ShuffleComponent27(
        left: crate::Unity::Mathematics::float2,
        right: crate::Unity::Mathematics::float2,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
        w: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_float3_float3_28(
        left: crate::Unity::Mathematics::float3,
        right: crate::Unity::Mathematics::float3,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_float3_float3_math_ShuffleComponent29(
        left: crate::Unity::Mathematics::float3,
        right: crate::Unity::Mathematics::float3,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_float3_float3_math_ShuffleComponent_math_ShuffleComponent30(
        left: crate::Unity::Mathematics::float3,
        right: crate::Unity::Mathematics::float3,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_float3_float3_math_ShuffleComponent_math_ShuffleComponent_math_ShuffleComponent31(
        left: crate::Unity::Mathematics::float3,
        right: crate::Unity::Mathematics::float3,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
        w: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_float4_float4_32(
        left: crate::Unity::Mathematics::float4,
        right: crate::Unity::Mathematics::float4,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_float4_float4_math_ShuffleComponent33(
        left: crate::Unity::Mathematics::float4,
        right: crate::Unity::Mathematics::float4,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_float4_float4_math_ShuffleComponent_math_ShuffleComponent34(
        left: crate::Unity::Mathematics::float4,
        right: crate::Unity::Mathematics::float4,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_float4_float4_math_ShuffleComponent_math_ShuffleComponent_math_ShuffleComponent35(
        left: crate::Unity::Mathematics::float4,
        right: crate::Unity::Mathematics::float4,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
        w: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_int2_int2_36(
        left: crate::Unity::Mathematics::int2,
        right: crate::Unity::Mathematics::int2,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_int2_int2_math_ShuffleComponent37(
        left: crate::Unity::Mathematics::int2,
        right: crate::Unity::Mathematics::int2,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_int2_int2_math_ShuffleComponent_math_ShuffleComponent38(
        left: crate::Unity::Mathematics::int2,
        right: crate::Unity::Mathematics::int2,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_int2_int2_math_ShuffleComponent_math_ShuffleComponent_math_ShuffleComponent39(
        left: crate::Unity::Mathematics::int2,
        right: crate::Unity::Mathematics::int2,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
        w: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_int3_int3_40(
        left: crate::Unity::Mathematics::int3,
        right: crate::Unity::Mathematics::int3,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_int3_int3_math_ShuffleComponent41(
        left: crate::Unity::Mathematics::int3,
        right: crate::Unity::Mathematics::int3,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_int3_int3_math_ShuffleComponent_math_ShuffleComponent42(
        left: crate::Unity::Mathematics::int3,
        right: crate::Unity::Mathematics::int3,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_int3_int3_math_ShuffleComponent_math_ShuffleComponent_math_ShuffleComponent43(
        left: crate::Unity::Mathematics::int3,
        right: crate::Unity::Mathematics::int3,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
        w: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_int4_int4_44(
        left: crate::Unity::Mathematics::int4,
        right: crate::Unity::Mathematics::int4,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_int4_int4_math_ShuffleComponent45(
        left: crate::Unity::Mathematics::int4,
        right: crate::Unity::Mathematics::int4,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_int4_int4_math_ShuffleComponent_math_ShuffleComponent46(
        left: crate::Unity::Mathematics::int4,
        right: crate::Unity::Mathematics::int4,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_int4_int4_math_ShuffleComponent_math_ShuffleComponent_math_ShuffleComponent47(
        left: crate::Unity::Mathematics::int4,
        right: crate::Unity::Mathematics::int4,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
        w: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_uint2_uint2_48(
        left: crate::Unity::Mathematics::uint2,
        right: crate::Unity::Mathematics::uint2,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_uint2_uint2_math_ShuffleComponent49(
        left: crate::Unity::Mathematics::uint2,
        right: crate::Unity::Mathematics::uint2,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_uint2_uint2_math_ShuffleComponent_math_ShuffleComponent50(
        left: crate::Unity::Mathematics::uint2,
        right: crate::Unity::Mathematics::uint2,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_uint2_uint2_math_ShuffleComponent_math_ShuffleComponent_math_ShuffleComponent51(
        left: crate::Unity::Mathematics::uint2,
        right: crate::Unity::Mathematics::uint2,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
        w: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_uint3_uint3_52(
        left: crate::Unity::Mathematics::uint3,
        right: crate::Unity::Mathematics::uint3,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_uint3_uint3_math_ShuffleComponent53(
        left: crate::Unity::Mathematics::uint3,
        right: crate::Unity::Mathematics::uint3,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_uint3_uint3_math_ShuffleComponent_math_ShuffleComponent54(
        left: crate::Unity::Mathematics::uint3,
        right: crate::Unity::Mathematics::uint3,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_uint3_uint3_math_ShuffleComponent_math_ShuffleComponent_math_ShuffleComponent55(
        left: crate::Unity::Mathematics::uint3,
        right: crate::Unity::Mathematics::uint3,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
        w: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_uint4_uint4_56(
        left: crate::Unity::Mathematics::uint4,
        right: crate::Unity::Mathematics::uint4,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_uint4_uint4_math_ShuffleComponent57(
        left: crate::Unity::Mathematics::uint4,
        right: crate::Unity::Mathematics::uint4,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_uint4_uint4_math_ShuffleComponent_math_ShuffleComponent58(
        left: crate::Unity::Mathematics::uint4,
        right: crate::Unity::Mathematics::uint4,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_uint4_uint4_math_ShuffleComponent_math_ShuffleComponent_math_ShuffleComponent59(
        left: crate::Unity::Mathematics::uint4,
        right: crate::Unity::Mathematics::uint4,
        x: crate::Unity::Mathematics::math_ShuffleComponent,
        y: crate::Unity::Mathematics::math_ShuffleComponent,
        z: crate::Unity::Mathematics::math_ShuffleComponent,
        w: crate::Unity::Mathematics::math_ShuffleComponent,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("shuffle", (left, right, x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn sign_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sign", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sign_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sign", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sign_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sign", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sign_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sign", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sign_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sign", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sign_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sign", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sign_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sign", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sign_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sign", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sin_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sin_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sin_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sin_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sin_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sin_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sin_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sin_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sin", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sincos_double2_5(
        x: crate::Unity::Mathematics::double2,
        s: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::double2>,
        c: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::double2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sincos", (x, s, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn sincos_double3_6(
        x: crate::Unity::Mathematics::double3,
        s: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::double3>,
        c: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::double3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sincos", (x, s, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn sincos_double4_7(
        x: crate::Unity::Mathematics::double4,
        s: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::double4>,
        c: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::double4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sincos", (x, s, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn sincos_f32_0(
        x: f32,
        s: quest_hook::libil2cpp::ByRefMut<f32>,
        c: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sincos", (x, s, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn sincos_f64_4(
        x: f64,
        s: quest_hook::libil2cpp::ByRefMut<f64>,
        c: quest_hook::libil2cpp::ByRefMut<f64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sincos", (x, s, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn sincos_float2_1(
        x: crate::Unity::Mathematics::float2,
        s: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float2>,
        c: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sincos", (x, s, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn sincos_float3_2(
        x: crate::Unity::Mathematics::float3,
        s: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float3>,
        c: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sincos", (x, s, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn sincos_float4_3(
        x: crate::Unity::Mathematics::float4,
        s: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float4>,
        c: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sincos", (x, s, c))?;
        Ok(__cordl_ret.into())
    }
    pub fn sinh_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sinh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sinh_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sinh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sinh_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sinh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sinh_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sinh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sinh_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sinh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sinh_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sinh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sinh_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sinh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sinh_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sinh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn slerp(
        q1: crate::Unity::Mathematics::quaternion,
        q2: crate::Unity::Mathematics::quaternion,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("slerp", (q1, q2, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn smoothstep_double2_double2_double2_5(
        a: crate::Unity::Mathematics::double2,
        b: crate::Unity::Mathematics::double2,
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("smoothstep", (a, b, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn smoothstep_double3_double3_double3_6(
        a: crate::Unity::Mathematics::double3,
        b: crate::Unity::Mathematics::double3,
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("smoothstep", (a, b, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn smoothstep_double4_double4_double4_7(
        a: crate::Unity::Mathematics::double4,
        b: crate::Unity::Mathematics::double4,
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("smoothstep", (a, b, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn smoothstep_f32_f32_f32_0(
        a: f32,
        b: f32,
        x: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("smoothstep", (a, b, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn smoothstep_f64_f64_f64_4(
        a: f64,
        b: f64,
        x: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("smoothstep", (a, b, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn smoothstep_float2_float2_float2_1(
        a: crate::Unity::Mathematics::float2,
        b: crate::Unity::Mathematics::float2,
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("smoothstep", (a, b, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn smoothstep_float3_float3_float3_2(
        a: crate::Unity::Mathematics::float3,
        b: crate::Unity::Mathematics::float3,
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("smoothstep", (a, b, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn smoothstep_float4_float4_float4_3(
        a: crate::Unity::Mathematics::float4,
        b: crate::Unity::Mathematics::float4,
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("smoothstep", (a, b, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sqrt_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sqrt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sqrt_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sqrt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sqrt_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sqrt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sqrt_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sqrt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sqrt_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sqrt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sqrt_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sqrt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sqrt_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sqrt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn sqrt_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("sqrt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn step_double2_double2_5(
        y: crate::Unity::Mathematics::double2,
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("step", (y, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn step_double3_double3_6(
        y: crate::Unity::Mathematics::double3,
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("step", (y, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn step_double4_double4_7(
        y: crate::Unity::Mathematics::double4,
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("step", (y, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn step_f32_f32_0(y: f32, x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("step", (y, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn step_f64_f64_4(y: f64, x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("step", (y, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn step_float2_float2_1(
        y: crate::Unity::Mathematics::float2,
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("step", (y, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn step_float3_float3_2(
        y: crate::Unity::Mathematics::float3,
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("step", (y, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn step_float4_float4_3(
        y: crate::Unity::Mathematics::float4,
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("step", (y, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tan_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tan_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tan_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tan_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tan_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tan_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tan_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tan_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tan", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tanh_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tanh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tanh_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tanh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tanh_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tanh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tanh_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tanh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tanh_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tanh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tanh_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tanh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tanh_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tanh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tanh_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tanh", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn transform_RigidTransform_float3_2(
        a: crate::Unity::Mathematics::RigidTransform,
        pos: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transform", (a, pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn transform_double4x4_double3_0(
        a: crate::Unity::Mathematics::double4x4,
        b: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transform", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn transform_float4x4_float3_1(
        a: crate::Unity::Mathematics::float4x4,
        b: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transform", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_bool2x2_0(
        v: crate::Unity::Mathematics::bool2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_bool2x3_1(
        v: crate::Unity::Mathematics::bool2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_bool2x4_2(
        v: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_bool3x2_3(
        v: crate::Unity::Mathematics::bool3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_bool3x3_4(
        v: crate::Unity::Mathematics::bool3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_bool3x4_5(
        v: crate::Unity::Mathematics::bool3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x3> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_bool4x2_6(
        v: crate::Unity::Mathematics::bool4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_bool4x3_7(
        v: crate::Unity::Mathematics::bool4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_bool4x4_8(
        v: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_double2x2_9(
        v: crate::Unity::Mathematics::double2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x2> {
        let __cordl_ret: crate::Unity::Mathematics::double2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_double2x3_10(
        v: crate::Unity::Mathematics::double2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x2> {
        let __cordl_ret: crate::Unity::Mathematics::double3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_double2x4_11(
        v: crate::Unity::Mathematics::double2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x2> {
        let __cordl_ret: crate::Unity::Mathematics::double4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_double3x2_12(
        v: crate::Unity::Mathematics::double3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x3> {
        let __cordl_ret: crate::Unity::Mathematics::double2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_double3x3_13(
        v: crate::Unity::Mathematics::double3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x3> {
        let __cordl_ret: crate::Unity::Mathematics::double3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_double3x4_14(
        v: crate::Unity::Mathematics::double3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x3> {
        let __cordl_ret: crate::Unity::Mathematics::double4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_double4x2_15(
        v: crate::Unity::Mathematics::double4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2x4> {
        let __cordl_ret: crate::Unity::Mathematics::double2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_double4x3_16(
        v: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3x4> {
        let __cordl_ret: crate::Unity::Mathematics::double3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_double4x4_17(
        v: crate::Unity::Mathematics::double4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4x4> {
        let __cordl_ret: crate::Unity::Mathematics::double4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_float2x2_18(
        v: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_float2x3_19(
        v: crate::Unity::Mathematics::float2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x2> {
        let __cordl_ret: crate::Unity::Mathematics::float3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_float2x4_20(
        v: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x2> {
        let __cordl_ret: crate::Unity::Mathematics::float4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_float3x2_21(
        v: crate::Unity::Mathematics::float3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x3> {
        let __cordl_ret: crate::Unity::Mathematics::float2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_float3x3_22(
        v: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x3> {
        let __cordl_ret: crate::Unity::Mathematics::float3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_float3x4_23(
        v: crate::Unity::Mathematics::float3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x3> {
        let __cordl_ret: crate::Unity::Mathematics::float4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_float4x2_24(
        v: crate::Unity::Mathematics::float4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x4> {
        let __cordl_ret: crate::Unity::Mathematics::float2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_float4x3_25(
        v: crate::Unity::Mathematics::float4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3x4> {
        let __cordl_ret: crate::Unity::Mathematics::float3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_float4x4_26(
        v: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4x4> {
        let __cordl_ret: crate::Unity::Mathematics::float4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_int2x2_27(
        v: crate::Unity::Mathematics::int2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x2> {
        let __cordl_ret: crate::Unity::Mathematics::int2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_int2x3_28(
        v: crate::Unity::Mathematics::int2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x2> {
        let __cordl_ret: crate::Unity::Mathematics::int3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_int2x4_29(
        v: crate::Unity::Mathematics::int2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x2> {
        let __cordl_ret: crate::Unity::Mathematics::int4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_int3x2_30(
        v: crate::Unity::Mathematics::int3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x3> {
        let __cordl_ret: crate::Unity::Mathematics::int2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_int3x3_31(
        v: crate::Unity::Mathematics::int3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x3> {
        let __cordl_ret: crate::Unity::Mathematics::int3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_int3x4_32(
        v: crate::Unity::Mathematics::int3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x3> {
        let __cordl_ret: crate::Unity::Mathematics::int4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_int4x2_33(
        v: crate::Unity::Mathematics::int4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2x4> {
        let __cordl_ret: crate::Unity::Mathematics::int2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_int4x3_34(
        v: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3x4> {
        let __cordl_ret: crate::Unity::Mathematics::int3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_int4x4_35(
        v: crate::Unity::Mathematics::int4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4x4> {
        let __cordl_ret: crate::Unity::Mathematics::int4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_uint2x2_36(
        v: crate::Unity::Mathematics::uint2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_uint2x3_37(
        v: crate::Unity::Mathematics::uint2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_uint2x4_38(
        v: crate::Unity::Mathematics::uint2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_uint3x2_39(
        v: crate::Unity::Mathematics::uint3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_uint3x3_40(
        v: crate::Unity::Mathematics::uint3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_uint3x4_41(
        v: crate::Unity::Mathematics::uint3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_uint4x2_42(
        v: crate::Unity::Mathematics::uint4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_uint4x3_43(
        v: crate::Unity::Mathematics::uint4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn transpose_uint4x4_44(
        v: crate::Unity::Mathematics::uint4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("transpose", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn trunc_double2_5(
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("trunc", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn trunc_double3_6(
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("trunc", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn trunc_double4_7(
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("trunc", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn trunc_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("trunc", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn trunc_f64_4(x: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("trunc", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn trunc_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("trunc", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn trunc_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("trunc", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn trunc_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("trunc", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tzcnt_i32_0(x: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tzcnt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tzcnt_i64_8(x: i64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tzcnt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tzcnt_int2_1(
        x: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tzcnt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tzcnt_int3_2(
        x: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tzcnt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tzcnt_int4_3(
        x: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tzcnt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tzcnt_u32_4(x: u32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tzcnt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tzcnt_u64_9(x: u64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tzcnt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tzcnt_uint2_5(
        x: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tzcnt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tzcnt_uint3_6(
        x: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tzcnt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn tzcnt_uint4_7(
        x: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("tzcnt", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2_bool2_4(
        v: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2_double2_10(
        v: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2_float2_8(
        v: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2_int2_6(
        v: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2_u32_2(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2_u32_u32_0(
        x: u32,
        y: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2_uint2_1(
        xy: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2", (xy))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x2__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x2_bool2x2_4(
        v: crate::Unity::Mathematics::bool2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x2_double2x2_10(
        v: crate::Unity::Mathematics::double2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x2_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x2_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x2_float2x2_8(
        v: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x2_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x2_int2x2_6(
        v: crate::Unity::Mathematics::int2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x2_u32_2(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x2_u32_u32_u32_u32_1(
        m00: u32,
        m01: u32,
        m10: u32,
        m11: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x2", (m00, m01, m10, m11))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x2_uint2_uint2_0(
        c0: crate::Unity::Mathematics::uint2,
        c1: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x2", (c0, c1))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x3__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x3_bool2x3_4(
        v: crate::Unity::Mathematics::bool2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x3_double2x3_10(
        v: crate::Unity::Mathematics::double2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x3_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x3_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x3_float2x3_8(
        v: crate::Unity::Mathematics::float2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x3_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x3_int2x3_6(
        v: crate::Unity::Mathematics::int2x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x3_u32_2(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x3_u32_u32_u32_u32_u32_u32_1(
        m00: u32,
        m01: u32,
        m02: u32,
        m10: u32,
        m11: u32,
        m12: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x3", (m00, m01, m02, m10, m11, m12))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x3_uint2_uint2_uint2_0(
        c0: crate::Unity::Mathematics::uint2,
        c1: crate::Unity::Mathematics::uint2,
        c2: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x3", (c0, c1, c2))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x4__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x4_bool2x4_4(
        v: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x4_double2x4_10(
        v: crate::Unity::Mathematics::double2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x4_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x4_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x4_float2x4_8(
        v: crate::Unity::Mathematics::float2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x4_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x4_int2x4_6(
        v: crate::Unity::Mathematics::int2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x4_u32_2(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x4_u32_u32_u32_u32_u32_u32_u32_u32_1(
        m00: u32,
        m01: u32,
        m02: u32,
        m03: u32,
        m10: u32,
        m11: u32,
        m12: u32,
        m13: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x4", (m00, m01, m02, m03, m10, m11, m12, m13))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint2x4_uint2_uint2_uint2_uint2_0(
        c0: crate::Unity::Mathematics::uint2,
        c1: crate::Unity::Mathematics::uint2,
        c2: crate::Unity::Mathematics::uint2,
        c3: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint2x4", (c0, c1, c2, c3))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3__cordl_bool5(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3_bool3_6(
        v: crate::Unity::Mathematics::bool3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3_double3_12(
        v: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3_f32_9(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3_f64_11(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3_float3_10(
        v: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3_i32_7(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3_int3_8(
        v: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3_u32_4(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3_u32_u32_u32_0(
        x: u32,
        y: u32,
        z: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3_u32_uint2_1(
        x: u32,
        yz: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3", (x, yz))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3_uint2_u32_2(
        xy: crate::Unity::Mathematics::uint2,
        z: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3", (xy, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3_uint3_3(
        xyz: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3", (xyz))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x2__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x2_bool3x2_4(
        v: crate::Unity::Mathematics::bool3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x2_double3x2_10(
        v: crate::Unity::Mathematics::double3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x2_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x2_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x2_float3x2_8(
        v: crate::Unity::Mathematics::float3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x2_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x2_int3x2_6(
        v: crate::Unity::Mathematics::int3x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x2_u32_2(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x2_u32_u32_u32_u32_u32_u32_1(
        m00: u32,
        m01: u32,
        m10: u32,
        m11: u32,
        m20: u32,
        m21: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x2", (m00, m01, m10, m11, m20, m21))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x2_uint3_uint3_0(
        c0: crate::Unity::Mathematics::uint3,
        c1: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x2", (c0, c1))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x3__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x3_bool3x3_4(
        v: crate::Unity::Mathematics::bool3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x3_double3x3_10(
        v: crate::Unity::Mathematics::double3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x3_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x3_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x3_float3x3_8(
        v: crate::Unity::Mathematics::float3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x3_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x3_int3x3_6(
        v: crate::Unity::Mathematics::int3x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x3_u32_2(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x3_u32_u32_u32_u32_u32_u32_u32_u32_u32_1(
        m00: u32,
        m01: u32,
        m02: u32,
        m10: u32,
        m11: u32,
        m12: u32,
        m20: u32,
        m21: u32,
        m22: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x3", (m00, m01, m02, m10, m11, m12, m20, m21, m22))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x3_uint3_uint3_uint3_0(
        c0: crate::Unity::Mathematics::uint3,
        c1: crate::Unity::Mathematics::uint3,
        c2: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x3", (c0, c1, c2))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x4__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x4_bool3x4_4(
        v: crate::Unity::Mathematics::bool3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x4_double3x4_10(
        v: crate::Unity::Mathematics::double3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x4_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x4_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x4_float3x4_8(
        v: crate::Unity::Mathematics::float3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x4_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x4_int3x4_6(
        v: crate::Unity::Mathematics::int3x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x4_u32_2(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x4_u32_u32_u32_u32_u32_u32_u32_u32_u32_u32_u32_u32_1(
        m00: u32,
        m01: u32,
        m02: u32,
        m03: u32,
        m10: u32,
        m11: u32,
        m12: u32,
        m13: u32,
        m20: u32,
        m21: u32,
        m22: u32,
        m23: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "uint3x4",
                (m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn uint3x4_uint3_uint3_uint3_uint3_0(
        c0: crate::Unity::Mathematics::uint3,
        c1: crate::Unity::Mathematics::uint3,
        c2: crate::Unity::Mathematics::uint3,
        c3: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint3x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint3x4", (c0, c1, c2, c3))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4__cordl_bool9(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4_bool4_10(
        v: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4_double4_16(
        v: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4_f32_13(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4_f64_15(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4_float4_14(
        v: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4_i32_11(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4_int4_12(
        v: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4_u32_8(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4_u32_u32_u32_u32_0(
        x: u32,
        y: u32,
        z: u32,
        w: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4", (x, y, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4_u32_u32_uint2_1(
        x: u32,
        y: u32,
        zw: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4", (x, y, zw))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4_u32_uint2_u32_2(
        x: u32,
        yz: crate::Unity::Mathematics::uint2,
        w: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4", (x, yz, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4_u32_uint3_3(
        x: u32,
        yzw: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4", (x, yzw))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4_uint2_u32_u32_4(
        xy: crate::Unity::Mathematics::uint2,
        z: u32,
        w: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4", (xy, z, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4_uint2_uint2_5(
        xy: crate::Unity::Mathematics::uint2,
        zw: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4", (xy, zw))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4_uint3_u32_6(
        xyz: crate::Unity::Mathematics::uint3,
        w: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4", (xyz, w))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4_uint4_7(
        xyzw: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4", (xyzw))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x2__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x2_bool4x2_4(
        v: crate::Unity::Mathematics::bool4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x2_double4x2_10(
        v: crate::Unity::Mathematics::double4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x2_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x2_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x2_float4x2_8(
        v: crate::Unity::Mathematics::float4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x2_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x2_int4x2_6(
        v: crate::Unity::Mathematics::int4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x2_u32_2(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x2", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x2_u32_u32_u32_u32_u32_u32_u32_u32_1(
        m00: u32,
        m01: u32,
        m10: u32,
        m11: u32,
        m20: u32,
        m21: u32,
        m30: u32,
        m31: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x2", (m00, m01, m10, m11, m20, m21, m30, m31))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x2_uint4_uint4_0(
        c0: crate::Unity::Mathematics::uint4,
        c1: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x2> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x2", (c0, c1))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x3__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x3_bool4x3_4(
        v: crate::Unity::Mathematics::bool4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x3_double4x3_10(
        v: crate::Unity::Mathematics::double4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x3_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x3_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x3_float4x3_8(
        v: crate::Unity::Mathematics::float4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x3_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x3_int4x3_6(
        v: crate::Unity::Mathematics::int4x3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x3_u32_2(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x3", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x3_u32_u32_u32_u32_u32_u32_u32_u32_u32_u32_u32_u32_1(
        m00: u32,
        m01: u32,
        m02: u32,
        m10: u32,
        m11: u32,
        m12: u32,
        m20: u32,
        m21: u32,
        m22: u32,
        m30: u32,
        m31: u32,
        m32: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "uint4x3",
                (m00, m01, m02, m10, m11, m12, m20, m21, m22, m30, m31, m32),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x3_uint4_uint4_uint4_0(
        c0: crate::Unity::Mathematics::uint4,
        c1: crate::Unity::Mathematics::uint4,
        c2: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x3> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x3", (c0, c1, c2))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x4__cordl_bool3(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x4_bool4x4_4(
        v: crate::Unity::Mathematics::bool4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x4_double4x4_10(
        v: crate::Unity::Mathematics::double4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x4_f32_7(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x4_f64_9(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x4_float4x4_8(
        v: crate::Unity::Mathematics::float4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x4_i32_5(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x4_int4x4_6(
        v: crate::Unity::Mathematics::int4x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x4_u32_2(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x4", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x4_u32_u32_u32_u32_u32_u32_u32_u32_u32_u32_u32_u32_u32_u32_u32_u32_1(
        m00: u32,
        m01: u32,
        m02: u32,
        m03: u32,
        m10: u32,
        m11: u32,
        m12: u32,
        m13: u32,
        m20: u32,
        m21: u32,
        m22: u32,
        m23: u32,
        m30: u32,
        m31: u32,
        m32: u32,
        m33: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "uint4x4",
                (
                    m00,
                    m01,
                    m02,
                    m03,
                    m10,
                    m11,
                    m12,
                    m13,
                    m20,
                    m21,
                    m22,
                    m23,
                    m30,
                    m31,
                    m32,
                    m33,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn uint4x4_uint4_uint4_uint4_uint4_0(
        c0: crate::Unity::Mathematics::uint4,
        c1: crate::Unity::Mathematics::uint4,
        c2: crate::Unity::Mathematics::uint4,
        c3: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4x4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("uint4x4", (c0, c1, c2, c3))?;
        Ok(__cordl_ret.into())
    }
    pub fn unitexp(
        q: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unitexp", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn unitlog(
        q: crate::Unity::Mathematics::quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unitlog", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn unlerp_double2_double2_double2_5(
        a: crate::Unity::Mathematics::double2,
        b: crate::Unity::Mathematics::double2,
        x: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unlerp", (a, b, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn unlerp_double3_double3_double3_6(
        a: crate::Unity::Mathematics::double3,
        b: crate::Unity::Mathematics::double3,
        x: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unlerp", (a, b, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn unlerp_double4_double4_double4_7(
        a: crate::Unity::Mathematics::double4,
        b: crate::Unity::Mathematics::double4,
        x: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unlerp", (a, b, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn unlerp_f32_f32_f32_0(
        a: f32,
        b: f32,
        x: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unlerp", (a, b, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn unlerp_f64_f64_f64_4(
        a: f64,
        b: f64,
        x: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unlerp", (a, b, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn unlerp_float2_float2_float2_1(
        a: crate::Unity::Mathematics::float2,
        b: crate::Unity::Mathematics::float2,
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unlerp", (a, b, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn unlerp_float3_float3_float3_2(
        a: crate::Unity::Mathematics::float3,
        b: crate::Unity::Mathematics::float3,
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unlerp", (a, b, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn unlerp_float4_float4_float4_3(
        a: crate::Unity::Mathematics::float4,
        b: crate::Unity::Mathematics::float4,
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unlerp", (a, b, x))?;
        Ok(__cordl_ret.into())
    }
    pub fn unpackhi_double4_double4_1(
        a: crate::Unity::Mathematics::double4,
        b: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unpackhi", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn unpackhi_float4_float4_0(
        a: crate::Unity::Mathematics::float4,
        b: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unpackhi", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn unpacklo_double4_double4_1(
        a: crate::Unity::Mathematics::double4,
        b: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unpacklo", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn unpacklo_float4_float4_0(
        a: crate::Unity::Mathematics::float4,
        b: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("unpacklo", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn up() -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("up", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+math")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Mathematics::math {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Mathematics+math+IntFloatUnion")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct math_IntFloatUnion {
    padding: [u8; 4usize],
}
#[cfg(feature = "Unity+Mathematics+math+IntFloatUnion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::math_IntFloatUnion =>
    "Unity.Mathematics"."math/IntFloatUnion"
);
#[cfg(feature = "Unity+Mathematics+math+IntFloatUnion")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Mathematics::math_IntFloatUnion {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+math+IntFloatUnion")]
impl crate::Unity::Mathematics::math_IntFloatUnion {}
#[cfg(feature = "Unity+Mathematics+math+LongDoubleUnion")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct math_LongDoubleUnion {
    padding: [u8; 8usize],
}
#[cfg(feature = "Unity+Mathematics+math+LongDoubleUnion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::math_LongDoubleUnion =>
    "Unity.Mathematics"."math/LongDoubleUnion"
);
#[cfg(feature = "Unity+Mathematics+math+LongDoubleUnion")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Mathematics::math_LongDoubleUnion {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+math+LongDoubleUnion")]
impl crate::Unity::Mathematics::math_LongDoubleUnion {}
#[cfg(feature = "Unity+Mathematics+math+RotationOrder")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum math_RotationOrder {
    Default = 4u8,
    XYZ = 0u8,
    XZY = 1u8,
    YXZ = 2u8,
    YZX = 3u8,
    ZYX = 5u8,
}
#[cfg(feature = "Unity+Mathematics+math+RotationOrder")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::math_RotationOrder =>
    "Unity.Mathematics"."math/RotationOrder"
);
#[cfg(feature = "Unity+Mathematics+math+ShuffleComponent")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum math_ShuffleComponent {
    LeftW = 3u8,
    LeftX = 0u8,
    LeftY = 1u8,
    LeftZ = 2u8,
    RightW = 7u8,
    RightX = 4u8,
    RightY = 5u8,
    RightZ = 6u8,
}
#[cfg(feature = "Unity+Mathematics+math+ShuffleComponent")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::math_ShuffleComponent =>
    "Unity.Mathematics"."math/ShuffleComponent"
);
