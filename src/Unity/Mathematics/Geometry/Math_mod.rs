#[cfg(feature = "Unity+Mathematics+Geometry+Math")]
#[repr(C)]
#[derive(Debug)]
pub struct Math {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Mathematics+Geometry+Math")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::Geometry::Math =>
    "Unity.Mathematics.Geometry"."Math"
);
#[cfg(feature = "Unity+Mathematics+Geometry+Math")]
impl std::ops::Deref for crate::Unity::Mathematics::Geometry::Math {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+Geometry+Math")]
impl std::ops::DerefMut for crate::Unity::Mathematics::Geometry::Math {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+Geometry+Math")]
impl crate::Unity::Mathematics::Geometry::Math {
    pub fn Transform_RigidTransform0(
        transform: crate::Unity::Mathematics::RigidTransform,
        aabb: crate::Unity::Mathematics::Geometry::MinMaxAABB,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::Geometry::MinMaxAABB> {
        let __cordl_ret: crate::Unity::Mathematics::Geometry::MinMaxAABB = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Transform", (transform, aabb))?;
        Ok(__cordl_ret.into())
    }
    pub fn Transform_float3x3_2(
        transform: crate::Unity::Mathematics::float3x3,
        aabb: crate::Unity::Mathematics::Geometry::MinMaxAABB,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::Geometry::MinMaxAABB> {
        let __cordl_ret: crate::Unity::Mathematics::Geometry::MinMaxAABB = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Transform", (transform, aabb))?;
        Ok(__cordl_ret.into())
    }
    pub fn Transform_float4x4_1(
        transform: crate::Unity::Mathematics::float4x4,
        aabb: crate::Unity::Mathematics::Geometry::MinMaxAABB,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::Geometry::MinMaxAABB> {
        let __cordl_ret: crate::Unity::Mathematics::Geometry::MinMaxAABB = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Transform", (transform, aabb))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+Geometry+Math")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Mathematics::Geometry::Math {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
