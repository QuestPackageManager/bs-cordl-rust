#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+AppendElements")]
#[repr(C)]
#[derive(Debug)]
pub struct AppendElements {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+AppendElements")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::AppendElements =>
    "UnityEngine.ProBuilder.MeshOperations"."AppendElements"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+AppendElements")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::AppendElements {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+AppendElements")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::AppendElements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+AppendElements")]
impl crate::UnityEngine::ProBuilder::MeshOperations::AppendElements {
    #[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+AppendElements+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::MeshOperations::AppendElements___c;
    #[cfg(
        feature = "UnityEngine+ProBuilder+MeshOperations+AppendElements+__c__DisplayClass17_0"
    )]
    pub type __c__DisplayClass17_0 = crate::UnityEngine::ProBuilder::MeshOperations::AppendElements___c__DisplayClass17_0;
    pub fn AppendFace(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        positions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
        uv0s: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        uv2s: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
        uv3s: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        common: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Face,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AppendFace",
                (mesh, positions, colors, uv0s, uv2s, uv3s, face, common),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendFaces(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        positions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
            >,
        >,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
            >,
        >,
        uvs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
            >,
        >,
        faces: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::ProBuilder::Face>,
        >,
        shared: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppArray<i32>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::ProBuilder::Face>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::ProBuilder::Face>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendFaces", (mesh, positions, colors, uvs, faces, shared))?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendVerticesToEdge_Edge0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edge: crate::UnityEngine::ProBuilder::Edge,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendVerticesToEdge", (mesh, edge, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendVerticesToEdge_IList_1_1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edges: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendVerticesToEdge", (mesh, edges, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendVerticesToFace_ProBuilderMesh_Face_Il2CppArray0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        points: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Face,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendVerticesToFace", (mesh, face, points))?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendVerticesToFace__cordl_bool1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        points: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        insertOnEdge: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Face,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AppendVerticesToFace", (mesh, face, points, insertOnEdge))?;
        Ok(__cordl_ret.into())
    }
    pub fn Bridge(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        a: crate::UnityEngine::ProBuilder::Edge,
        b: crate::UnityEngine::ProBuilder::Edge,
        allowNonManifoldGeometry: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Face,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Bridge", (mesh, a, b, allowNonManifoldGeometry))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearAndRefreshMesh(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearAndRefreshMesh", (mesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePolygon(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
        unordered: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Face,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreatePolygon", (mesh, indexes, unordered))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePolygonWithHole(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
        holes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::System::Collections::Generic::IList_1<i32>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Face,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreatePolygonWithHole", (mesh, indexes, holes))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateShapeFromPolygon_PolyShape0(
        poly: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::PolyShape>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ActionResult>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ActionResult,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateShapeFromPolygon", (poly))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateShapeFromPolygon_ProBuilderMesh_IList_1_f32__cordl_bool1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        points: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector3>,
        >,
        extrude: f32,
        flipNormals: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ActionResult>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ActionResult,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateShapeFromPolygon", (mesh, points, extrude, flipNormals))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateShapeFromPolygon_ProBuilderMesh_IList_1_f32__cordl_bool_IList_1_3(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        points: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector3>,
        >,
        extrude: f32,
        flipNormals: bool,
        holePoints: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::System::Collections::Generic::IList_1<
                    crate::UnityEngine::Vector3,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ActionResult>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ActionResult,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateShapeFromPolygon",
                (mesh, points, extrude, flipNormals, holePoints),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateShapeFromPolygon_ProBuilderMesh_IList_1_f32__cordl_bool_Vector3_IList_1_2(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        points: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector3>,
        >,
        extrude: f32,
        flipNormals: bool,
        cameraLookAt: crate::UnityEngine::Vector3,
        holePoints: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::System::Collections::Generic::IList_1<
                    crate::UnityEngine::Vector3,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ActionResult>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ActionResult,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateShapeFromPolygon",
                (mesh, points, extrude, flipNormals, cameraLookAt, holePoints),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DuplicateAndFlip(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::ProBuilder::Face>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DuplicateAndFlip", (mesh, faces))?;
        Ok(__cordl_ret.into())
    }
    pub fn FaceWithVertices(
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::ProBuilder::Vertex,
            >,
        >,
        unordered: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::FaceRebuildData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::FaceRebuildData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FaceWithVertices", (vertices, unordered))?;
        Ok(__cordl_ret.into())
    }
    pub fn FaceWithVerticesAndHole(
        borderVertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::ProBuilder::Vertex,
            >,
        >,
        holes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::System::Collections::Generic::List_1<
                    *mut crate::UnityEngine::ProBuilder::Vertex,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::FaceRebuildData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::FaceRebuildData,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FaceWithVerticesAndHole", (borderVertices, holes))?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertVertexInFace(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::ProBuilder::Face>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::ProBuilder::Face>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InsertVertexInFace", (mesh, face, point))?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertVertexInMesh(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        point: crate::UnityEngine::Vector3,
        normal: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Vertex,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InsertVertexInMesh", (mesh, point, normal))?;
        Ok(__cordl_ret.into())
    }
    pub fn InsertVertexOnEdge(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        originalEdge: crate::UnityEngine::ProBuilder::Edge,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Vertex,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InsertVertexOnEdge", (mesh, originalEdge, point))?;
        Ok(__cordl_ret.into())
    }
    pub fn TentCapWithVertices(
        path: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::ProBuilder::Vertex,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::ProBuilder::FaceRebuildData,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::ProBuilder::FaceRebuildData,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TentCapWithVertices", (path))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+AppendElements")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::AppendElements {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
