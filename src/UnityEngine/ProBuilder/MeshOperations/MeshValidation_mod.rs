#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation")]
#[repr(C)]
#[derive(Debug)]
pub struct MeshValidation {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::MeshValidation =>
    "UnityEngine.ProBuilder.MeshOperations"."MeshValidation"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshOperations::MeshValidation {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation")]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::MeshOperations::MeshValidation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation")]
impl crate::UnityEngine::ProBuilder::MeshOperations::MeshValidation {
    #[cfg(
        feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation+AttributeValidationStrategy"
    )]
    pub type AttributeValidationStrategy = crate::UnityEngine::ProBuilder::MeshOperations::MeshValidation_AttributeValidationStrategy;
    pub fn CollectFaceGroups(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::UnityEngine::ProBuilder::Triangle,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::UnityEngine::ProBuilder::Triangle,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CollectFaceGroups", (mesh, face))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsDegenerateTriangles_Face2(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ContainsDegenerateTriangles", (mesh, face))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsDegenerateTriangles_IList_1_1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ContainsDegenerateTriangles", (mesh, faces))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsDegenerateTriangles_ProBuilderMesh0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ContainsDegenerateTriangles", (mesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsNonContiguousTriangles(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        face: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ContainsNonContiguousTriangles", (mesh, face))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureArraySize<T>(
        attribute: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        >,
        expectedVertexCount: i32,
        strategy: crate::UnityEngine::ProBuilder::MeshOperations::MeshValidation_AttributeValidationStrategy,
        fill: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EnsureArraySize",
                (attribute, expectedVertexCount, strategy, fill),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureFacesAreComposedOfContiguousTriangles(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Face>,
            >,
        >,
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
            .invoke("EnsureFacesAreComposedOfContiguousTriangles", (mesh, faces))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureListSize<T>(
        attribute: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        >,
        expectedVertexCount: i32,
        strategy: crate::UnityEngine::ProBuilder::MeshOperations::MeshValidation_AttributeValidationStrategy,
        fill: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureListSize", (attribute, expectedVertexCount, strategy, fill))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureMeshIsValid(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        removedVertices: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureMeshIsValid", (mesh, removedVertices))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureRealNumbers_IList_1_0(
        attribute: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureRealNumbers", (attribute))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureRealNumbers_IList_1_1(
        attribute: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureRealNumbers", (attribute))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureRealNumbers_IList_1_2(
        attribute: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureRealNumbers", (attribute))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureValidAttributes(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureValidAttributes", (mesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn RebuildEdges(
        edges: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
        removed: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ProBuilder::Edge,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RebuildEdges", (edges, removed))?;
        Ok(__cordl_ret.into())
    }
    pub fn RebuildIndexes(
        indices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
        removed: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<i32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RebuildIndexes", (indices, removed))?;
        Ok(__cordl_ret.into())
    }
    pub fn RebuildSelectionIndexes(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        faces: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    *mut crate::UnityEngine::ProBuilder::Face,
                >,
            >,
        >,
        edges: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::ProBuilder::Edge>,
            >,
        >,
        indices: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        >,
        removed: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RebuildSelectionIndexes", (mesh, faces, edges, indices, removed))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveDegenerateTriangles(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        removed: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveDegenerateTriangles", (mesh, removed))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveUnusedVertices(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        removed: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveUnusedVertices", (mesh, removed))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::MeshOperations::MeshValidation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation+AttributeValidationStrategy"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MeshValidation_AttributeValidationStrategy {
    #[default]
    Nullify = 1i32,
    Resize = 0i32,
}
#[cfg(
    feature = "UnityEngine+ProBuilder+MeshOperations+MeshValidation+AttributeValidationStrategy"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::MeshOperations::MeshValidation_AttributeValidationStrategy
    => "UnityEngine.ProBuilder.MeshOperations"
    ."MeshValidation/AttributeValidationStrategy"
);
