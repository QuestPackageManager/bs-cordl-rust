#[cfg(feature = "UnityEngine+ProBuilder+Smoothing")]
#[repr(C)]
#[derive(Debug)]
pub struct Smoothing {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ProBuilder+Smoothing")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Smoothing =>
    "UnityEngine.ProBuilder"."Smoothing"
);
#[cfg(feature = "UnityEngine+ProBuilder+Smoothing")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Smoothing {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Smoothing")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Smoothing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Smoothing")]
impl crate::UnityEngine::ProBuilder::Smoothing {
    pub const hardRangeMax: i32 = 42i32;
    pub const hardRangeMin: i32 = 25i32;
    pub const smoothRangeMax: i32 = 24i32;
    pub const smoothRangeMin: i32 = 1i32;
    pub const smoothingGroupNone: i32 = 0i32;
    pub fn ApplySmoothingGroups_Gc1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        >,
        angleThreshold: f32,
        normals: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplySmoothingGroups", (mesh, faces, angleThreshold, normals))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplySmoothingGroups_Gc_Gc_f32_0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        >,
        angleThreshold: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplySmoothingGroups", (mesh, faces, angleThreshold))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindSoftEdgesRecursive(
        normals: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        wing: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        angleThreshold: f32,
        processed: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FindSoftEdgesRecursive",
                (normals, wing, angleThreshold, processed),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextUnusedSmoothingGroup(
        start: i32,
        used: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNextUnusedSmoothingGroup", (start, used))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUnusedSmoothingGroup(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUnusedSmoothingGroup", (mesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSmooth(index: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSmooth", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSoftEdge(
        normals: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        left: crate::UnityEngine::ProBuilder::EdgeLookup,
        right: crate::UnityEngine::ProBuilder::EdgeLookup,
        threshold: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSoftEdge", (normals, left, right, threshold))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Smoothing")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::Smoothing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
