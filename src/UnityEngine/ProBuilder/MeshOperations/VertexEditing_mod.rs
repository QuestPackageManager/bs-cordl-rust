#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+VertexEditing")]
#[repr(C)]
#[derive(Debug)]
pub struct VertexEditing {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+VertexEditing")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::MeshOperations::VertexEditing {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder.MeshOperations";
    const CLASS_NAME: &'static str = "VertexEditing";
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
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+VertexEditing")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::VertexEditing {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+VertexEditing")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::VertexEditing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+VertexEditing")]
impl crate::UnityEngine::ProBuilder::MeshOperations::VertexEditing {
    pub fn AlignEdgeWithDirection(
        edge: crate::UnityEngine::ProBuilder::EdgeLookup,
        commonIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::Edge> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::ProBuilder::EdgeLookup, i32),
                crate::UnityEngine::ProBuilder::Edge,
                2usize,
            >("AlignEdgeWithDirection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AlignEdgeWithDirection", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::ProBuilder::Edge = unsafe {
            method.invoke_unchecked((), (edge, commonIndex))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExplodeVertex(
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
            >,
        >,
        edgeAndCommonIndex: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                crate::UnityEngine::ProBuilder::SimpleTuple_2<
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::WingedEdge,
                    >,
                    i32,
                >,
            >,
        >,
        distance: f32,
        appendedVertices: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::Dictionary_2<
                    i32,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<i32>,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::FaceRebuildData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::ProBuilder::Vertex,
                            >,
                        >,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IList_1<
                            crate::UnityEngine::ProBuilder::SimpleTuple_2<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::ProBuilder::WingedEdge,
                                >,
                                i32,
                            >,
                        >,
                    >,
                    f32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::Dictionary_2<
                                i32,
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Collections::Generic::List_1<i32>,
                                >,
                            >,
                        >,
                    >,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::FaceRebuildData,
                >,
                4usize,
            >("ExplodeVertex")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ExplodeVertex", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::FaceRebuildData,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (vertices, edgeAndCommonIndex, distance, appendedVertices),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn MergeVertices(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        indexes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        collapseToFirst: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                    bool,
                ),
                i32,
                3usize,
            >("MergeVertices")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MergeVertices", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (mesh, indexes, collapseToFirst))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SplitVertices_Edge0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        edge: crate::UnityEngine::ProBuilder::Edge,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    crate::UnityEngine::ProBuilder::Edge,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SplitVertices")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SplitVertices", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mesh, edge))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SplitVertices_IEnumerable_1_1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<i32>,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SplitVertices")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SplitVertices", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mesh, vertices))
        };
        Ok(__cordl_ret.into())
    }
    pub fn WeldVertices(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
        neighborRadius: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::ProBuilder::ProBuilderMesh,
                    >,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<i32>,
                    >,
                    f32,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
                3usize,
            >("WeldVertices")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "WeldVertices", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = unsafe { method.invoke_unchecked((), (mesh, indexes, neighborRadius)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+VertexEditing")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::VertexEditing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
