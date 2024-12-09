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
