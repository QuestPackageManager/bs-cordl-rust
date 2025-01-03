#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ExtrudeElements")]
#[repr(C)]
#[derive(Debug)]
pub struct ExtrudeElements {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ExtrudeElements")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::ExtrudeElements =>
    "UnityEngine.ProBuilder.MeshOperations"."ExtrudeElements"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ExtrudeElements")]
impl std::ops::Deref
for crate::UnityEngine::ProBuilder::MeshOperations::ExtrudeElements {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ExtrudeElements")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::ExtrudeElements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ExtrudeElements")]
impl crate::UnityEngine::ProBuilder::MeshOperations::ExtrudeElements {
    pub fn DetachFaces_ProBuilderMesh_IEnumerable_1_0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::ProBuilder::Face,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::ProBuilder::Face,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::ProBuilder::Face,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DetachFaces", (mesh, faces))?;
        Ok(__cordl_ret.into())
    }
    pub fn DetachFaces__cordl_bool1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::ProBuilder::Face,
            >,
        >,
        deleteSourceFaces: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::ProBuilder::Face,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::ProBuilder::Face,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DetachFaces", (mesh, faces, deleteSourceFaces))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtrudeAsGroups(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::ProBuilder::Face,
            >,
        >,
        compensateAngleVertexDistance: bool,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::ProBuilder::Face>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::ProBuilder::Face>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ExtrudeAsGroups",
                (mesh, faces, compensateAngleVertexDistance, distance),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtrudePerFace(
        pb: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::ProBuilder::Face,
            >,
        >,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::ProBuilder::Face>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::ProBuilder::Face>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtrudePerFace", (pb, faces, distance))?;
        Ok(__cordl_ret.into())
    }
    pub fn Extrude_ExtrudeMethod_f32_0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::UnityEngine::ProBuilder::Face,
            >,
        >,
        method: crate::UnityEngine::ProBuilder::ExtrudeMethod,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::ProBuilder::Face>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::ProBuilder::Face>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Extrude", (mesh, faces, method, distance))?;
        Ok(__cordl_ret.into())
    }
    pub fn Extrude_f32__cordl_bool__cordl_bool1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edges: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
        distance: f32,
        extrudeAsGroup: bool,
        enableManifoldExtrude: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ProBuilder::Edge>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ProBuilder::Edge>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Extrude",
                (mesh, edges, distance, extrudeAsGroup, enableManifoldExtrude),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFaceGroups(
        wings: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::UnityEngine::ProBuilder::WingedEdge,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::System::Collections::Generic::HashSet_1<
                    *mut crate::UnityEngine::ProBuilder::Face,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::System::Collections::Generic::HashSet_1<
                    *mut crate::UnityEngine::ProBuilder::Face,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFaceGroups", (wings))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPerimeterEdges(
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                *mut crate::UnityEngine::ProBuilder::Face,
            >,
        >,
        lookup: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<i32, i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::ProBuilder::EdgeLookup,
                *mut crate::UnityEngine::ProBuilder::Face,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::ProBuilder::EdgeLookup,
                *mut crate::UnityEngine::ProBuilder::Face,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPerimeterEdges", (faces, lookup))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+ExtrudeElements")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::ExtrudeElements {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
