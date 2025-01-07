#[cfg(feature = "Unity+Mathematics+Random")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Random {
    pub state: u32,
}
#[cfg(feature = "Unity+Mathematics+Random")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Mathematics::Random {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Mathematics";
    const CLASS_NAME: &'static str = "Random";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "Unity+Mathematics+Random")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::Mathematics::Random {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Mathematics+Random")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Unity::Mathematics::Random {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "Unity+Mathematics+Random")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::Mathematics::Random {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "Unity+Mathematics+Random")]
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Mathematics::Random {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "Unity+Mathematics+Random")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::Random {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+Random")]
impl crate::Unity::Mathematics::Random {
    pub fn CheckIndexForHash(
        index: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckIndexForHash", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckInitState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckInitState",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckNextIntMax(
        &mut self,
        max: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckNextIntMax",
            (max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckNextIntMinMax(
        &mut self,
        min: i32,
        max: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckNextIntMinMax",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckNextUIntMinMax(
        &mut self,
        min: u32,
        max: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckNextUIntMinMax",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckState",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateFromIndex(
        index: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::Random> {
        let __cordl_ret: crate::Unity::Mathematics::Random = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateFromIndex", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitState(
        &mut self,
        seed: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InitState",
            (seed),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextBool(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextBool",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextBool2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextBool2",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextBool3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool3> {
        let __cordl_ret: crate::Unity::Mathematics::bool3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextBool3",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextBool4(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4> {
        let __cordl_ret: crate::Unity::Mathematics::bool4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextBool4",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble2Direction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextDouble2Direction",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble2_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextDouble2",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble2_double2_1(
        &mut self,
        max: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextDouble2",
            (max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble2_double2_double2_2(
        &mut self,
        min: crate::Unity::Mathematics::double2,
        max: crate::Unity::Mathematics::double2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double2> {
        let __cordl_ret: crate::Unity::Mathematics::double2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextDouble2",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble3Direction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextDouble3Direction",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble3_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextDouble3",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble3_double3_1(
        &mut self,
        max: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextDouble3",
            (max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble3_double3_double3_2(
        &mut self,
        min: crate::Unity::Mathematics::double3,
        max: crate::Unity::Mathematics::double3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double3> {
        let __cordl_ret: crate::Unity::Mathematics::double3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextDouble3",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble4_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextDouble4",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble4_double4_1(
        &mut self,
        max: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextDouble4",
            (max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble4_double4_double4_2(
        &mut self,
        min: crate::Unity::Mathematics::double4,
        max: crate::Unity::Mathematics::double4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::double4> {
        let __cordl_ret: crate::Unity::Mathematics::double4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextDouble4",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble_0(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextDouble",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble_f64_1(&mut self, max: f64) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextDouble",
            (max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextDouble_f64_f64_2(
        &mut self,
        min: f64,
        max: f64,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextDouble",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat2Direction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextFloat2Direction",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat2_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextFloat2",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat2_float2_1(
        &mut self,
        max: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextFloat2",
            (max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat2_float2_float2_2(
        &mut self,
        min: crate::Unity::Mathematics::float2,
        max: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextFloat2",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat3Direction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextFloat3Direction",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat3_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextFloat3",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat3_float3_1(
        &mut self,
        max: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextFloat3",
            (max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat3_float3_float3_2(
        &mut self,
        min: crate::Unity::Mathematics::float3,
        max: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextFloat3",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat4_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextFloat4",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat4_float4_1(
        &mut self,
        max: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextFloat4",
            (max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat4_float4_float4_2(
        &mut self,
        min: crate::Unity::Mathematics::float4,
        max: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextFloat4",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat_0(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextFloat",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat_f32_1(&mut self, max: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextFloat",
            (max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextFloat_f32_f32_2(
        &mut self,
        min: f32,
        max: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextFloat",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextInt2_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextInt2",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextInt2_int2_1(
        &mut self,
        max: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextInt2",
            (max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextInt2_int2_int2_2(
        &mut self,
        min: crate::Unity::Mathematics::int2,
        max: crate::Unity::Mathematics::int2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int2> {
        let __cordl_ret: crate::Unity::Mathematics::int2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextInt2",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextInt3_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextInt3",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextInt3_int3_1(
        &mut self,
        max: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextInt3",
            (max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextInt3_int3_int3_2(
        &mut self,
        min: crate::Unity::Mathematics::int3,
        max: crate::Unity::Mathematics::int3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int3> {
        let __cordl_ret: crate::Unity::Mathematics::int3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextInt3",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextInt4_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextInt4",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextInt4_int4_1(
        &mut self,
        max: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextInt4",
            (max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextInt4_int4_int4_2(
        &mut self,
        min: crate::Unity::Mathematics::int4,
        max: crate::Unity::Mathematics::int4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::int4> {
        let __cordl_ret: crate::Unity::Mathematics::int4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextInt4",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextInt_0(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextInt",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextInt_i32_1(&mut self, max: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextInt",
            (max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextInt_i32_i32_2(
        &mut self,
        min: i32,
        max: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextInt",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextQuaternionRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::quaternion> {
        let __cordl_ret: crate::Unity::Mathematics::quaternion = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextQuaternionRotation",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextState(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextState",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt2_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextUInt2",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt2_uint2_1(
        &mut self,
        max: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextUInt2",
            (max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt2_uint2_uint2_2(
        &mut self,
        min: crate::Unity::Mathematics::uint2,
        max: crate::Unity::Mathematics::uint2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint2> {
        let __cordl_ret: crate::Unity::Mathematics::uint2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextUInt2",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt3_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextUInt3",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt3_uint3_1(
        &mut self,
        max: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextUInt3",
            (max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt3_uint3_uint3_2(
        &mut self,
        min: crate::Unity::Mathematics::uint3,
        max: crate::Unity::Mathematics::uint3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint3> {
        let __cordl_ret: crate::Unity::Mathematics::uint3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextUInt3",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt4_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextUInt4",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt4_uint4_1(
        &mut self,
        max: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextUInt4",
            (max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt4_uint4_uint4_2(
        &mut self,
        min: crate::Unity::Mathematics::uint4,
        max: crate::Unity::Mathematics::uint4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::uint4> {
        let __cordl_ret: crate::Unity::Mathematics::uint4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextUInt4",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt_0(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextUInt",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt_u32_1(&mut self, max: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextUInt",
            (max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn NextUInt_u32_u32_2(
        &mut self,
        min: u32,
        max: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "NextUInt",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WangHash(n: u32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WangHash", (n))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        seed: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (seed),
        )?;
        Ok(__cordl_ret.into())
    }
}
