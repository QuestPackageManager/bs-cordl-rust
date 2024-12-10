#[cfg(feature = "UnityEngine+Rendering+CommandBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct CommandBuffer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Ptr: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+Rendering+CommandBuffer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::CommandBuffer =>
    "UnityEngine.Rendering"."CommandBuffer"
);
#[cfg(feature = "UnityEngine+Rendering+CommandBuffer")]
impl std::ops::Deref for crate::UnityEngine::Rendering::CommandBuffer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CommandBuffer")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::CommandBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CommandBuffer")]
impl crate::UnityEngine::Rendering::CommandBuffer {
    pub fn Blit_Identifier(
        &mut self,
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        dest: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
        scale: crate::UnityEngine::Vector2,
        offset: crate::UnityEngine::Vector2,
        sourceDepthSlice: i32,
        destDepthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Blit_Identifier",
                (
                    source,
                    dest,
                    mat,
                    pass,
                    scale,
                    offset,
                    sourceDepthSlice,
                    destDepthSlice,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Blit_Identifier_Injected(
        &mut self,
        source: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        dest: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
        scale: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        offset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        sourceDepthSlice: i32,
        destDepthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Blit_Identifier_Injected",
                (
                    source,
                    dest,
                    mat,
                    pass,
                    scale,
                    offset,
                    sourceDepthSlice,
                    destDepthSlice,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Blit_Material1(
        &mut self,
        source: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        dest: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Blit", (source, dest, mat))?;
        Ok(__cordl_ret.into())
    }
    pub fn Blit_Material_i32_2(
        &mut self,
        source: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        dest: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Blit", (source, dest, mat, pass))?;
        Ok(__cordl_ret.into())
    }
    pub fn Blit_RenderTargetIdentifier_RenderTargetIdentifier0(
        &mut self,
        source: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        dest: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Blit", (source, dest))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyTexture(
        &mut self,
        src: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        dst: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTexture", (src, dst))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyTexture_Internal(
        &mut self,
        src: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        srcElement: i32,
        srcMip: i32,
        srcX: i32,
        srcY: i32,
        srcWidth: i32,
        srcHeight: i32,
        dst: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        dstElement: i32,
        dstMip: i32,
        dstX: i32,
        dstY: i32,
        mode: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CopyTexture_Internal",
                (
                    src,
                    srcElement,
                    srcMip,
                    srcX,
                    srcY,
                    srcWidth,
                    srcHeight,
                    dst,
                    dstElement,
                    dstMip,
                    dstX,
                    dstY,
                    mode,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstanced(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        matrices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        count: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DrawMeshInstanced",
                (mesh, submeshIndex, material, shaderPass, matrices, count, properties),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawMesh_Mesh_Matrix4x4_Material3(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawMesh", (mesh, matrix, material))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawMesh_i32_2(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawMesh", (mesh, matrix, material, submeshIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawMesh_i32_i32_1(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
        shaderPass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawMesh", (mesh, matrix, material, submeshIndex, shaderPass))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawMesh_i32_i32_MaterialPropertyBlock0(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
        shaderPass: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DrawMesh",
                (mesh, matrix, material, submeshIndex, shaderPass, properties),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawRenderer(
        &mut self,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
        shaderPass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawRenderer", (renderer, material, submeshIndex, shaderPass))?;
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRT_GraphicsFormat3(
        &mut self,
        nameID: i32,
        width: i32,
        height: i32,
        depthBuffer: i32,
        filter: crate::UnityEngine::FilterMode,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetTemporaryRT",
                (nameID, width, height, depthBuffer, filter, format),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRT_GraphicsFormat_i32_2(
        &mut self,
        nameID: i32,
        width: i32,
        height: i32,
        depthBuffer: i32,
        filter: crate::UnityEngine::FilterMode,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        antiAliasing: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetTemporaryRT",
                (nameID, width, height, depthBuffer, filter, format, antiAliasing),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRT_GraphicsFormat_i32__cordl_bool_RenderTextureMemoryless1(
        &mut self,
        nameID: i32,
        width: i32,
        height: i32,
        depthBuffer: i32,
        filter: crate::UnityEngine::FilterMode,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        antiAliasing: i32,
        enableRandomWrite: bool,
        memorylessMode: crate::UnityEngine::RenderTextureMemoryless,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetTemporaryRT",
                (
                    nameID,
                    width,
                    height,
                    depthBuffer,
                    filter,
                    format,
                    antiAliasing,
                    enableRandomWrite,
                    memorylessMode,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRT_GraphicsFormat_i32__cordl_bool_RenderTextureMemoryless__cordl_bool0(
        &mut self,
        nameID: i32,
        width: i32,
        height: i32,
        depthBuffer: i32,
        filter: crate::UnityEngine::FilterMode,
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        antiAliasing: i32,
        enableRandomWrite: bool,
        memorylessMode: crate::UnityEngine::RenderTextureMemoryless,
        useDynamicScale: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetTemporaryRT",
                (
                    nameID,
                    width,
                    height,
                    depthBuffer,
                    filter,
                    format,
                    antiAliasing,
                    enableRandomWrite,
                    memorylessMode,
                    useDynamicScale,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTemporaryRT_RenderTextureFormat4(
        &mut self,
        nameID: i32,
        width: i32,
        height: i32,
        depthBuffer: i32,
        filter: crate::UnityEngine::FilterMode,
        format: crate::UnityEngine::RenderTextureFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetTemporaryRT",
                (nameID, width, height, depthBuffer, filter, format),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMesh(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
        shaderPass: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Internal_DrawMesh",
                (mesh, matrix, material, submeshIndex, shaderPass, properties),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMeshInstanced(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        shaderPass: i32,
        matrices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        count: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Internal_DrawMeshInstanced",
                (mesh, submeshIndex, material, shaderPass, matrices, count, properties),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMesh_Injected(
        &mut self,
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
        shaderPass: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Internal_DrawMesh_Injected",
                (mesh, matrix, material, submeshIndex, shaderPass, properties),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawRenderer(
        &mut self,
        renderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        submeshIndex: i32,
        shaderPass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Internal_DrawRenderer",
                (renderer, material, submeshIndex, shaderPass),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReleaseBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleaseBuffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseTemporaryRT(
        &mut self,
        nameID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleaseTemporaryRT", (nameID))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalFloat(
        &mut self,
        nameID: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGlobalFloat", (nameID, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_Il2CppString0(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGlobalTexture", (name, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_Impl(
        &mut self,
        nameID: i32,
        rt: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGlobalTexture_Impl", (nameID, rt, element))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalTexture_i32_RenderTextureSubElement1(
        &mut self,
        nameID: i32,
        value: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        element: crate::UnityEngine::Rendering::RenderTextureSubElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGlobalTexture", (nameID, value, element))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVector(
        &mut self,
        nameID: i32,
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGlobalVector", (nameID, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGlobalVector_Injected(
        &mut self,
        nameID: i32,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGlobalVector_Injected", (nameID, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget(
        &mut self,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRenderTarget", (rt))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTargetSingle_Internal(
        &mut self,
        rt: crate::UnityEngine::Rendering::RenderTargetIdentifier,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetRenderTargetSingle_Internal",
                (
                    rt,
                    colorLoadAction,
                    colorStoreAction,
                    depthLoadAction,
                    depthStoreAction,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTargetSingle_Internal_Injected(
        &mut self,
        rt: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderTargetIdentifier,
        >,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        depthLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetRenderTargetSingle_Internal_Injected",
                (
                    rt,
                    colorLoadAction,
                    colorStoreAction,
                    depthLoadAction,
                    depthStoreAction,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetViewProjectionMatrices(
        &mut self,
        view: crate::UnityEngine::Matrix4x4,
        proj: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetViewProjectionMatrices", (view, proj))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetViewProjectionMatrices_Injected(
        &mut self,
        view: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        proj: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetViewProjectionMatrices_Injected", (view, proj))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateAgainstExecutionFlags(
        &mut self,
        requiredFlags: crate::UnityEngine::Rendering::CommandBufferExecutionFlags,
        invalidFlags: crate::UnityEngine::Rendering::CommandBufferExecutionFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ValidateAgainstExecutionFlags", (requiredFlags, invalidFlags))?;
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
    pub fn set_name(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_name", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+CommandBuffer")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rendering::CommandBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
