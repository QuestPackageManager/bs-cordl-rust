#[cfg(feature = "UnityEngine+ProBuilder+HandleUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct HandleUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+HandleUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::HandleUtility =>
    "UnityEngine.ProBuilder"."HandleUtility"
);
#[cfg(feature = "UnityEngine+ProBuilder+HandleUtility")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::HandleUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+HandleUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::HandleUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+HandleUtility")]
impl crate::UnityEngine::ProBuilder::HandleUtility {
    pub fn FaceRaycastBothCullModes(
        worldRay: crate::UnityEngine::Ray,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        back: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ProBuilder::SimpleTuple_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                crate::UnityEngine::Vector3,
            >,
        >,
        front: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ProBuilder::SimpleTuple_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                crate::UnityEngine::Vector3,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FaceRaycastBothCullModes", (worldRay, mesh, back, front))?;
        Ok(__cordl_ret.into())
    }
    pub fn FaceRaycast_CullingMode_HashSet_1_2(
        InWorldRay: crate::UnityEngine::Ray,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        hits: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::RaycastHit>,
                >,
            >,
        >,
        cullingMode: crate::UnityEngine::ProBuilder::CullingMode,
        ignore: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FaceRaycast", (InWorldRay, mesh, hits, cullingMode, ignore))?;
        Ok(__cordl_ret.into())
    }
    pub fn FaceRaycast_HashSet_1_0(
        worldRay: crate::UnityEngine::Ray,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        hit: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::RaycastHit>,
        >,
        ignore: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FaceRaycast", (worldRay, mesh, hit, ignore))?;
        Ok(__cordl_ret.into())
    }
    pub fn FaceRaycast_f32_CullingMode_HashSet_1_1(
        worldRay: crate::UnityEngine::Ray,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        hit: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::RaycastHit>,
        >,
        distance: f32,
        cullingMode: crate::UnityEngine::ProBuilder::CullingMode,
        ignore: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "FaceRaycast",
                (worldRay, mesh, hit, distance, cullingMode, ignore),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetActiveElementPosition_ProBuilderMesh_IEnumerable_1_0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetActiveElementPosition", (mesh, faces))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetActiveElementPosition_ProBuilderMesh_IEnumerable_1_1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edges: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetActiveElementPosition", (mesh, edges))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetActiveElementPosition_ProBuilderMesh_IEnumerable_1_2(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetActiveElementPosition", (mesh, vertices))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEdgeRotation_Edge1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edge: crate::UnityEngine::ProBuilder::Edge,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEdgeRotation", (mesh, edge))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEdgeRotation_HandleOrientation_IEnumerable_1_0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        orientation: crate::UnityEngine::ProBuilder::HandleOrientation,
        edges: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEdgeRotation", (mesh, orientation, edges))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFaceRotation_Face1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFaceRotation", (mesh, face))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFaceRotation_HandleOrientation_IEnumerable_1_0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        orientation: crate::UnityEngine::ProBuilder::HandleOrientation,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFaceRotation", (mesh, orientation, faces))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRotation(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        indices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRotation", (mesh, indices))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexRotation_HandleOrientation_IEnumerable_1_0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        orientation: crate::UnityEngine::ProBuilder::HandleOrientation,
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVertexRotation", (mesh, orientation, vertices))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexRotation_i32_1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        vertex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVertexRotation", (mesh, vertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn InverseTransformRay(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        InWorldRay: crate::UnityEngine::Ray,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Ray> {
        let __cordl_ret: crate::UnityEngine::Ray = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InverseTransformRay", (transform, InWorldRay))?;
        Ok(__cordl_ret.into())
    }
    pub fn MeshRaycast_GameObject_ByRefMut_f32_0(
        InWorldRay: crate::UnityEngine::Ray,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        hit: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::RaycastHit>,
        >,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MeshRaycast", (InWorldRay, gameObject, hit, distance))?;
        Ok(__cordl_ret.into())
    }
    pub fn MeshRaycast_Il2CppArray_Il2CppArray_ByRefMut_f32_1(
        InRay: crate::UnityEngine::Ray,
        mesh: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        triangles: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        hit: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::RaycastHit>,
        >,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MeshRaycast", (InRay, mesh, triangles, hit, distance))?;
        Ok(__cordl_ret.into())
    }
    pub fn PointIsOccluded(
        cam: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        pb: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        worldPoint: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PointIsOccluded", (cam, pb, worldPoint))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScreenToGuiPoint(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        point: crate::UnityEngine::Vector3,
        pixelsPerPoint: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScreenToGuiPoint", (camera, point, pixelsPerPoint))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+HandleUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::HandleUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
