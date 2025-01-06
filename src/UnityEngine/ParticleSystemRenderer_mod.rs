#[cfg(feature = "UnityEngine+ParticleSystemRenderer")]
#[repr(C)]
#[derive(Debug)]
pub struct ParticleSystemRenderer {
    __cordl_parent: crate::UnityEngine::Renderer,
}
#[cfg(feature = "UnityEngine+ParticleSystemRenderer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ParticleSystemRenderer =>
    "UnityEngine"."ParticleSystemRenderer"
);
#[cfg(feature = "UnityEngine+ParticleSystemRenderer")]
impl std::ops::Deref for crate::UnityEngine::ParticleSystemRenderer {
    type Target = crate::UnityEngine::Renderer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystemRenderer")]
impl std::ops::DerefMut for crate::UnityEngine::ParticleSystemRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystemRenderer")]
impl crate::UnityEngine::ParticleSystemRenderer {
    #[cfg(feature = "UnityEngine+ParticleSystemRenderer+BakeTextureOutput")]
    pub type BakeTextureOutput = crate::UnityEngine::ParticleSystemRenderer_BakeTextureOutput;
    pub fn AreVertexStreamsEnabled(
        &mut self,
        streams: crate::UnityEngine::ParticleSystemVertexStreams,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AreVertexStreamsEnabled", (streams))?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeMesh_Camera_ParticleSystemBakeMeshOptions3(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        options: crate::UnityEngine::ParticleSystemBakeMeshOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BakeMesh", (mesh, camera, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeMesh_Camera__cordl_bool1(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        useTransform: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BakeMesh", (mesh, camera, useTransform))?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeMesh_ParticleSystemBakeMeshOptions2(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        options: crate::UnityEngine::ParticleSystemBakeMeshOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BakeMesh", (mesh, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeMesh__cordl_bool0(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        useTransform: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BakeMesh", (mesh, useTransform))?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeTextureInternal(
        &mut self,
        verticesTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        indicesTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        options: crate::UnityEngine::ParticleSystemBakeTextureOptions,
        indexCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystemRenderer_BakeTextureOutput,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystemRenderer_BakeTextureOutput = __cordl_object
            .invoke(
                "BakeTextureInternal",
                (verticesTexture, indicesTexture, camera, options, indexCount),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeTextureInternal_Injected(
        &mut self,
        verticesTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        indicesTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        options: crate::UnityEngine::ParticleSystemBakeTextureOptions,
        indexCount: quest_hook::libil2cpp::ByRefMut<i32>,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystemRenderer_BakeTextureOutput,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "BakeTextureInternal_Injected",
                (verticesTexture, indicesTexture, camera, options, indexCount, ret),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeTextureNoIndicesInternal(
        &mut self,
        verticesTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        options: crate::UnityEngine::ParticleSystemBakeTextureOptions,
        indexCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = __cordl_object
            .invoke(
                "BakeTextureNoIndicesInternal",
                (verticesTexture, camera, options, indexCount),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeTexture_ByRefMut_Camera_ParticleSystemBakeTextureOptions3(
        &mut self,
        verticesTexture: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
        indicesTexture: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        options: crate::UnityEngine::ParticleSystemBakeTextureOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("BakeTexture", (verticesTexture, indicesTexture, camera, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeTexture_ByRefMut_ParticleSystemBakeTextureOptions2(
        &mut self,
        verticesTexture: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
        indicesTexture: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
        options: crate::UnityEngine::ParticleSystemBakeTextureOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("BakeTexture", (verticesTexture, indicesTexture, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeTexture_Camera_ParticleSystemBakeTextureOptions1(
        &mut self,
        verticesTexture: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        options: crate::UnityEngine::ParticleSystemBakeTextureOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("BakeTexture", (verticesTexture, camera, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeTexture_ParticleSystemBakeTextureOptions0(
        &mut self,
        verticesTexture: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
        options: crate::UnityEngine::ParticleSystemBakeTextureOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("BakeTexture", (verticesTexture, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeTrailsMesh_Camera_ParticleSystemBakeMeshOptions3(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        options: crate::UnityEngine::ParticleSystemBakeMeshOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BakeTrailsMesh", (mesh, camera, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeTrailsMesh_Camera__cordl_bool1(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        useTransform: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BakeTrailsMesh", (mesh, camera, useTransform))?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeTrailsMesh_ParticleSystemBakeMeshOptions2(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        options: crate::UnityEngine::ParticleSystemBakeMeshOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BakeTrailsMesh", (mesh, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeTrailsMesh__cordl_bool0(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        useTransform: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BakeTrailsMesh", (mesh, useTransform))?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeTrailsTextureInternal(
        &mut self,
        verticesTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        indicesTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        options: crate::UnityEngine::ParticleSystemBakeTextureOptions,
        indexCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystemRenderer_BakeTextureOutput,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystemRenderer_BakeTextureOutput = __cordl_object
            .invoke(
                "BakeTrailsTextureInternal",
                (verticesTexture, indicesTexture, camera, options, indexCount),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeTrailsTextureInternal_Injected(
        &mut self,
        verticesTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        indicesTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        options: crate::UnityEngine::ParticleSystemBakeTextureOptions,
        indexCount: quest_hook::libil2cpp::ByRefMut<i32>,
        ret: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::ParticleSystemRenderer_BakeTextureOutput,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "BakeTrailsTextureInternal_Injected",
                (verticesTexture, indicesTexture, camera, options, indexCount, ret),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeTrailsTexture_Camera_ParticleSystemBakeTextureOptions1(
        &mut self,
        verticesTexture: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
        indicesTexture: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        options: crate::UnityEngine::ParticleSystemBakeTextureOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke(
                "BakeTrailsTexture",
                (verticesTexture, indicesTexture, camera, options),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BakeTrailsTexture_ParticleSystemBakeTextureOptions0(
        &mut self,
        verticesTexture: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
        indicesTexture: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
        options: crate::UnityEngine::ParticleSystemBakeTextureOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("BakeTrailsTexture", (verticesTexture, indicesTexture, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisableVertexStreams(
        &mut self,
        streams: crate::UnityEngine::ParticleSystemVertexStreams,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableVertexStreams", (streams))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnableVertexStreams(
        &mut self,
        streams: crate::UnityEngine::ParticleSystemVertexStreams,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnableVertexStreams", (streams))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetActiveTrailVertexStreams(
        &mut self,
        streams: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ParticleSystemVertexStream,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetActiveTrailVertexStreams", (streams))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetActiveVertexStreams(
        &mut self,
        streams: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ParticleSystemVertexStream,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetActiveVertexStreams", (streams))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnabledVertexStreams(
        &mut self,
        streams: crate::UnityEngine::ParticleSystemVertexStreams,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ParticleSystemVertexStreams> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystemVertexStreams = __cordl_object
            .invoke("GetEnabledVertexStreams", (streams))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMeshWeightings(
        &mut self,
        weightings: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetMeshWeightings", (weightings))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMeshes(
        &mut self,
        meshes: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetMeshes", (meshes))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetEnabledVertexStreams(
        &mut self,
        streams: crate::UnityEngine::ParticleSystemVertexStreams,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ParticleSystemVertexStreams> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystemVertexStreams = __cordl_object
            .invoke("Internal_GetEnabledVertexStreams", (streams))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetVertexStreams(
        &mut self,
        streams: crate::UnityEngine::ParticleSystemVertexStreams,
        enabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Internal_SetVertexStreams", (streams, enabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetActiveTrailVertexStreams(
        &mut self,
        streams: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ParticleSystemVertexStream,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetActiveTrailVertexStreams", (streams))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetActiveVertexStreams(
        &mut self,
        streams: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::ParticleSystemVertexStream,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetActiveVertexStreams", (streams))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMeshWeightings_Il2CppArray1(
        &mut self,
        weightings: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMeshWeightings", (weightings))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMeshWeightings_i32_0(
        &mut self,
        weightings: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMeshWeightings", (weightings, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMeshes_Il2CppArray1(
        &mut self,
        meshes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMeshes", (meshes))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMeshes_i32_0(
        &mut self,
        meshes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
            >,
        >,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMeshes", (meshes, _cordl_size))?;
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
    pub fn get_activeTrailVertexStreamsCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_activeTrailVertexStreamsCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_activeVertexStreamsCount(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_activeVertexStreamsCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_alignment(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ParticleSystemRenderSpace> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystemRenderSpace = __cordl_object
            .invoke("get_alignment", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_allowRoll(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_allowRoll", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cameraVelocityScale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_cameraVelocityScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enableGPUInstancing(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableGPUInstancing", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flip(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_flip", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flip_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_flip_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_freeformStretching(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_freeformStretching", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lengthScale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_lengthScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maskInteraction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::SpriteMaskInteraction> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::SpriteMaskInteraction = __cordl_object
            .invoke("get_maskInteraction", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxParticleSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_maxParticleSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh> = __cordl_object
            .invoke("get_mesh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_meshCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_meshCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_meshDistribution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ParticleSystemMeshDistribution,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystemMeshDistribution = __cordl_object
            .invoke("get_meshDistribution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_minParticleSize(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_minParticleSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_normalDirection(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_normalDirection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pivot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_pivot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pivot_Injected(
        &mut self,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("get_pivot_Injected", (ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_renderMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ParticleSystemRenderMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystemRenderMode = __cordl_object
            .invoke("get_renderMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotateWithStretchDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_rotateWithStretchDirection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_shadowBias(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_shadowBias", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sortMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ParticleSystemSortMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ParticleSystemSortMode = __cordl_object
            .invoke("get_sortMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sortingFudge(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_sortingFudge", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_trailMaterial(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke("get_trailMaterial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_velocityScale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_velocityScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_alignment(
        &mut self,
        value: crate::UnityEngine::ParticleSystemRenderSpace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_alignment", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_allowRoll(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_allowRoll", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_cameraVelocityScale(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_cameraVelocityScale", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_enableGPUInstancing(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enableGPUInstancing", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_flip(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_flip", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_flip_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_flip_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_freeformStretching(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_freeformStretching", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_lengthScale(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_lengthScale", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maskInteraction(
        &mut self,
        value: crate::UnityEngine::SpriteMaskInteraction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maskInteraction", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxParticleSize(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxParticleSize", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_mesh(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_mesh", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_meshDistribution(
        &mut self,
        value: crate::UnityEngine::ParticleSystemMeshDistribution,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_meshDistribution", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_minParticleSize(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_minParticleSize", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_normalDirection(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_normalDirection", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_oldTrailMaterial(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_oldTrailMaterial", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_pivot(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_pivot", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_pivot_Injected(
        &mut self,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_pivot_Injected", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_renderMode(
        &mut self,
        value: crate::UnityEngine::ParticleSystemRenderMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_renderMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rotateWithStretchDirection(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rotateWithStretchDirection", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_shadowBias(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_shadowBias", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sortMode(
        &mut self,
        value: crate::UnityEngine::ParticleSystemSortMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sortMode", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_sortingFudge(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sortingFudge", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_trailMaterial(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_trailMaterial", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_velocityScale(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_velocityScale", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ParticleSystemRenderer")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ParticleSystemRenderer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ParticleSystemRenderer+BakeTextureOutput")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParticleSystemRenderer_BakeTextureOutput {
    pub vertices: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    pub indices: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
}
#[cfg(feature = "UnityEngine+ParticleSystemRenderer+BakeTextureOutput")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ParticleSystemRenderer_BakeTextureOutput => "UnityEngine"
    ."ParticleSystemRenderer/BakeTextureOutput"
);
#[cfg(feature = "UnityEngine+ParticleSystemRenderer+BakeTextureOutput")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ParticleSystemRenderer_BakeTextureOutput {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ParticleSystemRenderer+BakeTextureOutput")]
impl crate::UnityEngine::ParticleSystemRenderer_BakeTextureOutput {}
