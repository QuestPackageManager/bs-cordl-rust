#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+VertexEditing")]
#[repr(C)]
#[derive(Debug)]
pub struct VertexEditing {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+VertexEditing")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::VertexEditing =>
    "UnityEngine.ProBuilder.MeshOperations"."VertexEditing"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+VertexEditing")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::VertexEditing {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+VertexEditing")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::VertexEditing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+VertexEditing")]
impl crate::UnityEngine::ProBuilder::MeshOperations::VertexEditing {
    pub fn AlignEdgeWithDirection(
        edge: crate::UnityEngine::ProBuilder::EdgeLookup,
        commonIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::Edge> {
        let __cordl_ret: crate::UnityEngine::ProBuilder::Edge = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AlignEdgeWithDirection", (edge, commonIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExplodeVertex(
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ProBuilder::Vertex,
            >,
        >,
        edgeAndCommonIndex: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ProBuilder::SimpleTuple_2<
                    *mut crate::UnityEngine::ProBuilder::WingedEdge,
                    i32,
                >,
            >,
        >,
        distance: f32,
        appendedVertices: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Collections::Generic::Dictionary_2<
                i32,
                *mut crate::System::Collections::Generic::List_1<i32>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::FaceRebuildData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::FaceRebuildData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ExplodeVertex",
                (vertices, edgeAndCommonIndex, distance, appendedVertices),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MergeVertices(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        indexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        collapseToFirst: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MergeVertices", (mesh, indexes, collapseToFirst))?;
        Ok(__cordl_ret.into())
    }
    pub fn SplitVertices_Edge0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edge: crate::UnityEngine::ProBuilder::Edge,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SplitVertices", (mesh, edge))?;
        Ok(__cordl_ret.into())
    }
    pub fn SplitVertices_IEnumerable_1_1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SplitVertices", (mesh, vertices))?;
        Ok(__cordl_ret.into())
    }
    pub fn WeldVertices(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
        neighborRadius: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WeldVertices", (mesh, indexes, neighborRadius))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+VertexEditing")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::VertexEditing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
