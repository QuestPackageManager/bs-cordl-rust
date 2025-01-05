#[cfg(feature = "UnityEngine+ProBuilder+VertexPositioning")]
#[repr(C)]
#[derive(Debug)]
pub struct VertexPositioning {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+ProBuilder+VertexPositioning")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::VertexPositioning =>
    "UnityEngine.ProBuilder"."VertexPositioning"
);
#[cfg(feature = "UnityEngine+ProBuilder+VertexPositioning")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::VertexPositioning {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+VertexPositioning")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::VertexPositioning {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+VertexPositioning")]
impl crate::UnityEngine::ProBuilder::VertexPositioning {
    pub fn SetSharedVertexPosition(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        sharedVertexHandle: i32,
        position: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSharedVertexPosition", (mesh, sharedVertexHandle, position))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSharedVertexValues(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        sharedVertexHandle: i32,
        vertex: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSharedVertexValues", (mesh, sharedVertexHandle, vertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn TranslateVerticesInWorldSpace_Gc_Gc_Vector3_0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        indexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        offset: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TranslateVerticesInWorldSpace", (mesh, indexes, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn TranslateVerticesInWorldSpace_f32__cordl_bool1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        indexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        offset: crate::UnityEngine::Vector3,
        snapValue: f32,
        snapAxisOnly: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TranslateVerticesInWorldSpace",
                (mesh, indexes, offset, snapValue, snapAxisOnly),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TranslateVerticesInternal(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        indices: quest_hook::libil2cpp::Gc<i32>,
        offset: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TranslateVerticesInternal", (mesh, indices, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn TranslateVertices_Gc_Gc_Vector3_0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        indexes: quest_hook::libil2cpp::Gc<i32>,
        offset: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TranslateVertices", (mesh, indexes, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn TranslateVertices_Gc_Gc_Vector3_1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edges: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Edge>,
        offset: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TranslateVertices", (mesh, edges, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn TranslateVertices_Gc_Gc_Vector3_2(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
        >,
        offset: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TranslateVertices", (mesh, faces, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerticesInWorldSpace(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VerticesInWorldSpace", (mesh))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+VertexPositioning")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::VertexPositioning {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
