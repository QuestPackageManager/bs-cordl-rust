#[cfg(feature = "UnityEngine+ProBuilder+SelectPathFaces")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectPathFaces {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectPathFaces")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::SelectPathFaces =>
    "UnityEngine.ProBuilder"."SelectPathFaces"
);
#[cfg(feature = "UnityEngine+ProBuilder+SelectPathFaces")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::SelectPathFaces {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectPathFaces")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::SelectPathFaces {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectPathFaces")]
impl crate::UnityEngine::ProBuilder::SelectPathFaces {
    pub fn Dijkstra(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        start: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Dijkstra", (mesh, start))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMinimalPath(
        predecessors: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        start: i32,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMinimalPath", (predecessors, start, end))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPath(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        start: i32,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPath", (mesh, start, end))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWeight(
        face1: i32,
        face2: i32,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetWeight", (face1, face2, mesh))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectPathFaces")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::SelectPathFaces {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
