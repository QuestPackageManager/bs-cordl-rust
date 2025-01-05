#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+CombineMeshes")]
#[repr(C)]
#[derive(Debug)]
pub struct CombineMeshes {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+CombineMeshes")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::CombineMeshes =>
    "UnityEngine.ProBuilder.MeshOperations"."CombineMeshes"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+CombineMeshes")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::CombineMeshes {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+CombineMeshes")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::CombineMeshes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+CombineMeshes")]
impl crate::UnityEngine::ProBuilder::MeshOperations::CombineMeshes {
    pub fn AccumulateMeshesInfo(
        meshes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        >,
        offset: i32,
        vertices: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
            >,
        >,
        faces: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
        autoUvFaces: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
        sharedVertices: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SharedVertex>,
            >,
        >,
        sharedTextures: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SharedVertex>,
            >,
        >,
        materialMap: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
            >,
        >,
        targetTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AccumulateMeshesInfo",
                (
                    meshes,
                    offset,
                    vertices,
                    faces,
                    autoUvFaces,
                    sharedVertices,
                    sharedTextures,
                    materialMap,
                    targetTransform,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineToNewMeshes(
        meshes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineToNewMeshes", (meshes))?;
        Ok(__cordl_ret.into())
    }
    pub fn Combine_Gc0(
        meshes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Combine", (meshes))?;
        Ok(__cordl_ret.into())
    }
    pub fn Combine_Gc1(
        meshes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        >,
        meshTarget: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Combine", (meshes, meshTarget))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateMeshFromSplit(
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        >,
        faces: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        >,
        sharedVertexLookup: quest_hook::libil2cpp::Gc<i32, i32>,
        sharedTextureLookup: quest_hook::libil2cpp::Gc<i32, i32>,
        remap: quest_hook::libil2cpp::Gc<i32, i32>,
        materials: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateMeshFromSplit",
                (
                    vertices,
                    faces,
                    sharedVertexLookup,
                    sharedTextureLookup,
                    remap,
                    materials,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SplitByMaxVertexCount(
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        >,
        faces: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        >,
        sharedVertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SharedVertex>,
        >,
        sharedTextures: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::SharedVertex>,
        >,
        maxVertexCount: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SplitByMaxVertexCount",
                (vertices, faces, sharedVertices, sharedTextures, maxVertexCount),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+CombineMeshes")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::CombineMeshes {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
