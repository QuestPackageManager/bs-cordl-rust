#[cfg(feature = "UnityEngine+Graphics")]
#[repr(C)]
#[derive(Debug)]
pub struct Graphics {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Graphics")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Graphics => "UnityEngine"
    ."Graphics"
);
#[cfg(feature = "UnityEngine+Graphics")]
impl std::ops::Deref for crate::UnityEngine::Graphics {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Graphics")]
impl std::ops::DerefMut for crate::UnityEngine::Graphics {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Graphics")]
impl crate::UnityEngine::Graphics {
    pub fn Blit2(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Blit2", (source, dest))?;
        Ok(__cordl_ret.into())
    }
    pub fn Blit4(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        scale: crate::UnityEngine::Vector2,
        offset: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Blit4", (source, dest, scale, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn Blit4_Injected(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        scale: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
        offset: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Blit4_Injected", (source, dest, scale, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn Blit_Gc3(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Blit", (source, dest, mat))?;
        Ok(__cordl_ret.into())
    }
    pub fn Blit_Gc_Gc0(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Blit", (source, dest))?;
        Ok(__cordl_ret.into())
    }
    pub fn Blit_Gc_i32_2(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Blit", (source, dest, mat, pass))?;
        Ok(__cordl_ret.into())
    }
    pub fn Blit_Vector2_Vector2_1(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        scale: crate::UnityEngine::Vector2,
        offset: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Blit", (source, dest, scale, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyTexture_Full(
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyTexture_Full", (src, dst))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyTexture_Gc0(
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyTexture", (src, dst))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyTexture_Slice(
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        srcElement: i32,
        srcMip: i32,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dstElement: i32,
        dstMip: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CopyTexture_Slice",
                (src, srcElement, srcMip, dst, dstElement, dstMip),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyTexture_i32_i32_Gc_i32_i32_1(
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        srcElement: i32,
        srcMip: i32,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dstElement: i32,
        dstMip: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CopyTexture", (src, srcElement, srcMip, dst, dstElement, dstMip))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstanced_Gc_LightProbeUsage_Gc0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        matrices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        count: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        castShadows: crate::UnityEngine::Rendering::ShadowCastingMode,
        receiveShadows: bool,
        layer: i32,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        lightProbeUsage: crate::UnityEngine::Rendering::LightProbeUsage,
        lightProbeProxyVolume: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::LightProbeProxyVolume,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DrawMeshInstanced",
                (
                    mesh,
                    submeshIndex,
                    material,
                    matrices,
                    count,
                    properties,
                    castShadows,
                    receiveShadows,
                    layer,
                    camera,
                    lightProbeUsage,
                    lightProbeProxyVolume,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshInstanced_Gc_i32_Gc_Gc_i32_Gc_ShadowCastingMode__cordl_bool_i32_1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        matrices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        count: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        castShadows: crate::UnityEngine::Rendering::ShadowCastingMode,
        receiveShadows: bool,
        layer: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DrawMeshInstanced",
                (
                    mesh,
                    submeshIndex,
                    material,
                    matrices,
                    count,
                    properties,
                    castShadows,
                    receiveShadows,
                    layer,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawMeshNow(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        materialIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DrawMeshNow", (mesh, matrix, materialIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawMesh_ShadowCastingMode_Gc_LightProbeUsage_Gc1(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        layer: i32,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        submeshIndex: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        castShadows: crate::UnityEngine::Rendering::ShadowCastingMode,
        receiveShadows: bool,
        probeAnchor: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        lightProbeUsage: crate::UnityEngine::Rendering::LightProbeUsage,
        lightProbeProxyVolume: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::LightProbeProxyVolume,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DrawMesh",
                (
                    mesh,
                    matrix,
                    material,
                    layer,
                    camera,
                    submeshIndex,
                    properties,
                    castShadows,
                    receiveShadows,
                    probeAnchor,
                    lightProbeUsage,
                    lightProbeProxyVolume,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawMesh__cordl_bool__cordl_bool0(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        layer: i32,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        submeshIndex: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        castShadows: bool,
        receiveShadows: bool,
        useLightProbes: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DrawMesh",
                (
                    mesh,
                    matrix,
                    material,
                    layer,
                    camera,
                    submeshIndex,
                    properties,
                    castShadows,
                    receiveShadows,
                    useLightProbes,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteCommandBuffer(
        buffer: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExecuteCommandBuffer", (buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_BlitMaterial5(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        pass: i32,
        setRT: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_BlitMaterial5", (source, dest, mat, pass, setRT))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMesh(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        matrix: crate::UnityEngine::Matrix4x4,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        layer: i32,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        castShadows: crate::UnityEngine::Rendering::ShadowCastingMode,
        receiveShadows: bool,
        probeAnchor: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        lightProbeUsage: crate::UnityEngine::Rendering::LightProbeUsage,
        lightProbeProxyVolume: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::LightProbeProxyVolume,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_DrawMesh",
                (
                    mesh,
                    submeshIndex,
                    matrix,
                    material,
                    layer,
                    camera,
                    properties,
                    castShadows,
                    receiveShadows,
                    probeAnchor,
                    lightProbeUsage,
                    lightProbeProxyVolume,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMeshInstanced(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        matrices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Matrix4x4>,
        >,
        count: i32,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        castShadows: crate::UnityEngine::Rendering::ShadowCastingMode,
        receiveShadows: bool,
        layer: i32,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        lightProbeUsage: crate::UnityEngine::Rendering::LightProbeUsage,
        lightProbeProxyVolume: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::LightProbeProxyVolume,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_DrawMeshInstanced",
                (
                    mesh,
                    submeshIndex,
                    material,
                    matrices,
                    count,
                    properties,
                    castShadows,
                    receiveShadows,
                    layer,
                    camera,
                    lightProbeUsage,
                    lightProbeProxyVolume,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMeshNow2(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        subsetIndex: i32,
        matrix: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_DrawMeshNow2", (mesh, subsetIndex, matrix))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMeshNow2_Injected(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        subsetIndex: i32,
        matrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_DrawMeshNow2_Injected", (mesh, subsetIndex, matrix))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_DrawMesh_Injected(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        submeshIndex: i32,
        matrix: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        layer: i32,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        castShadows: crate::UnityEngine::Rendering::ShadowCastingMode,
        receiveShadows: bool,
        probeAnchor: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        lightProbeUsage: crate::UnityEngine::Rendering::LightProbeUsage,
        lightProbeProxyVolume: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::LightProbeProxyVolume,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_DrawMesh_Injected",
                (
                    mesh,
                    submeshIndex,
                    matrix,
                    material,
                    layer,
                    camera,
                    properties,
                    castShadows,
                    receiveShadows,
                    probeAnchor,
                    lightProbeUsage,
                    lightProbeProxyVolume,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_GetMaxDrawMeshInstanceCount() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_GetMaxDrawMeshInstanceCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetNullRT() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_SetNullRT", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRTSimple(
        color: crate::UnityEngine::RenderBuffer,
        depth: crate::UnityEngine::RenderBuffer,
        mip: i32,
        face: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_SetRTSimple", (color, depth, mip, face, depthSlice))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_SetRTSimple_Injected(
        color: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderBuffer>,
        depth: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderBuffer>,
        mip: i32,
        face: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Internal_SetRTSimple_Injected",
                (color, depth, mip, face, depthSlice),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTargetImpl_Gc_i32_CubemapFace_i32_1(
        rt: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        mipLevel: i32,
        face: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetRenderTargetImpl", (rt, mipLevel, face, depthSlice))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTargetImpl_RenderBuffer_RenderBuffer_i32_CubemapFace_i32_0(
        colorBuffer: crate::UnityEngine::RenderBuffer,
        depthBuffer: crate::UnityEngine::RenderBuffer,
        mipLevel: i32,
        face: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetRenderTargetImpl",
                (colorBuffer, depthBuffer, mipLevel, face, depthSlice),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_Gc1(
        rt: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetRenderTarget", (rt))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_i32_CubemapFace2(
        rt: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        mipLevel: i32,
        face: crate::UnityEngine::CubemapFace,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetRenderTarget", (rt, mipLevel, face))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRenderTarget_i32_CubemapFace_i32_0(
        rt: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        mipLevel: i32,
        face: crate::UnityEngine::CubemapFace,
        depthSlice: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetRenderTarget", (rt, mipLevel, face, depthSlice))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_activeTier() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::GraphicsTier,
    > {
        let __cordl_ret: crate::UnityEngine::Rendering::GraphicsTier = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_activeTier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_activeTier(
        value: crate::UnityEngine::Rendering::GraphicsTier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_activeTier", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Graphics")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Graphics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
