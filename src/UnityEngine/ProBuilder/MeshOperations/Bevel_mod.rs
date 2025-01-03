#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Bevel")]
#[repr(C)]
#[derive(Debug)]
pub struct Bevel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Bevel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::MeshOperations::Bevel
    => "UnityEngine.ProBuilder.MeshOperations"."Bevel"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Bevel")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::Bevel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Bevel")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::MeshOperations::Bevel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Bevel")]
impl crate::UnityEngine::ProBuilder::MeshOperations::Bevel {
    pub fn BevelEdges(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edges: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
        amount: f32,
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
            .invoke("BevelEdges", (mesh, edges, amount))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBridgeFaces(
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ProBuilder::Vertex,
            >,
        >,
        left: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        right: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        holes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                i32,
                *mut crate::System::Collections::Generic::List_1<
                    crate::UnityEngine::ProBuilder::SimpleTuple_2<
                        *mut crate::UnityEngine::ProBuilder::FaceRebuildData,
                        *mut crate::System::Collections::Generic::List_1<i32>,
                    >,
                >,
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
            .invoke("GetBridgeFaces", (vertices, left, right, holes))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLeadingEdge(
        wing: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        common: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::Edge> {
        let __cordl_ret: crate::UnityEngine::ProBuilder::Edge = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLeadingEdge", (wing, common))?;
        Ok(__cordl_ret.into())
    }
    pub fn SlideEdge(
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ProBuilder::Vertex,
            >,
        >,
        we: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        amount: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SlideEdge", (vertices, we, amount))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+Bevel")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::Bevel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
