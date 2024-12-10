#[cfg(feature = "Unity+Mathematics+Geometry+MinMaxAABB")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MinMaxAABB {
    pub Min: crate::Unity::Mathematics::float3,
    pub Max: crate::Unity::Mathematics::float3,
}
#[cfg(feature = "Unity+Mathematics+Geometry+MinMaxAABB")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::Geometry::MinMaxAABB =>
    "Unity.Mathematics.Geometry"."MinMaxAABB"
);
#[cfg(feature = "Unity+Mathematics+Geometry+MinMaxAABB")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Mathematics::Geometry::MinMaxAABB {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+Geometry+MinMaxAABB")]
impl crate::Unity::Mathematics::Geometry::MinMaxAABB {
    pub fn Contains_MinMaxAABB1(
        &mut self,
        aabb: crate::Unity::Mathematics::Geometry::MinMaxAABB,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Contains",
            (aabb),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains_float3_0(
        &mut self,
        point: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Contains",
            (point),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Encapsulate_MinMaxAABB0(
        &mut self,
        aabb: crate::Unity::Mathematics::Geometry::MinMaxAABB,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Encapsulate",
            (aabb),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Encapsulate_float3_1(
        &mut self,
        point: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Encapsulate",
            (point),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        other: crate::Unity::Mathematics::Geometry::MinMaxAABB,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Expand(
        &mut self,
        signedDistance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Expand",
            (signedDistance),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Overlaps(
        &mut self,
        aabb: crate::Unity::Mathematics::Geometry::MinMaxAABB,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Overlaps",
            (aabb),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        min: crate::Unity::Mathematics::float3,
        max: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (min, max),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Center(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Center",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Extents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Extents",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HalfExtents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_HalfExtents",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsValid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SurfaceArea(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_SurfaceArea",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+Geometry+MinMaxAABB")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Mathematics::Geometry::MinMaxAABB>>
for crate::Unity::Mathematics::Geometry::MinMaxAABB {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Mathematics::Geometry::MinMaxAABB> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+Geometry+MinMaxAABB")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Mathematics::Geometry::MinMaxAABB>>
for crate::Unity::Mathematics::Geometry::MinMaxAABB {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::Unity::Mathematics::Geometry::MinMaxAABB,
    > {
        todo!()
    }
}
