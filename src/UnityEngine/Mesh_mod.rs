#[cfg(feature = "UnityEngine+Mesh")]
#[repr(C)]
#[derive(Debug)]
pub struct Mesh {
    __cordl_parent: crate::UnityEngine::Object,
}
#[cfg(feature = "UnityEngine+Mesh")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Mesh => "UnityEngine"."Mesh"
);
#[cfg(feature = "UnityEngine+Mesh")]
impl std::ops::Deref for crate::UnityEngine::Mesh {
    type Target = crate::UnityEngine::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Mesh")]
impl std::ops::DerefMut for crate::UnityEngine::Mesh {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Mesh")]
impl crate::UnityEngine::Mesh {
    #[cfg(feature = "UnityEngine+Mesh+MeshData")]
    pub type MeshData = crate::UnityEngine::Mesh_MeshData;
    #[cfg(feature = "UnityEngine+Mesh+MeshDataArray")]
    pub type MeshDataArray = crate::UnityEngine::Mesh_MeshDataArray;
    pub fn AcquireReadOnlyMeshData_Il2CppArray1(
        meshes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Mesh>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Mesh_MeshDataArray> {
        let __cordl_ret: crate::UnityEngine::Mesh_MeshDataArray = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AcquireReadOnlyMeshData", (meshes))?;
        Ok(__cordl_ret.into())
    }
    pub fn AcquireReadOnlyMeshData_List_1_2(
        meshes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<*mut crate::UnityEngine::Mesh>,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Mesh_MeshDataArray> {
        let __cordl_ret: crate::UnityEngine::Mesh_MeshDataArray = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AcquireReadOnlyMeshData", (meshes))?;
        Ok(__cordl_ret.into())
    }
    pub fn AcquireReadOnlyMeshData_Mesh0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Mesh_MeshDataArray> {
        let __cordl_ret: crate::UnityEngine::Mesh_MeshDataArray = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AcquireReadOnlyMeshData", (mesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddBlendShapeFrame(
        &mut self,
        shapeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        frameWeight: f32,
        deltaVertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        deltaNormals: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        deltaTangents: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddBlendShapeFrame",
                (shapeName, frameWeight, deltaVertices, deltaNormals, deltaTangents),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AllocateWritableMeshData(
        meshCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Mesh_MeshDataArray> {
        let __cordl_ret: crate::UnityEngine::Mesh_MeshDataArray = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllocateWritableMeshData", (meshCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyAndDisposeWritableMeshData_Il2CppArray1(
        data: crate::UnityEngine::Mesh_MeshDataArray,
        meshes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Mesh>,
        >,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyAndDisposeWritableMeshData", (data, meshes, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyAndDisposeWritableMeshData_List_1_2(
        data: crate::UnityEngine::Mesh_MeshDataArray,
        meshes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<*mut crate::UnityEngine::Mesh>,
        >,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyAndDisposeWritableMeshData", (data, meshes, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyAndDisposeWritableMeshData_Mesh0(
        data: crate::UnityEngine::Mesh_MeshDataArray,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyAndDisposeWritableMeshData", (data, mesh, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckCanAccessSubmesh(
        &mut self,
        submesh: i32,
        errorAboutTriangles: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckCanAccessSubmesh", (submesh, errorAboutTriangles))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckCanAccessSubmeshIndices(
        &mut self,
        submesh: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckCanAccessSubmeshIndices", (submesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckCanAccessSubmeshTriangles(
        &mut self,
        submesh: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckCanAccessSubmeshTriangles", (submesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckIndicesArrayRange(
        &mut self,
        valuesLength: i32,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckIndicesArrayRange", (valuesLength, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearBlendShapes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearBlendShapes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearImpl(
        &mut self,
        keepVertexLayout: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearImpl", (keepVertexLayout))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear__cordl_bool0(
        &mut self,
        keepVertexLayout: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", (keepVertexLayout))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineMeshesImpl(
        &mut self,
        combine: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::CombineInstance>,
        >,
        mergeSubMeshes: bool,
        useMatrices: bool,
        hasLightmapData: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CombineMeshesImpl",
                (combine, mergeSubMeshes, useMatrices, hasLightmapData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineMeshes_Il2CppArray3(
        &mut self,
        combine: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::CombineInstance>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CombineMeshes", (combine))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineMeshes__cordl_bool2(
        &mut self,
        combine: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::CombineInstance>,
        >,
        mergeSubMeshes: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CombineMeshes", (combine, mergeSubMeshes))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineMeshes__cordl_bool__cordl_bool1(
        &mut self,
        combine: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::CombineInstance>,
        >,
        mergeSubMeshes: bool,
        useMatrices: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CombineMeshes", (combine, mergeSubMeshes, useMatrices))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineMeshes__cordl_bool__cordl_bool__cordl_bool0(
        &mut self,
        combine: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::CombineInstance>,
        >,
        mergeSubMeshes: bool,
        useMatrices: bool,
        hasLightmapData: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CombineMeshes",
                (combine, mergeSubMeshes, useMatrices, hasLightmapData),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DefaultDimensionForChannel(
        channel: crate::UnityEngine::Rendering::VertexAttribute,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DefaultDimensionForChannel", (channel))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromInstanceID(
        id: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromInstanceID", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllBoneWeights(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::BoneWeight1>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::BoneWeight1,
        > = __cordl_object.invoke("GetAllBoneWeights", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllBoneWeightsArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("GetAllBoneWeightsArray", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllBoneWeightsArraySize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetAllBoneWeightsArraySize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllocArrayFromChannelImpl(
        &mut self,
        channel: crate::UnityEngine::Rendering::VertexAttribute,
        format: crate::UnityEngine::Rendering::VertexAttributeFormat,
        dim: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Array>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Array> = __cordl_object
            .invoke("GetAllocArrayFromChannelImpl", (channel, format, dim))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllocArrayFromChannel_VertexAttribute1<T>(
        &mut self,
        channel: crate::UnityEngine::Rendering::VertexAttribute,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = __cordl_object.invoke("GetAllocArrayFromChannel", (channel))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllocArrayFromChannel_VertexAttributeFormat_i32_0<T>(
        &mut self,
        channel: crate::UnityEngine::Rendering::VertexAttribute,
        format: crate::UnityEngine::Rendering::VertexAttributeFormat,
        dim: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = __cordl_object.invoke("GetAllocArrayFromChannel", (channel, format, dim))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetArrayFromChannelImpl(
        &mut self,
        channel: crate::UnityEngine::Rendering::VertexAttribute,
        format: crate::UnityEngine::Rendering::VertexAttributeFormat,
        dim: i32,
        values: quest_hook::libil2cpp::Gc<crate::System::Array>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetArrayFromChannelImpl", (channel, format, dim, values))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBaseVertex(&mut self, submesh: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("GetBaseVertex", (submesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBaseVertexImpl(
        &mut self,
        submesh: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("GetBaseVertexImpl", (submesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBindposesArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("GetBindposesArray", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBindposesNonAllocImpl(
        &mut self,
        values: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetBindposesNonAllocImpl", (values))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBindposes_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Matrix4x4>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Matrix4x4,
        > = __cordl_object.invoke("GetBindposes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBindposes_List_1_1(
        &mut self,
        bindposes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetBindposes", (bindposes))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBlendShapeBufferImpl(
        &mut self,
        layout: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer> = __cordl_object
            .invoke("GetBlendShapeBufferImpl", (layout))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBlendShapeBufferRange(
        &mut self,
        blendShapeIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::BlendShapeBufferRange> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::BlendShapeBufferRange = __cordl_object
            .invoke("GetBlendShapeBufferRange", (blendShapeIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBlendShapeBuffer_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer> = __cordl_object
            .invoke("GetBlendShapeBuffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBlendShapeBuffer_BlendShapeBufferLayout0(
        &mut self,
        layout: crate::UnityEngine::Rendering::BlendShapeBufferLayout,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer> = __cordl_object
            .invoke("GetBlendShapeBuffer", (layout))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBlendShapeFrameCount(
        &mut self,
        shapeIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetBlendShapeFrameCount", (shapeIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBlendShapeFrameVertices(
        &mut self,
        shapeIndex: i32,
        frameIndex: i32,
        deltaVertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        deltaNormals: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        deltaTangents: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetBlendShapeFrameVertices",
                (shapeIndex, frameIndex, deltaVertices, deltaNormals, deltaTangents),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBlendShapeFrameWeight(
        &mut self,
        shapeIndex: i32,
        frameIndex: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetBlendShapeFrameWeight", (shapeIndex, frameIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBlendShapeIndex(
        &mut self,
        blendShapeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetBlendShapeIndex", (blendShapeName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBlendShapeName(
        &mut self,
        shapeIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetBlendShapeName", (shapeIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBlendShapeOffsetInternal(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::BlendShape> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::BlendShape = __cordl_object
            .invoke("GetBlendShapeOffsetInternal", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBlendShapeOffsetInternal_Injected(
        &mut self,
        index: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::BlendShape>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetBlendShapeOffsetInternal_Injected", (index, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoneWeightBuffer(
        &mut self,
        layout: crate::UnityEngine::SkinWeights,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer> = __cordl_object
            .invoke("GetBoneWeightBuffer", (layout))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoneWeightBufferImpl(
        &mut self,
        bonesPerVertex: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer> = __cordl_object
            .invoke("GetBoneWeightBufferImpl", (bonesPerVertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoneWeightBufferLayoutInternal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetBoneWeightBufferLayoutInternal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoneWeights(
        &mut self,
        boneWeights: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::BoneWeight>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetBoneWeights", (boneWeights))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoneWeightsImpl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::BoneWeight>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::BoneWeight>,
        > = __cordl_object.invoke("GetBoneWeightsImpl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBoneWeightsNonAllocImpl(
        &mut self,
        values: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::BoneWeight>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetBoneWeightsNonAllocImpl", (values))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBonesPerVertex(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<u8> = __cordl_object
            .invoke("GetBonesPerVertex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBonesPerVertexArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("GetBonesPerVertexArray", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetColors_List_1_0(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Color>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetColors", (colors))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetColors_List_1_1(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Color32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetColors", (colors))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndexBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer> = __cordl_object
            .invoke("GetIndexBuffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndexBufferImpl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer> = __cordl_object
            .invoke("GetIndexBufferImpl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndexCount(&mut self, submesh: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("GetIndexCount", (submesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndexCountImpl(
        &mut self,
        submesh: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("GetIndexCountImpl", (submesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndexStart(&mut self, submesh: i32) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("GetIndexStart", (submesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndexStartImpl(
        &mut self,
        submesh: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("GetIndexStartImpl", (submesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndicesImpl(
        &mut self,
        submesh: i32,
        applyBaseVertex: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = __cordl_object.invoke("GetIndicesImpl", (submesh, applyBaseVertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndicesNonAllocImpl(
        &mut self,
        values: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
        submesh: i32,
        applyBaseVertex: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetIndicesNonAllocImpl", (values, submesh, applyBaseVertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndicesNonAllocImpl16(
        &mut self,
        values: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u16>,
        >,
        submesh: i32,
        applyBaseVertex: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetIndicesNonAllocImpl16", (values, submesh, applyBaseVertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndices_List_1_i32_2(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
        submesh: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetIndices", (indices, submesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndices_List_1_i32__cordl_bool3(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
        submesh: i32,
        applyBaseVertex: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetIndices", (indices, submesh, applyBaseVertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndices_List_1_i32__cordl_bool4(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<u16>,
        >,
        submesh: i32,
        applyBaseVertex: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetIndices", (indices, submesh, applyBaseVertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndices_i32_0(
        &mut self,
        submesh: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = __cordl_object.invoke("GetIndices", (submesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndices_i32__cordl_bool1(
        &mut self,
        submesh: i32,
        applyBaseVertex: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = __cordl_object.invoke("GetIndices", (submesh, applyBaseVertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetListForChannel_List_1_i32_VertexAttribute_i32_0<T>(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        >,
        capacity: i32,
        channel: crate::UnityEngine::Rendering::VertexAttribute,
        dim: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetListForChannel", (buffer, capacity, channel, dim))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetListForChannel_VertexAttributeFormat1<T>(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        >,
        capacity: i32,
        channel: crate::UnityEngine::Rendering::VertexAttribute,
        dim: i32,
        channelType: crate::UnityEngine::Rendering::VertexAttributeFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetListForChannel", (buffer, capacity, channel, dim, channelType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNativeIndexBufferPtr(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("GetNativeIndexBufferPtr", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNativeVertexBufferPtr(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("GetNativeVertexBufferPtr", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNormals(
        &mut self,
        normals: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetNormals", (normals))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSubMesh(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::SubMeshDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::SubMeshDescriptor = __cordl_object
            .invoke("GetSubMesh", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSubMesh_Injected(
        &mut self,
        index: i32,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::SubMeshDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetSubMesh_Injected", (index, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTangents(
        &mut self,
        tangents: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetTangents", (tangents))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTopology(
        &mut self,
        submesh: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::MeshTopology> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::MeshTopology = __cordl_object
            .invoke("GetTopology", (submesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTopologyImpl(
        &mut self,
        submesh: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::MeshTopology> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::MeshTopology = __cordl_object
            .invoke("GetTopologyImpl", (submesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTotalIndexCount(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("GetTotalIndexCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTrianglesCountImpl(
        &mut self,
        submesh: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("GetTrianglesCountImpl", (submesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTrianglesImpl(
        &mut self,
        submesh: i32,
        applyBaseVertex: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = __cordl_object.invoke("GetTrianglesImpl", (submesh, applyBaseVertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTrianglesNonAllocImpl(
        &mut self,
        values: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        >,
        submesh: i32,
        applyBaseVertex: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetTrianglesNonAllocImpl", (values, submesh, applyBaseVertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTrianglesNonAllocImpl16(
        &mut self,
        values: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<u16>,
        >,
        submesh: i32,
        applyBaseVertex: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetTrianglesNonAllocImpl16", (values, submesh, applyBaseVertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTriangles_List_1_i32_2(
        &mut self,
        triangles: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
        submesh: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetTriangles", (triangles, submesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTriangles_List_1_i32__cordl_bool3(
        &mut self,
        triangles: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
        submesh: i32,
        applyBaseVertex: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetTriangles", (triangles, submesh, applyBaseVertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTriangles_List_1_i32__cordl_bool4(
        &mut self,
        triangles: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<u16>,
        >,
        submesh: i32,
        applyBaseVertex: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetTriangles", (triangles, submesh, applyBaseVertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTriangles_i32_0(
        &mut self,
        submesh: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = __cordl_object.invoke("GetTriangles", (submesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTriangles_i32__cordl_bool1(
        &mut self,
        submesh: i32,
        applyBaseVertex: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = __cordl_object.invoke("GetTriangles", (submesh, applyBaseVertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUVChannel(
        uvIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::VertexAttribute> {
        let __cordl_ret: crate::UnityEngine::Rendering::VertexAttribute = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUVChannel", (uvIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUVDistributionMetric(
        &mut self,
        uvSetIndex: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetUVDistributionMetric", (uvSetIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUVsImpl<T>(
        &mut self,
        uvIndex: i32,
        uvs: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        dim: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetUVsImpl", (uvIndex, uvs, dim))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUVs_i32_List_1_0(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetUVs", (channel, uvs))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUVs_i32_List_1_1(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetUVs", (channel, uvs))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUVs_i32_List_1_2(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetUVs", (channel, uvs))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexAttribute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::VertexAttributeDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::VertexAttributeDescriptor = __cordl_object
            .invoke("GetVertexAttribute", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexAttributeCountImpl(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetVertexAttributeCountImpl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexAttributeDimension(
        &mut self,
        attr: crate::UnityEngine::Rendering::VertexAttribute,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetVertexAttributeDimension", (attr))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexAttributeFormat(
        &mut self,
        attr: crate::UnityEngine::Rendering::VertexAttribute,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::VertexAttributeFormat,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::VertexAttributeFormat = __cordl_object
            .invoke("GetVertexAttributeFormat", (attr))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexAttributeOffset(
        &mut self,
        attr: crate::UnityEngine::Rendering::VertexAttribute,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetVertexAttributeOffset", (attr))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexAttributeStream(
        &mut self,
        attr: crate::UnityEngine::Rendering::VertexAttribute,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetVertexAttributeStream", (attr))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexAttribute_Injected(
        &mut self,
        index: i32,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::VertexAttributeDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetVertexAttribute_Injected", (index, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexAttributesAlloc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Array>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Array> = __cordl_object
            .invoke("GetVertexAttributesAlloc", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexAttributesArray(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::VertexAttributeDescriptor,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetVertexAttributesArray", (attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexAttributesList(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Rendering::VertexAttributeDescriptor,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetVertexAttributesList", (attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexAttributes_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::VertexAttributeDescriptor,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::VertexAttributeDescriptor,
            >,
        > = __cordl_object.invoke("GetVertexAttributes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexAttributes_Il2CppArray1(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::VertexAttributeDescriptor,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetVertexAttributes", (attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexAttributes_List_1_2(
        &mut self,
        attributes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Rendering::VertexAttributeDescriptor,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetVertexAttributes", (attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexBuffer(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer> = __cordl_object
            .invoke("GetVertexBuffer", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexBufferImpl(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GraphicsBuffer> = __cordl_object
            .invoke("GetVertexBufferImpl", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexBufferStride(
        &mut self,
        stream: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetVertexBufferStride", (stream))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertices(
        &mut self,
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetVertices", (vertices))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasBoneWeights(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasBoneWeights", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasVertexAttribute(
        &mut self,
        attr: crate::UnityEngine::Rendering::VertexAttribute,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasVertexAttribute", (attr))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetBoneWeights(
        &mut self,
        bonesPerVertex: crate::System::IntPtr,
        bonesPerVertexSize: i32,
        weights: crate::System::IntPtr,
        weightsSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InternalSetBoneWeights",
                (bonesPerVertex, bonesPerVertexSize, weights, weightsSize),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetIndexBufferData(
        &mut self,
        data: crate::System::IntPtr,
        dataStart: i32,
        meshBufferStart: i32,
        count: i32,
        elemSize: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InternalSetIndexBufferData",
                (data, dataStart, meshBufferStart, count, elemSize, flags),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetIndexBufferDataFromArray(
        &mut self,
        data: quest_hook::libil2cpp::Gc<crate::System::Array>,
        dataStart: i32,
        meshBufferStart: i32,
        count: i32,
        elemSize: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InternalSetIndexBufferDataFromArray",
                (data, dataStart, meshBufferStart, count, elemSize, flags),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetVertexBufferData(
        &mut self,
        stream: i32,
        data: crate::System::IntPtr,
        dataStart: i32,
        meshBufferStart: i32,
        count: i32,
        elemSize: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InternalSetVertexBufferData",
                (stream, data, dataStart, meshBufferStart, count, elemSize, flags),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalSetVertexBufferDataFromArray(
        &mut self,
        stream: i32,
        data: quest_hook::libil2cpp::Gc<crate::System::Array>,
        dataStart: i32,
        meshBufferStart: i32,
        count: i32,
        elemSize: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "InternalSetVertexBufferDataFromArray",
                (stream, data, dataStart, meshBufferStart, count, elemSize, flags),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_Create(
        mono: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_Create", (mono))?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkDynamic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkDynamic", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkDynamicImpl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkDynamicImpl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MarkModified(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkModified", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Optimize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Optimize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OptimizeImpl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OptimizeImpl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OptimizeIndexBuffers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OptimizeIndexBuffers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OptimizeIndexBuffersImpl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OptimizeIndexBuffersImpl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OptimizeReorderVertexBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OptimizeReorderVertexBuffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OptimizeReorderVertexBufferImpl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OptimizeReorderVertexBufferImpl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn PrintErrorCantAccessChannel(
        &mut self,
        ch: crate::UnityEngine::Rendering::VertexAttribute,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrintErrorCantAccessChannel", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn PrintErrorCantAccessIndices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrintErrorCantAccessIndices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RecalculateBoundsImpl(
        &mut self,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecalculateBoundsImpl", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecalculateBounds_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecalculateBounds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RecalculateBounds_MeshUpdateFlags1(
        &mut self,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecalculateBounds", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecalculateNormalsImpl(
        &mut self,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecalculateNormalsImpl", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecalculateNormals_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecalculateNormals", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RecalculateNormals_MeshUpdateFlags1(
        &mut self,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecalculateNormals", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecalculateTangentsImpl(
        &mut self,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecalculateTangentsImpl", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecalculateTangents_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecalculateTangents", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RecalculateTangents_MeshUpdateFlags1(
        &mut self,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecalculateTangents", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecalculateUVDistributionMetric(
        &mut self,
        uvSetIndex: i32,
        uvAreaThreshold: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecalculateUVDistributionMetric", (uvSetIndex, uvAreaThreshold))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecalculateUVDistributionMetricImpl(
        &mut self,
        uvSetIndex: i32,
        uvAreaThreshold: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RecalculateUVDistributionMetricImpl",
                (uvSetIndex, uvAreaThreshold),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RecalculateUVDistributionMetrics(
        &mut self,
        uvAreaThreshold: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecalculateUVDistributionMetrics", (uvAreaThreshold))?;
        Ok(__cordl_ret.into())
    }
    pub fn RecalculateUVDistributionMetricsImpl(
        &mut self,
        uvAreaThreshold: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecalculateUVDistributionMetricsImpl", (uvAreaThreshold))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAllSubMeshesAtOnceFromArray(
        &mut self,
        desc: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::SubMeshDescriptor,
            >,
        >,
        start: i32,
        count: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAllSubMeshesAtOnceFromArray", (desc, start, count, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetAllSubMeshesAtOnceFromNativeArray(
        &mut self,
        desc: crate::System::IntPtr,
        start: i32,
        count: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetAllSubMeshesAtOnceFromNativeArray",
                (desc, start, count, flags),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetArrayForChannelImpl(
        &mut self,
        channel: crate::UnityEngine::Rendering::VertexAttribute,
        format: crate::UnityEngine::Rendering::VertexAttributeFormat,
        dim: i32,
        values: quest_hook::libil2cpp::Gc<crate::System::Array>,
        arraySize: i32,
        valuesStart: i32,
        valuesCount: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetArrayForChannelImpl",
                (
                    channel,
                    format,
                    dim,
                    values,
                    arraySize,
                    valuesStart,
                    valuesCount,
                    flags,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetArrayForChannel_Il2CppArray_MeshUpdateFlags1<T>(
        &mut self,
        channel: crate::UnityEngine::Rendering::VertexAttribute,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetArrayForChannel", (channel, values, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetArrayForChannel_VertexAttributeFormat_i32_Il2CppArray_MeshUpdateFlags0<T>(
        &mut self,
        channel: crate::UnityEngine::Rendering::VertexAttribute,
        format: crate::UnityEngine::Rendering::VertexAttributeFormat,
        dim: i32,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetArrayForChannel", (channel, format, dim, values, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBoneWeights(
        &mut self,
        bonesPerVertex: crate::Unity::Collections::NativeArray_1<u8>,
        weights: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::BoneWeight1,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBoneWeights", (bonesPerVertex, weights))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBoneWeightsImpl(
        &mut self,
        weights: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::BoneWeight>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBoneWeightsImpl", (weights))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColors_Il2CppArray3(
        &mut self,
        inColors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColors", (inColors))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColors_Il2CppArray9(
        &mut self,
        inColors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColors", (inColors))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColors_Il2CppArray_i32_i32_10(
        &mut self,
        inColors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColors", (inColors, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColors_Il2CppArray_i32_i32_4(
        &mut self,
        inColors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColors", (inColors, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColors_Il2CppArray_i32_i32_MeshUpdateFlags11(
        &mut self,
        inColors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColors", (inColors, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColors_Il2CppArray_i32_i32_MeshUpdateFlags5(
        &mut self,
        inColors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColors", (inColors, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColors_List_1_0(
        &mut self,
        inColors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Color>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColors", (inColors))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColors_List_1_6(
        &mut self,
        inColors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Color32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColors", (inColors))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColors_List_1_i32_i32_1(
        &mut self,
        inColors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Color>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColors", (inColors, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColors_List_1_i32_i32_7(
        &mut self,
        inColors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Color32>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColors", (inColors, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColors_List_1_i32_i32_MeshUpdateFlags2(
        &mut self,
        inColors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Color>,
        >,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColors", (inColors, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColors_List_1_i32_i32_MeshUpdateFlags8(
        &mut self,
        inColors: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Color32>,
        >,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColors", (inColors, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColors_NativeArray_1_12<T>(
        &mut self,
        inColors: crate::Unity::Collections::NativeArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColors", (inColors))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColors_NativeArray_1_i32_i32_13<T>(
        &mut self,
        inColors: crate::Unity::Collections::NativeArray_1<T>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColors", (inColors, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColors_NativeArray_1_i32_i32_MeshUpdateFlags14<T>(
        &mut self,
        inColors: crate::Unity::Collections::NativeArray_1<T>,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColors", (inColors, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIndexBufferData_Il2CppArray1<T>(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        dataStart: i32,
        meshBufferStart: i32,
        count: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetIndexBufferData",
                (data, dataStart, meshBufferStart, count, flags),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIndexBufferData_List_1_2<T>(
        &mut self,
        data: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        dataStart: i32,
        meshBufferStart: i32,
        count: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetIndexBufferData",
                (data, dataStart, meshBufferStart, count, flags),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIndexBufferData_NativeArray_1_0<T>(
        &mut self,
        data: crate::Unity::Collections::NativeArray_1<T>,
        dataStart: i32,
        meshBufferStart: i32,
        count: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetIndexBufferData",
                (data, dataStart, meshBufferStart, count, flags),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIndexBufferParams(
        &mut self,
        indexCount: i32,
        format: crate::UnityEngine::Rendering::IndexFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIndexBufferParams", (indexCount, format))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIndicesImpl(
        &mut self,
        submesh: i32,
        topology: crate::UnityEngine::MeshTopology,
        indicesFormat: crate::UnityEngine::Rendering::IndexFormat,
        indices: quest_hook::libil2cpp::Gc<crate::System::Array>,
        arrayStart: i32,
        arraySize: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetIndicesImpl",
                (
                    submesh,
                    topology,
                    indicesFormat,
                    indices,
                    arrayStart,
                    arraySize,
                    calculateBounds,
                    baseVertex,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIndicesNativeArrayImpl(
        &mut self,
        submesh: i32,
        topology: crate::UnityEngine::MeshTopology,
        indicesFormat: crate::UnityEngine::Rendering::IndexFormat,
        indices: crate::System::IntPtr,
        arrayStart: i32,
        arraySize: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetIndicesNativeArrayImpl",
                (
                    submesh,
                    topology,
                    indicesFormat,
                    indices,
                    arrayStart,
                    arraySize,
                    calculateBounds,
                    baseVertex,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIndices_Il2CppArray_MeshTopology0(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        topology: crate::UnityEngine::MeshTopology,
        submesh: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIndices", (indices, topology, submesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIndices_Il2CppArray_MeshTopology__cordl_bool1(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        topology: crate::UnityEngine::MeshTopology,
        submesh: i32,
        calculateBounds: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIndices", (indices, topology, submesh, calculateBounds))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIndices_Il2CppArray_MeshTopology__cordl_bool_i32_2(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        topology: crate::UnityEngine::MeshTopology,
        submesh: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetIndices",
                (indices, topology, submesh, calculateBounds, baseVertex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIndices_Il2CppArray_MeshTopology__cordl_bool_i32_4(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
        topology: crate::UnityEngine::MeshTopology,
        submesh: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetIndices",
                (indices, topology, submesh, calculateBounds, baseVertex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIndices_Il2CppArray_i32_MeshTopology_i32__cordl_bool_i32_3(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        indicesStart: i32,
        indicesLength: i32,
        topology: crate::UnityEngine::MeshTopology,
        submesh: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetIndices",
                (
                    indices,
                    indicesStart,
                    indicesLength,
                    topology,
                    submesh,
                    calculateBounds,
                    baseVertex,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIndices_Il2CppArray_i32_MeshTopology_i32__cordl_bool_i32_5(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
        indicesStart: i32,
        indicesLength: i32,
        topology: crate::UnityEngine::MeshTopology,
        submesh: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetIndices",
                (
                    indices,
                    indicesStart,
                    indicesLength,
                    topology,
                    submesh,
                    calculateBounds,
                    baseVertex,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIndices_List_1_MeshTopology__cordl_bool_i32_10(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<u16>,
        >,
        topology: crate::UnityEngine::MeshTopology,
        submesh: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetIndices",
                (indices, topology, submesh, calculateBounds, baseVertex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIndices_List_1_MeshTopology__cordl_bool_i32_8(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
        topology: crate::UnityEngine::MeshTopology,
        submesh: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetIndices",
                (indices, topology, submesh, calculateBounds, baseVertex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIndices_List_1_i32_MeshTopology_i32__cordl_bool_i32_11(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<u16>,
        >,
        indicesStart: i32,
        indicesLength: i32,
        topology: crate::UnityEngine::MeshTopology,
        submesh: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetIndices",
                (
                    indices,
                    indicesStart,
                    indicesLength,
                    topology,
                    submesh,
                    calculateBounds,
                    baseVertex,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIndices_List_1_i32_MeshTopology_i32__cordl_bool_i32_9(
        &mut self,
        indices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
        indicesStart: i32,
        indicesLength: i32,
        topology: crate::UnityEngine::MeshTopology,
        submesh: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetIndices",
                (
                    indices,
                    indicesStart,
                    indicesLength,
                    topology,
                    submesh,
                    calculateBounds,
                    baseVertex,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIndices_NativeArray_1_MeshTopology__cordl_bool_i32_6<T>(
        &mut self,
        indices: crate::Unity::Collections::NativeArray_1<T>,
        topology: crate::UnityEngine::MeshTopology,
        submesh: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetIndices",
                (indices, topology, submesh, calculateBounds, baseVertex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIndices_NativeArray_1_i32_MeshTopology_i32__cordl_bool_i32_7<T>(
        &mut self,
        indices: crate::Unity::Collections::NativeArray_1<T>,
        indicesStart: i32,
        indicesLength: i32,
        topology: crate::UnityEngine::MeshTopology,
        submesh: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetIndices",
                (
                    indices,
                    indicesStart,
                    indicesLength,
                    topology,
                    submesh,
                    calculateBounds,
                    baseVertex,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetListForChannel_List_1_i32_MeshUpdateFlags1<T>(
        &mut self,
        channel: crate::UnityEngine::Rendering::VertexAttribute,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        >,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetListForChannel", (channel, values, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetListForChannel_VertexAttributeFormat_List_1_i32_i32_MeshUpdateFlags0<T>(
        &mut self,
        channel: crate::UnityEngine::Rendering::VertexAttribute,
        format: crate::UnityEngine::Rendering::VertexAttributeFormat,
        dim: i32,
        values: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<T>,
        >,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetListForChannel",
                (channel, format, dim, values, start, length, flags),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNativeArrayForChannelImpl(
        &mut self,
        channel: crate::UnityEngine::Rendering::VertexAttribute,
        format: crate::UnityEngine::Rendering::VertexAttributeFormat,
        dim: i32,
        values: crate::System::IntPtr,
        arraySize: i32,
        valuesStart: i32,
        valuesCount: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetNativeArrayForChannelImpl",
                (
                    channel,
                    format,
                    dim,
                    values,
                    arraySize,
                    valuesStart,
                    valuesCount,
                    flags,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNormals_Il2CppArray3(
        &mut self,
        inNormals: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNormals", (inNormals))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNormals_Il2CppArray_i32_i32_4(
        &mut self,
        inNormals: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNormals", (inNormals, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNormals_Il2CppArray_i32_i32_MeshUpdateFlags5(
        &mut self,
        inNormals: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNormals", (inNormals, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNormals_List_1_0(
        &mut self,
        inNormals: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNormals", (inNormals))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNormals_List_1_i32_i32_1(
        &mut self,
        inNormals: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNormals", (inNormals, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNormals_List_1_i32_i32_MeshUpdateFlags2(
        &mut self,
        inNormals: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNormals", (inNormals, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNormals_NativeArray_1_6<T>(
        &mut self,
        inNormals: crate::Unity::Collections::NativeArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNormals", (inNormals))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNormals_NativeArray_1_i32_i32_7<T>(
        &mut self,
        inNormals: crate::Unity::Collections::NativeArray_1<T>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNormals", (inNormals, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNormals_NativeArray_1_i32_i32_MeshUpdateFlags8<T>(
        &mut self,
        inNormals: crate::Unity::Collections::NativeArray_1<T>,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNormals", (inNormals, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSizedArrayForChannel(
        &mut self,
        channel: crate::UnityEngine::Rendering::VertexAttribute,
        format: crate::UnityEngine::Rendering::VertexAttributeFormat,
        dim: i32,
        values: quest_hook::libil2cpp::Gc<crate::System::Array>,
        valuesArrayLength: i32,
        valuesStart: i32,
        valuesCount: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetSizedArrayForChannel",
                (
                    channel,
                    format,
                    dim,
                    values,
                    valuesArrayLength,
                    valuesStart,
                    valuesCount,
                    flags,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSizedNativeArrayForChannel(
        &mut self,
        channel: crate::UnityEngine::Rendering::VertexAttribute,
        format: crate::UnityEngine::Rendering::VertexAttributeFormat,
        dim: i32,
        values: crate::System::IntPtr,
        valuesArrayLength: i32,
        valuesStart: i32,
        valuesCount: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetSizedNativeArrayForChannel",
                (
                    channel,
                    format,
                    dim,
                    values,
                    valuesArrayLength,
                    valuesStart,
                    valuesCount,
                    flags,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSubMesh(
        &mut self,
        index: i32,
        desc: crate::UnityEngine::Rendering::SubMeshDescriptor,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSubMesh", (index, desc, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSubMesh_Injected(
        &mut self,
        index: i32,
        desc: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::SubMeshDescriptor,
        >,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSubMesh_Injected", (index, desc, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSubMeshes_Il2CppArray_MeshUpdateFlags1(
        &mut self,
        desc: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::SubMeshDescriptor,
            >,
        >,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSubMeshes", (desc, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSubMeshes_Il2CppArray_i32_i32_MeshUpdateFlags0(
        &mut self,
        desc: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::SubMeshDescriptor,
            >,
        >,
        start: i32,
        count: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSubMeshes", (desc, start, count, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSubMeshes_List_1_MeshUpdateFlags3(
        &mut self,
        desc: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Rendering::SubMeshDescriptor,
            >,
        >,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSubMeshes", (desc, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSubMeshes_List_1_i32_i32_MeshUpdateFlags2(
        &mut self,
        desc: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::Rendering::SubMeshDescriptor,
            >,
        >,
        start: i32,
        count: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSubMeshes", (desc, start, count, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSubMeshes_NativeArray_1_MeshUpdateFlags5<T>(
        &mut self,
        desc: crate::Unity::Collections::NativeArray_1<T>,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSubMeshes", (desc, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSubMeshes_NativeArray_1_i32_i32_MeshUpdateFlags4<T>(
        &mut self,
        desc: crate::Unity::Collections::NativeArray_1<T>,
        start: i32,
        count: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSubMeshes", (desc, start, count, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTangents_Il2CppArray3(
        &mut self,
        inTangents: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTangents", (inTangents))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTangents_Il2CppArray_i32_i32_4(
        &mut self,
        inTangents: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTangents", (inTangents, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTangents_Il2CppArray_i32_i32_MeshUpdateFlags5(
        &mut self,
        inTangents: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTangents", (inTangents, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTangents_List_1_0(
        &mut self,
        inTangents: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTangents", (inTangents))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTangents_List_1_i32_i32_1(
        &mut self,
        inTangents: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTangents", (inTangents, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTangents_List_1_i32_i32_MeshUpdateFlags2(
        &mut self,
        inTangents: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTangents", (inTangents, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTangents_NativeArray_1_6<T>(
        &mut self,
        inTangents: crate::Unity::Collections::NativeArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTangents", (inTangents))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTangents_NativeArray_1_i32_i32_7<T>(
        &mut self,
        inTangents: crate::Unity::Collections::NativeArray_1<T>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTangents", (inTangents, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTangents_NativeArray_1_i32_i32_MeshUpdateFlags8<T>(
        &mut self,
        inTangents: crate::Unity::Collections::NativeArray_1<T>,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTangents", (inTangents, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTrianglesImpl(
        &mut self,
        submesh: i32,
        indicesFormat: crate::UnityEngine::Rendering::IndexFormat,
        triangles: quest_hook::libil2cpp::Gc<crate::System::Array>,
        trianglesArrayLength: i32,
        start: i32,
        length: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetTrianglesImpl",
                (
                    submesh,
                    indicesFormat,
                    triangles,
                    trianglesArrayLength,
                    start,
                    length,
                    calculateBounds,
                    baseVertex,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTriangles_Il2CppArray0(
        &mut self,
        triangles: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        submesh: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTriangles", (triangles, submesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTriangles_Il2CppArray__cordl_bool1(
        &mut self,
        triangles: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        submesh: i32,
        calculateBounds: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTriangles", (triangles, submesh, calculateBounds))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTriangles_Il2CppArray__cordl_bool_i32_2(
        &mut self,
        triangles: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        submesh: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTriangles", (triangles, submesh, calculateBounds, baseVertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTriangles_Il2CppArray__cordl_bool_i32_4(
        &mut self,
        triangles: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
        submesh: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTriangles", (triangles, submesh, calculateBounds, baseVertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTriangles_Il2CppArray_i32_i32__cordl_bool_i32_3(
        &mut self,
        triangles: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
        trianglesStart: i32,
        trianglesLength: i32,
        submesh: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetTriangles",
                (
                    triangles,
                    trianglesStart,
                    trianglesLength,
                    submesh,
                    calculateBounds,
                    baseVertex,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTriangles_Il2CppArray_i32_i32__cordl_bool_i32_5(
        &mut self,
        triangles: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
        trianglesStart: i32,
        trianglesLength: i32,
        submesh: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetTriangles",
                (
                    triangles,
                    trianglesStart,
                    trianglesLength,
                    submesh,
                    calculateBounds,
                    baseVertex,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTriangles_List_1_6(
        &mut self,
        triangles: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
        submesh: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTriangles", (triangles, submesh))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTriangles_List_1__cordl_bool7(
        &mut self,
        triangles: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
        submesh: i32,
        calculateBounds: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTriangles", (triangles, submesh, calculateBounds))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTriangles_List_1__cordl_bool_i32_10(
        &mut self,
        triangles: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<u16>,
        >,
        submesh: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTriangles", (triangles, submesh, calculateBounds, baseVertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTriangles_List_1__cordl_bool_i32_8(
        &mut self,
        triangles: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
        submesh: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTriangles", (triangles, submesh, calculateBounds, baseVertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTriangles_List_1_i32_i32__cordl_bool_i32_11(
        &mut self,
        triangles: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<u16>,
        >,
        trianglesStart: i32,
        trianglesLength: i32,
        submesh: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetTriangles",
                (
                    triangles,
                    trianglesStart,
                    trianglesLength,
                    submesh,
                    calculateBounds,
                    baseVertex,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTriangles_List_1_i32_i32__cordl_bool_i32_9(
        &mut self,
        triangles: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<i32>,
        >,
        trianglesStart: i32,
        trianglesLength: i32,
        submesh: i32,
        calculateBounds: bool,
        baseVertex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetTriangles",
                (
                    triangles,
                    trianglesStart,
                    trianglesLength,
                    submesh,
                    calculateBounds,
                    baseVertex,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_Il2CppArray10(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_Il2CppArray11(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_Il2CppArray9(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_Il2CppArray_i32_i32_12(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_Il2CppArray_i32_i32_14(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_Il2CppArray_i32_i32_16(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_Il2CppArray_i32_i32_MeshUpdateFlags13(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_Il2CppArray_i32_i32_MeshUpdateFlags15(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_Il2CppArray_i32_i32_MeshUpdateFlags17(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_List_1_0(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_List_1_1(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_List_1_2(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_List_1_i32_i32_3(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_List_1_i32_i32_5(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_List_1_i32_i32_7(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_List_1_i32_i32_MeshUpdateFlags4(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector2>,
        >,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_List_1_i32_i32_MeshUpdateFlags6(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_List_1_i32_i32_MeshUpdateFlags8(
        &mut self,
        channel: i32,
        uvs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
        >,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_NativeArray_1_18<T>(
        &mut self,
        channel: i32,
        uvs: crate::Unity::Collections::NativeArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_NativeArray_1_i32_i32_19<T>(
        &mut self,
        channel: i32,
        uvs: crate::Unity::Collections::NativeArray_1<T>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUVs_NativeArray_1_i32_i32_MeshUpdateFlags20<T>(
        &mut self,
        channel: i32,
        uvs: crate::Unity::Collections::NativeArray_1<T>,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUVs", (channel, uvs, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUvsImpl_Array1(
        &mut self,
        uvIndex: i32,
        dim: i32,
        uvs: quest_hook::libil2cpp::Gc<crate::System::Array>,
        arrayStart: i32,
        arraySize: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUvsImpl", (uvIndex, dim, uvs, arrayStart, arraySize, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetUvsImpl_List_1_0<T>(
        &mut self,
        uvIndex: i32,
        dim: i32,
        uvs: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUvsImpl", (uvIndex, dim, uvs, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVertexBufferData_Il2CppArray1<T>(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        dataStart: i32,
        meshBufferStart: i32,
        count: i32,
        stream: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetVertexBufferData",
                (data, dataStart, meshBufferStart, count, stream, flags),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVertexBufferData_List_1_2<T>(
        &mut self,
        data: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::List_1<T>>,
        dataStart: i32,
        meshBufferStart: i32,
        count: i32,
        stream: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetVertexBufferData",
                (data, dataStart, meshBufferStart, count, stream, flags),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVertexBufferData_NativeArray_1_0<T>(
        &mut self,
        data: crate::Unity::Collections::NativeArray_1<T>,
        dataStart: i32,
        meshBufferStart: i32,
        count: i32,
        stream: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetVertexBufferData",
                (data, dataStart, meshBufferStart, count, stream, flags),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVertexBufferParamsFromArray(
        &mut self,
        vertexCount: i32,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::VertexAttributeDescriptor,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVertexBufferParamsFromArray", (vertexCount, attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVertexBufferParamsFromPtr(
        &mut self,
        vertexCount: i32,
        attributesPtr: crate::System::IntPtr,
        attributesCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetVertexBufferParamsFromPtr",
                (vertexCount, attributesPtr, attributesCount),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVertexBufferParams_Il2CppArray0(
        &mut self,
        vertexCount: i32,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::VertexAttributeDescriptor,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVertexBufferParams", (vertexCount, attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVertexBufferParams_NativeArray_1_1(
        &mut self,
        vertexCount: i32,
        attributes: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::Rendering::VertexAttributeDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVertexBufferParams", (vertexCount, attributes))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVertices_Il2CppArray3(
        &mut self,
        inVertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVertices", (inVertices))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVertices_Il2CppArray_i32_i32_4(
        &mut self,
        inVertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVertices", (inVertices, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVertices_Il2CppArray_i32_i32_MeshUpdateFlags5(
        &mut self,
        inVertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVertices", (inVertices, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVertices_List_1_0(
        &mut self,
        inVertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVertices", (inVertices))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVertices_List_1_i32_i32_1(
        &mut self,
        inVertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVertices", (inVertices, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVertices_List_1_i32_i32_MeshUpdateFlags2(
        &mut self,
        inVertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector3>,
        >,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVertices", (inVertices, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVertices_NativeArray_1_6<T>(
        &mut self,
        inVertices: crate::Unity::Collections::NativeArray_1<T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVertices", (inVertices))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVertices_NativeArray_1_i32_i32_7<T>(
        &mut self,
        inVertices: crate::Unity::Collections::NativeArray_1<T>,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVertices", (inVertices, start, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVertices_NativeArray_1_i32_i32_MeshUpdateFlags8<T>(
        &mut self,
        inVertices: crate::Unity::Collections::NativeArray_1<T>,
        start: i32,
        length: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetVertices", (inVertices, start, length, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn UploadMeshData(
        &mut self,
        markNoLongerReadable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UploadMeshData", (markNoLongerReadable))?;
        Ok(__cordl_ret.into())
    }
    pub fn UploadMeshDataImpl(
        &mut self,
        markNoLongerReadable: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UploadMeshDataImpl", (markNoLongerReadable))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bindposeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_bindposeCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bindposes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        > = __cordl_object.invoke("get_bindposes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_blendShapeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_blendShapeCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_boneWeights(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::BoneWeight>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::BoneWeight>,
        > = __cordl_object.invoke("get_boneWeights", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bounds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Bounds = __cordl_object
            .invoke("get_bounds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bounds_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_bounds_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_canAccess(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_canAccess", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_colors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        > = __cordl_object.invoke("get_colors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_colors32(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        > = __cordl_object.invoke("get_colors32", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_indexBufferTarget(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::GraphicsBuffer_Target> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::GraphicsBuffer_Target = __cordl_object
            .invoke("get_indexBufferTarget", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_indexFormat(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::IndexFormat> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::IndexFormat = __cordl_object
            .invoke("get_indexFormat", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isReadable(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isReadable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_normals(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        > = __cordl_object.invoke("get_normals", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_skinWeightBufferLayout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SkinWeights> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SkinWeights = __cordl_object
            .invoke("get_skinWeightBufferLayout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_subMeshCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_subMeshCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tangents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        > = __cordl_object.invoke("get_tangents", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_triangles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<i32>,
        > = __cordl_object.invoke("get_triangles", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_uv(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        > = __cordl_object.invoke("get_uv", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_uv2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        > = __cordl_object.invoke("get_uv2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_uv3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        > = __cordl_object.invoke("get_uv3", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_uv4(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        > = __cordl_object.invoke("get_uv4", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_uv5(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        > = __cordl_object.invoke("get_uv5", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_uv6(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        > = __cordl_object.invoke("get_uv6", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_uv7(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        > = __cordl_object.invoke("get_uv7", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_uv8(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        > = __cordl_object.invoke("get_uv8", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vertexAttributeCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_vertexAttributeCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vertexBufferCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_vertexBufferCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vertexBufferTarget(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::GraphicsBuffer_Target> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::GraphicsBuffer_Target = __cordl_object
            .invoke("get_vertexBufferTarget", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vertexCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_vertexCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vertices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        > = __cordl_object.invoke("get_vertices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bindposes(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bindposes", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_boneWeights(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::BoneWeight>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_boneWeights", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bounds(
        &mut self,
        value: crate::UnityEngine::Bounds,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bounds", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bounds_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Bounds>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bounds_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_colors(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_colors", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_colors32(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_colors32", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_indexBufferTarget(
        &mut self,
        value: crate::UnityEngine::GraphicsBuffer_Target,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_indexBufferTarget", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_indexFormat(
        &mut self,
        value: crate::UnityEngine::Rendering::IndexFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_indexFormat", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_normals(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_normals", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_subMeshCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_subMeshCount", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_tangents(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_tangents", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_triangles(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_triangles", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_uv(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_uv", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_uv2(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_uv2", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_uv3(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_uv3", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_uv4(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_uv4", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_uv5(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_uv5", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_uv6(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_uv6", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_uv7(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_uv7", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_uv8(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_uv8", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_vertexBufferTarget(
        &mut self,
        value: crate::UnityEngine::GraphicsBuffer_Target,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_vertexBufferTarget", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_vertices(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_vertices", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Mesh")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Mesh {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Mesh+MeshData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Mesh_MeshData {
    pub m_Ptr: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+Mesh+MeshData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Mesh_MeshData => "UnityEngine"
    ."Mesh/MeshData"
);
#[cfg(feature = "UnityEngine+Mesh+MeshData")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Mesh_MeshData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Mesh+MeshData")]
impl crate::UnityEngine::Mesh_MeshData {
    pub fn GetIndexData<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<T> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetIndexData",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndexDataPtr(
        _cordl_self: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIndexDataPtr", (_cordl_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIndexDataSize(
        _cordl_self: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIndexDataSize", (_cordl_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexBufferCount(
        _cordl_self: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVertexBufferCount", (_cordl_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexData<T>(
        &mut self,
        stream: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Collections::NativeArray_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Unity::Collections::NativeArray_1<T> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetVertexData",
            (stream),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexDataPtr(
        _cordl_self: crate::System::IntPtr,
        stream: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVertexDataPtr", (_cordl_self, stream))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetVertexDataSize(
        _cordl_self: crate::System::IntPtr,
        stream: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetVertexDataSize", (_cordl_self, stream))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIndexBufferParams(
        &mut self,
        indexCount: i32,
        format: crate::UnityEngine::Rendering::IndexFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetIndexBufferParams",
            (indexCount, format),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIndexBufferParamsImpl(
        _cordl_self: crate::System::IntPtr,
        indexCount: i32,
        indexFormat: crate::UnityEngine::Rendering::IndexFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetIndexBufferParamsImpl", (_cordl_self, indexCount, indexFormat))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSubMesh(
        &mut self,
        index: i32,
        desc: crate::UnityEngine::Rendering::SubMeshDescriptor,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetSubMesh",
            (index, desc, flags),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSubMeshCount(
        _cordl_self: crate::System::IntPtr,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSubMeshCount", (_cordl_self, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSubMeshImpl(
        _cordl_self: crate::System::IntPtr,
        index: i32,
        desc: crate::UnityEngine::Rendering::SubMeshDescriptor,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSubMeshImpl", (_cordl_self, index, desc, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSubMeshImpl_Injected(
        _cordl_self: crate::System::IntPtr,
        index: i32,
        desc: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::SubMeshDescriptor,
        >,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSubMeshImpl_Injected", (_cordl_self, index, desc, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVertexBufferParams(
        &mut self,
        vertexCount: i32,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::VertexAttributeDescriptor,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetVertexBufferParams",
            (vertexCount, attributes),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetVertexBufferParamsFromArray(
        _cordl_self: crate::System::IntPtr,
        vertexCount: i32,
        attributes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::VertexAttributeDescriptor,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetVertexBufferParamsFromArray",
                (_cordl_self, vertexCount, attributes),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vertexBufferCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_vertexBufferCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_subMeshCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_subMeshCount",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Mesh+MeshDataArray")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Mesh_MeshDataArray {
    pub m_Ptrs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_Length: i32,
}
#[cfg(feature = "UnityEngine+Mesh+MeshDataArray")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Mesh_MeshDataArray => "UnityEngine"
    ."Mesh/MeshDataArray"
);
#[cfg(feature = "UnityEngine+Mesh+MeshDataArray")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Mesh_MeshDataArray {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Mesh+MeshDataArray")]
impl crate::UnityEngine::Mesh_MeshDataArray {
    pub fn AcquireReadOnlyMeshData(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        datas: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AcquireReadOnlyMeshData", (mesh, datas))?;
        Ok(__cordl_ret.into())
    }
    pub fn AcquireReadOnlyMeshDatas(
        meshes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Mesh>,
        >,
        datas: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AcquireReadOnlyMeshDatas", (meshes, datas, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyToMeshAndDispose(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyToMeshAndDispose",
            (mesh, flags),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyToMeshImpl(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        data: crate::System::IntPtr,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyToMeshImpl", (mesh, data, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyToMeshesAndDispose(
        &mut self,
        meshes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Mesh>,
        >,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ApplyToMeshesAndDispose",
            (meshes, flags),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyToMeshesImpl(
        meshes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Mesh>,
        >,
        datas: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
        flags: crate::UnityEngine::Rendering::MeshUpdateFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyToMeshesImpl", (meshes, datas, count, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateNewMeshDatas(
        datas: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateNewMeshDatas", (datas, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseMeshDatas(
        datas: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReleaseMeshDatas", (datas, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray_i32__cordl_bool1(
        &mut self,
        meshes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Mesh>,
        >,
        meshesCount: i32,
        checkReadWrite: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (meshes, meshesCount, checkReadWrite),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Mesh__cordl_bool0(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        checkReadWrite: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (mesh, checkReadWrite),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_2(
        &mut self,
        meshesCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (meshesCount),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Mesh_MeshData> {
        let __cordl_ret: crate::UnityEngine::Mesh_MeshData = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Length",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Mesh+MeshDataArray")]
impl AsRef<crate::System::IDisposable> for crate::UnityEngine::Mesh_MeshDataArray {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Mesh+MeshDataArray")]
impl AsMut<crate::System::IDisposable> for crate::UnityEngine::Mesh_MeshDataArray {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
