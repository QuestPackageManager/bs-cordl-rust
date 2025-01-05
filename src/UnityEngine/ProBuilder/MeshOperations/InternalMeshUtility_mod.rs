#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+InternalMeshUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct InternalMeshUtility {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+InternalMeshUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::InternalMeshUtility =>
    "UnityEngine.ProBuilder.MeshOperations"."InternalMeshUtility"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+InternalMeshUtility")]
impl std::ops::Deref
for crate::UnityEngine::ProBuilder::MeshOperations::InternalMeshUtility {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+InternalMeshUtility")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::InternalMeshUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+InternalMeshUtility")]
impl crate::UnityEngine::ProBuilder::MeshOperations::InternalMeshUtility {
    pub fn AverageNormalWithIndexes(
        shared: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SharedVertex>,
        all: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        norm: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AverageNormalWithIndexes", (shared, all, norm))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateMeshWithTransform(
        t: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        preserveFaces: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateMeshWithTransform", (t, preserveFaces))?;
        Ok(__cordl_ret.into())
    }
    pub fn FilterUnusedSubmeshIndexes(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FilterUnusedSubmeshIndexes", (mesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResetPbObjectWithMeshFilter(
        pb: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        preserveFaces: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResetPbObjectWithMeshFilter", (pb, preserveFaces))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+InternalMeshUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::InternalMeshUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
