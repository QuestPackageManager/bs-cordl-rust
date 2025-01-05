#[cfg(feature = "UnityEngine+ProBuilder+ProBuilderSnapping")]
#[repr(C)]
#[derive(Debug)]
pub struct ProBuilderSnapping {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ProBuilder+ProBuilderSnapping")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::ProBuilderSnapping =>
    "UnityEngine.ProBuilder"."ProBuilderSnapping"
);
#[cfg(feature = "UnityEngine+ProBuilder+ProBuilderSnapping")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::ProBuilderSnapping {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ProBuilderSnapping")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::ProBuilderSnapping {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ProBuilderSnapping")]
impl crate::UnityEngine::ProBuilder::ProBuilderSnapping {
    pub const k_MaxRaySnapDistance: f32 = std::f32::INFINITY;
    pub fn GetSnappingMaskBasedOnNormalVector(
        normal: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSnappingMaskBasedOnNormalVector", (normal))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsCardinalDirection(
        direction: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsCardinalDirection", (direction))?;
        Ok(__cordl_ret.into())
    }
    pub fn SnapValueOnRay(
        ray: crate::UnityEngine::Ray,
        distance: f32,
        snap: f32,
        mask: crate::UnityEngine::ProBuilder::Vector3Mask,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SnapValueOnRay", (ray, distance, snap, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn SnapVertices(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        indexes: quest_hook::libil2cpp::Gc<i32>,
        snap: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SnapVertices", (mesh, indexes, snap))?;
        Ok(__cordl_ret.into())
    }
    pub fn Snap_Vector3_Vector3_1(
        val: crate::UnityEngine::Vector3,
        snap: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Snap", (val, snap))?;
        Ok(__cordl_ret.into())
    }
    pub fn Snap_f32_f32_0(val: f32, snap: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Snap", (val, snap))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ProBuilderSnapping")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::ProBuilderSnapping {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
