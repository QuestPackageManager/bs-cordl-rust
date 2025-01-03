#[cfg(feature = "UnityEngine+ProBuilder+MeshUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct MeshUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::MeshUtility =>
    "UnityEngine.ProBuilder"."MeshUtility"
);
#[cfg(feature = "UnityEngine+ProBuilder+MeshUtility")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::MeshUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshUtility")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::MeshUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshUtility")]
impl crate::UnityEngine::ProBuilder::MeshUtility {
    pub fn CollapseSharedVertices(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::ProBuilder::Vertex,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CollapseSharedVertices", (mesh, vertices))?;
        Ok(__cordl_ret.into())
    }
    pub fn Compile(
        probuilderMesh: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::ProBuilderMesh,
        >,
        targetMesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        preferredTopology: crate::UnityEngine::MeshTopology,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Compile", (probuilderMesh, targetMesh, preferredTopology))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyTo(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyTo", (source, destination))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeepCopy(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeepCopy", (source))?;
        Ok(__cordl_ret.into())
    }
    pub fn FitToSize(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
        currentSize: crate::UnityEngine::Bounds,
        sizeToFit: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FitToSize", (mesh, currentSize, sizeToFit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GeneratePerTriangleMesh(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::ProBuilder::Vertex,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::ProBuilder::Vertex,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GeneratePerTriangleMesh", (mesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateTangent(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenerateTangent", (mesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBounds(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_ret: crate::UnityEngine::Bounds = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBounds", (mesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndexCount(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIndexCount", (mesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMeshChannel<T>(
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        attributeGetter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<*mut crate::UnityEngine::Mesh, T>,
        >,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMeshChannel", (gameObject, attributeGetter))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPrimitiveCount(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPrimitiveCount", (mesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertices(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::ProBuilder::Vertex,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::ProBuilder::Vertex,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVertices", (mesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsUsedInParticleSystem(
        pbmesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsUsedInParticleSystem", (pbmesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn Print(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Print", (mesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrintAttribute<T>(
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        attrib: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        fmt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PrintAttribute", (sb, title, attrib, fmt))?;
        Ok(__cordl_ret.into())
    }
    pub fn RestoreParticleSystem(
        pbmesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RestoreParticleSystem", (pbmesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn SanityCheck_IList_1_2(
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::UnityEngine::ProBuilder::Vertex,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SanityCheck", (vertices))?;
        Ok(__cordl_ret.into())
    }
    pub fn SanityCheck_Mesh1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SanityCheck", (mesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn SanityCheck_ProBuilderMesh0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::ProBuilderMesh>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SanityCheck", (mesh))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+MeshUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::MeshUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
