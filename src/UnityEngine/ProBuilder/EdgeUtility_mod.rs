#[cfg(feature = "UnityEngine+ProBuilder+EdgeUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct EdgeUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+EdgeUtility")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::ProBuilder::EdgeUtility {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder";
    const CLASS_NAME: &'static str = "EdgeUtility";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+EdgeUtility")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::EdgeUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+EdgeUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::EdgeUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+EdgeUtility")]
impl crate::UnityEngine::ProBuilder::EdgeUtility {
    pub fn AllTriangles(
        edges: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ProBuilder::Edge>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllTriangles", (edges))?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains_Edge0(
        edges: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ProBuilder::Edge>,
        >,
        edge: crate::UnityEngine::ProBuilder::Edge,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Contains", (edges, edge))?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains_i32_i32_1(
        edges: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ProBuilder::Edge>,
        >,
        x: i32,
        y: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Contains", (edges, x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEdgeWithSharedVertexHandles(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edge: crate::UnityEngine::ProBuilder::Edge,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::Edge> {
        let __cordl_ret: crate::UnityEngine::ProBuilder::Edge = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEdgeWithSharedVertexHandles", (mesh, edge))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFace(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edge: crate::UnityEngine::ProBuilder::Edge,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Face,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFace", (mesh, edge))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSharedVertexHandleEdge(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edge: crate::UnityEngine::ProBuilder::Edge,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::Edge> {
        let __cordl_ret: crate::UnityEngine::ProBuilder::Edge = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSharedVertexHandleEdge", (mesh, edge))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSharedVertexHandleEdges(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edges: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSharedVertexHandleEdges", (mesh, edges))?;
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edges: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
        edge: crate::UnityEngine::ProBuilder::Edge,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IndexOf", (mesh, edges, edge))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateEdge(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edge: crate::UnityEngine::ProBuilder::Edge,
        validEdge: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ProBuilder::SimpleTuple_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidateEdge", (mesh, edge, validEdge))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+EdgeUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::EdgeUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
