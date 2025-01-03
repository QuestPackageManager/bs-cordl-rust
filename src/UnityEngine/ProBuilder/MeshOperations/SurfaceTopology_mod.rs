#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+SurfaceTopology")]
#[repr(C)]
#[derive(Debug)]
pub struct SurfaceTopology {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+SurfaceTopology")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::SurfaceTopology =>
    "UnityEngine.ProBuilder.MeshOperations"."SurfaceTopology"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+SurfaceTopology")]
impl std::ops::Deref
for crate::UnityEngine::ProBuilder::MeshOperations::SurfaceTopology {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+SurfaceTopology")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::SurfaceTopology {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+SurfaceTopology")]
impl crate::UnityEngine::ProBuilder::MeshOperations::SurfaceTopology {
    #[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+SurfaceTopology+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::MeshOperations::SurfaceTopology___c;
    pub fn BreakFaceIntoTris(
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::ProBuilder::Vertex,
            >,
        >,
        lookup: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<i32, i32>,
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
            .invoke("BreakFaceIntoTris", (face, vertices, lookup))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConformNormals(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::ProBuilder::Face,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ActionResult>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ActionResult,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConformNormals", (mesh, faces))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConformOppositeNormal(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ActionResult>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ActionResult,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConformOppositeNormal", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn FlipEdge(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FlipEdge", (mesh, face))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCommonEdgeInWindingOrder(
        wing: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::Edge> {
        let __cordl_ret: crate::UnityEngine::ProBuilder::Edge = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCommonEdgeInWindingOrder", (wing))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWindingFlags(
        edge: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        flag: bool,
        flags: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                *mut crate::UnityEngine::ProBuilder::Face,
                bool,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWindingFlags", (edge, flag, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWindingOrder_IList_1_2(
        points: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::WindingOrder> {
        let __cordl_ret: crate::UnityEngine::ProBuilder::WindingOrder = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWindingOrder", (points))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWindingOrder_IList_1_IList_1_1(
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ProBuilder::Vertex,
            >,
        >,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::WindingOrder> {
        let __cordl_ret: crate::UnityEngine::ProBuilder::WindingOrder = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWindingOrder", (vertices, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWindingOrder_ProBuilderMesh_Face0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::WindingOrder> {
        let __cordl_ret: crate::UnityEngine::ProBuilder::WindingOrder = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWindingOrder", (mesh, face))?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchNormal(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        target: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        lookup: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<i32, i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MatchNormal", (source, target, lookup))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToTriangles(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ProBuilder::Face,
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
            .invoke("ToTriangles", (mesh, faces))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+SurfaceTopology")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::SurfaceTopology {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
