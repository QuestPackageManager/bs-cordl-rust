#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+QuadUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct QuadUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+QuadUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::QuadUtility =>
    "UnityEngine.ProBuilder.MeshOperations"."QuadUtility"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+QuadUtility")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::QuadUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+QuadUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::MeshOperations::QuadUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+QuadUtility")]
impl crate::UnityEngine::ProBuilder::MeshOperations::QuadUtility {
    pub fn GetBestQuadConnection(
        wing: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        connections: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                crate::UnityEngine::ProBuilder::EdgeLookup,
                f32,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Face,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBestQuadConnection", (wing, connections))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetQuadScore(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        left: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        right: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::WingedEdge>,
        normalThreshold: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetQuadScore", (mesh, left, right, normalThreshold))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToQuads(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
        smoothing: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ToQuads", (mesh, faces, smoothing))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+QuadUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::QuadUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
