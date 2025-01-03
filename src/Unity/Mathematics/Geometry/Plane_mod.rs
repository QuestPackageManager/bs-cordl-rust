#[cfg(feature = "Unity+Mathematics+Geometry+Plane")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Plane {
    pub NormalAndDistance: crate::Unity::Mathematics::float4,
}
#[cfg(feature = "Unity+Mathematics+Geometry+Plane")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::Geometry::Plane =>
    "Unity.Mathematics.Geometry"."Plane"
);
#[cfg(feature = "Unity+Mathematics+Geometry+Plane")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Mathematics::Geometry::Plane {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+Geometry+Plane")]
impl crate::Unity::Mathematics::Geometry::Plane {
    pub fn CheckPlaneIsNormalized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckPlaneIsNormalized",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateFromUnitNormalAndDistance(
        unitNormal: crate::Unity::Mathematics::float3,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::Geometry::Plane> {
        let __cordl_ret: crate::Unity::Mathematics::Geometry::Plane = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateFromUnitNormalAndDistance", (unitNormal, distance))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateFromUnitNormalAndPointInPlane(
        unitNormal: crate::Unity::Mathematics::float3,
        pointInPlane: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::Geometry::Plane> {
        let __cordl_ret: crate::Unity::Mathematics::Geometry::Plane = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateFromUnitNormalAndPointInPlane", (unitNormal, pointInPlane))?;
        Ok(__cordl_ret.into())
    }
    pub fn Normalize_Plane0(
        plane: crate::Unity::Mathematics::Geometry::Plane,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::Geometry::Plane> {
        let __cordl_ret: crate::Unity::Mathematics::Geometry::Plane = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Normalize", (plane))?;
        Ok(__cordl_ret.into())
    }
    pub fn Normalize_float4_1(
        planeCoefficients: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Normalize", (planeCoefficients))?;
        Ok(__cordl_ret.into())
    }
    pub fn Projection(
        &mut self,
        point: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Projection",
            (point),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SignedDistanceToPoint(
        &mut self,
        point: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SignedDistanceToPoint",
            (point),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_f32_f32_f32_0(
        &mut self,
        coefficientA: f32,
        coefficientB: f32,
        coefficientC: f32,
        coefficientD: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (coefficientA, coefficientB, coefficientC, coefficientD),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float3_f32_1(
        &mut self,
        normal: crate::Unity::Mathematics::float3,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (normal, distance),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float3_float3_2(
        &mut self,
        normal: crate::Unity::Mathematics::float3,
        pointInPlane: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (normal, pointInPlane),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float3_float3_float3_3(
        &mut self,
        vector1InPlane: crate::Unity::Mathematics::float3,
        vector2InPlane: crate::Unity::Mathematics::float3,
        pointInPlane: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (vector1InPlane, vector2InPlane, pointInPlane),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Distance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Distance",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Flipped(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::Geometry::Plane> {
        let __cordl_ret: crate::Unity::Mathematics::Geometry::Plane = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Flipped",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Normal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Normal",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        plane: crate::Unity::Mathematics::Geometry::Plane,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (plane))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Distance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Distance",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Normal(
        &mut self,
        value: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Normal",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
