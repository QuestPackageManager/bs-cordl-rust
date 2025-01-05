#[cfg(feature = "UnityEngine+ProBuilder+Projection")]
#[repr(C)]
#[derive(Debug)]
pub struct Projection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+Projection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Projection =>
    "UnityEngine.ProBuilder"."Projection"
);
#[cfg(feature = "UnityEngine+ProBuilder+Projection")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Projection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Projection")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Projection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Projection")]
impl crate::UnityEngine::ProBuilder::Projection {
    pub fn FindBestPlane_IList_1_IList_1_0(
        points: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector3>,
        >,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Plane> {
        let __cordl_ret: crate::UnityEngine::Plane = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindBestPlane", (points, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindBestPlane_ProBuilderMesh_i32_1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        textureGroup: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Plane> {
        let __cordl_ret: crate::UnityEngine::Plane = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindBestPlane", (mesh, textureGroup))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTangentToAxis(
        axis: crate::UnityEngine::ProBuilder::ProjectionAxis,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTangentToAxis", (axis))?;
        Ok(__cordl_ret.into())
    }
    pub fn PlanarProject_IList_1_IList_1_0(
        positions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector3>,
        >,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PlanarProject", (positions, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn PlanarProject_IList_1_IList_1_Vector3_1(
        positions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector3>,
        >,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
        direction: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PlanarProject", (positions, indexes, direction))?;
        Ok(__cordl_ret.into())
    }
    pub fn PlanarProject_IList_1_IList_1_Vector3_List_1_2(
        positions: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector3>,
        >,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
        direction: crate::UnityEngine::Vector3,
        results: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PlanarProject", (positions, indexes, direction, results))?;
        Ok(__cordl_ret.into())
    }
    pub fn PlanarProject_ProBuilderMesh_Face_Vector3_4(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        projection: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PlanarProject", (mesh, face, projection))?;
        Ok(__cordl_ret.into())
    }
    pub fn PlanarProject_ProBuilderMesh_i32_AutoUnwrapSettings3(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        textureGroup: i32,
        unwrapSettings: crate::UnityEngine::ProBuilder::AutoUnwrapSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PlanarProject", (mesh, textureGroup, unwrapSettings))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProjectionAxisToVector(
        axis: crate::UnityEngine::ProBuilder::ProjectionAxis,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProjectionAxisToVector", (axis))?;
        Ok(__cordl_ret.into())
    }
    pub fn Sort(
        verts: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector2>,
        >,
        method: crate::UnityEngine::ProBuilder::SortMethod,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector2>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector2>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Sort", (verts, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn SphericalProject(
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector3>,
        >,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SphericalProject", (vertices, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn VectorToProjectionAxis(
        direction: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::ProjectionAxis> {
        let __cordl_ret: crate::UnityEngine::ProBuilder::ProjectionAxis = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VectorToProjectionAxis", (direction))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Projection")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::Projection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
