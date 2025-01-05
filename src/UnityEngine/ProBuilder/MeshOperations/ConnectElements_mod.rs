#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ConnectElements")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectElements {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ConnectElements")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::ConnectElements =>
    "UnityEngine.ProBuilder.MeshOperations"."ConnectElements"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ConnectElements")]
impl std::ops::Deref
for crate::UnityEngine::ProBuilder::MeshOperations::ConnectElements {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ConnectElements")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::ConnectElements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ConnectElements")]
impl crate::UnityEngine::ProBuilder::MeshOperations::ConnectElements {
    pub fn ConnectEdgesInFace_Gc0(
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        b: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ProBuilder::MeshOperations::ConnectFaceRebuildData,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ProBuilder::MeshOperations::ConnectFaceRebuildData,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConnectEdgesInFace", (face, a, b, vertices))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConnectEdgesInFace_Gc_Gc_Gc1(
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        edges: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        >,
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ProBuilder::MeshOperations::ConnectFaceRebuildData,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ProBuilder::MeshOperations::ConnectFaceRebuildData,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConnectEdgesInFace", (face, edges, vertices))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConnectIndexesPerFace_Gc_Gc_i32_1(
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        indexes: quest_hook::libil2cpp::Gc<i32>,
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        >,
        lookup: quest_hook::libil2cpp::Gc<i32, i32>,
        sharedIndexOffset: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ProBuilder::MeshOperations::ConnectFaceRebuildData,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ProBuilder::MeshOperations::ConnectFaceRebuildData,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ConnectIndexesPerFace",
                (face, indexes, vertices, lookup, sharedIndexOffset),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ConnectIndexesPerFace_i32_i32_Gc0(
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        a: i32,
        b: i32,
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        >,
        lookup: quest_hook::libil2cpp::Gc<i32, i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ProBuilder::MeshOperations::ConnectFaceRebuildData,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ProBuilder::MeshOperations::ConnectFaceRebuildData,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConnectIndexesPerFace", (face, a, b, vertices, lookup))?;
        Ok(__cordl_ret.into())
    }
    pub fn Connect_ByRefMut_ByRefMut__cordl_bool__cordl_bool_Gc3(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edges: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Edge>,
        addedFaces: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                >,
            >,
        >,
        connections: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ProBuilder::Edge>,
            >,
        >,
        returnFaces: bool,
        returnEdges: bool,
        faceMask: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ActionResult>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ActionResult,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Connect",
                (
                    mesh,
                    edges,
                    addedFaces,
                    connections,
                    returnFaces,
                    returnEdges,
                    faceMask,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Connect_Gc_Gc0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Connect", (mesh, faces))?;
        Ok(__cordl_ret.into())
    }
    pub fn Connect_Gc_Gc1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edges: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Edge>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ProBuilder::SimpleTuple_2<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                >,
            >,
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ProBuilder::Edge>,
            >,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::ProBuilder::SimpleTuple_2<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                >,
            >,
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ProBuilder::Edge>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Connect", (mesh, edges))?;
        Ok(__cordl_ret.into())
    }
    pub fn Connect_Gc_Gc2(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        indexes: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Connect", (mesh, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertVertices(
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        edges: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        >,
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        >,
        data: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::ProBuilder::MeshOperations::ConnectFaceRebuildData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InsertVertices", (face, edges, vertices, data))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ConnectElements")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::ConnectElements {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
