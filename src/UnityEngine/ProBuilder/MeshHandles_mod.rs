#[cfg(feature = "UnityEngine+ProBuilder+MeshHandles")]
#[repr(C)]
#[derive(Debug)]
pub struct MeshHandles {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshHandles")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::MeshHandles =>
    "UnityEngine.ProBuilder"."MeshHandles"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshHandles")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshHandles {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshHandles")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::MeshHandles {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshHandles")]
impl crate::UnityEngine::ProBuilder::MeshHandles {
    pub fn CreateEdgeBillboardMesh_Gc1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        edges: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Edge>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateEdgeBillboardMesh", (mesh, target, edges))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateEdgeBillboardMesh_Gc_Gc0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateEdgeBillboardMesh", (mesh, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateEdgeMesh_Gc1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        edges: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ProBuilder::Edge>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateEdgeMesh", (mesh, target, edges))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateEdgeMesh_Gc_Gc0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateEdgeMesh", (mesh, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateFaceMesh(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateFaceMesh", (mesh, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateFaceMeshFromFaces(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        >,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateFaceMeshFromFaces", (mesh, faces, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePointBillboardMesh_Gc1(
        positions: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>,
        indexes: quest_hook::libil2cpp::Gc<i32>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreatePointBillboardMesh", (positions, indexes, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePointBillboardMesh_Gc_Gc0(
        positions: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector3>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreatePointBillboardMesh", (positions, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePointMesh(
        positions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        indexes: quest_hook::libil2cpp::Gc<i32>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreatePointMesh", (positions, indexes, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateVertexMesh_Gc1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        indexes: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateVertexMesh", (mesh, target, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateVertexMesh_Gc_Gc0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateVertexMesh", (mesh, target))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshHandles")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::MeshHandles {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
