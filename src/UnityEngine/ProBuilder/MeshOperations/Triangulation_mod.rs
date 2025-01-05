#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Triangulation")]
#[repr(C)]
#[derive(Debug)]
pub struct Triangulation {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Triangulation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::Triangulation =>
    "UnityEngine.ProBuilder.MeshOperations"."Triangulation"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Triangulation")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::Triangulation {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Triangulation")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::Triangulation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Triangulation")]
impl crate::UnityEngine::ProBuilder::MeshOperations::Triangulation {
    pub fn SortAndTriangulate(
        points: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector2>,
        indexes: quest_hook::libil2cpp::ByRefMut<quest_hook::libil2cpp::Gc<i32>>,
        convex: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SortAndTriangulate", (points, indexes, convex))?;
        Ok(__cordl_ret.into())
    }
    pub fn TriangulateVertices_Gc1(
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        triangles: quest_hook::libil2cpp::ByRefMut<quest_hook::libil2cpp::Gc<i32>>,
        holes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TriangulateVertices", (vertices, triangles, holes))?;
        Ok(__cordl_ret.into())
    }
    pub fn TriangulateVertices__cordl_bool__cordl_bool0(
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        >,
        triangles: quest_hook::libil2cpp::ByRefMut<quest_hook::libil2cpp::Gc<i32>>,
        unordered: bool,
        convex: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TriangulateVertices", (vertices, triangles, unordered, convex))?;
        Ok(__cordl_ret.into())
    }
    pub fn TriangulateVertices__cordl_bool__cordl_bool2(
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        triangles: quest_hook::libil2cpp::ByRefMut<quest_hook::libil2cpp::Gc<i32>>,
        unordered: bool,
        convex: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TriangulateVertices", (vertices, triangles, unordered, convex))?;
        Ok(__cordl_ret.into())
    }
    pub fn Triangulate_ByRefMut__cordl_bool0(
        points: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector2>,
        indexes: quest_hook::libil2cpp::ByRefMut<quest_hook::libil2cpp::Gc<i32>>,
        convex: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Triangulate", (points, indexes, convex))?;
        Ok(__cordl_ret.into())
    }
    pub fn Triangulate_Gc_ByRefMut1(
        points: quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector2>,
        holes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Vector2>,
        >,
        indexes: quest_hook::libil2cpp::ByRefMut<quest_hook::libil2cpp::Gc<i32>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Triangulate", (points, holes, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_triangulationContext() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationContext,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Poly2Tri::TriangulationContext,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_triangulationContext", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Triangulation")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::Triangulation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
