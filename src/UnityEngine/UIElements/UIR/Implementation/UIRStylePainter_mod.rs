#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter")]
#[repr(C)]
#[derive(Debug)]
pub struct UIRStylePainter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Owner: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::RenderChain,
    >,
    pub m_Entries: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_Entry,
        >,
    >,
    pub m_Atlas: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::AtlasBase>,
    pub m_VectorImageManager: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::VectorImageManager,
    >,
    pub m_CurrentEntry: crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_Entry,
    pub m_ClosingInfo: crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_ClosingInfo,
    pub m_MaskDepth: i32,
    pub m_StencilRef: i32,
    pub m_ClipRectID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub m_SVGBackgroundEntryIndex: i32,
    pub m_VertsPool: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::TempAllocator_1<
            crate::UnityEngine::UIElements::Vertex,
        >,
    >,
    pub m_IndicesPool: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::TempAllocator_1<u16>,
    >,
    pub m_MeshWriteDataPool: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MeshWriteData>,
        >,
    >,
    pub m_NextMeshWriteDataPoolItem: i32,
    pub m_RepeatRectUVList: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_RepeatRectUV,
            >,
        >,
    >,
    pub m_AllocRawVertsIndicesDelegate: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::AllocMeshData_MeshBuilder_Allocator,
    >,
    pub m_AllocThroughDrawMeshDelegate: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::AllocMeshData_MeshBuilder_Allocator,
    >,
    pub _meshGenerationContext_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::MeshGenerationContext,
    >,
    pub _currentElement_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::VisualElement,
    >,
    pub _totalVertices_k__BackingField: i32,
    pub _totalIndices_k__BackingField: i32,
    pub m_TextInfo: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::TextCore::Text::TextInfo,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter =>
    "UnityEngine.UIElements.UIR.Implementation"."UIRStylePainter"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter")]
impl crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter {
    #[cfg(
        feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+ClosingInfo"
    )]
    pub type ClosingInfo = crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_ClosingInfo;
    #[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+Entry")]
    pub type Entry = crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_Entry;
    #[cfg(
        feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+RepeatRectUV"
    )]
    pub type RepeatRectUV = crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_RepeatRectUV;
    pub fn AdjustSpriteWinding(
        &mut self,
        vertices: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
        >,
        indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u16>,
        > = __cordl_object.invoke("AdjustSpriteWinding", (vertices, indices))?;
        Ok(__cordl_ret.into())
    }
    pub fn AllocRawVertsIndices(
        &mut self,
        vertexCount: u32,
        indexCount: u32,
        allocatorData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::MeshBuilder_AllocMeshData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MeshWriteData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshWriteData,
        > = __cordl_object
            .invoke("AllocRawVertsIndices", (vertexCount, indexCount, allocatorData))?;
        Ok(__cordl_ret.into())
    }
    pub fn AllocThroughDrawMesh(
        &mut self,
        vertexCount: u32,
        indexCount: u32,
        allocatorData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::MeshBuilder_AllocMeshData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MeshWriteData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshWriteData,
        > = __cordl_object
            .invoke("AllocThroughDrawMesh", (vertexCount, indexCount, allocatorData))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyInset(
        &mut self,
        rectParams: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams,
        >,
        tex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyInset", (rectParams, tex))?;
        Ok(__cordl_ret.into())
    }
    pub fn ApplyVisualElementClipping(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyVisualElementClipping", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Begin(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Begin", (ve))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildEntryFromNativeMesh(
        &mut self,
        meshData: crate::UnityEngine::UIElements::MeshWriteDataInterface,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        textureId: crate::UnityEngine::UIElements::TextureId,
        isAtlas: bool,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        flags: crate::UnityEngine::UIElements::MeshGenerationContext_MeshFlags,
        uvRegion: crate::UnityEngine::Rect,
        addFlags: crate::UnityEngine::UIElements::UIR::VertexFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "BuildEntryFromNativeMesh",
                (
                    meshData,
                    texture,
                    textureId,
                    isAtlas,
                    material,
                    flags,
                    uvRegion,
                    addFlags,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildGradientEntryFromNativeMesh(
        &mut self,
        meshData: crate::UnityEngine::UIElements::MeshWriteDataInterface,
        svgTextureId: crate::UnityEngine::UIElements::TextureId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildGradientEntryFromNativeMesh", (meshData, svgTextureId))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildRawEntryFromNativeMesh(
        &mut self,
        meshData: crate::UnityEngine::UIElements::MeshWriteDataInterface,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildRawEntryFromNativeMesh", (meshData))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawBorder(
        &mut self,
        borderParams: crate::UnityEngine::UIElements::MeshGenerationContextUtils_BorderParams,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawBorder", (borderParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawImmediate(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        cullingEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawImmediate", (callback, cullingEnabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawMesh(
        &mut self,
        vertexCount: i32,
        indexCount: i32,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        flags: crate::UnityEngine::UIElements::MeshGenerationContext_MeshFlags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MeshWriteData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshWriteData,
        > = __cordl_object
            .invoke("DrawMesh", (vertexCount, indexCount, texture, material, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawRectangle(
        &mut self,
        rectParams: crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawRectangle", (rectParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawRectangleRepeat(
        &mut self,
        rectParams: crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams,
        totalRect: crate::UnityEngine::Rect,
        scaledPixelsPerPoint: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DrawRectangleRepeat",
                (rectParams, totalRect, scaledPixelsPerPoint),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawSprite(
        &mut self,
        rectParams: crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawSprite", (rectParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawText(
        &mut self,
        te: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawText", (te))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawTextInfo(
        &mut self,
        textInfo: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::TextCore::Text::TextInfo,
        >,
        offset: crate::UnityEngine::Vector2,
        useHints: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawTextInfo", (textInfo, offset, useHints))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawVectorImage(
        &mut self,
        rectParams: crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawVectorImage", (rectParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawVisualElementBackground(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawVisualElementBackground", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DrawVisualElementBorder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DrawVisualElementBorder", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateStencilClipEntryForRoundedRectBackground(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateStencilClipEntryForRoundedRectBackground", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GenerateStencilClipEntryForSVGBackground(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GenerateStencilClipEntryForSVGBackground", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPooledMeshWriteData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MeshWriteData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshWriteData,
        > = __cordl_object.invoke("GetPooledMeshWriteData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LandClipRegisterMesh(
        &mut self,
        vertices: crate::Unity::Collections::NativeSlice_1<
            crate::UnityEngine::UIElements::Vertex,
        >,
        indices: crate::Unity::Collections::NativeSlice_1<u16>,
        indexOffset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LandClipRegisterMesh", (vertices, indices, indexOffset))?;
        Ok(__cordl_ret.into())
    }
    pub fn LandClipUnregisterMeshDrawCommand(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LandClipUnregisterMeshDrawCommand", (cmd))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeVectorGraphics(
        &mut self,
        rectParams: crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams,
        isUsingGradients: bool,
        svgTexture: crate::UnityEngine::UIElements::TextureId,
        settingIndexOffset: i32,
        finalVertexCount: quest_hook::libil2cpp::ByRefMut<i32>,
        finalIndexCount: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "MakeVectorGraphics",
                (
                    rectParams,
                    isUsingGradients,
                    svgTexture,
                    settingIndexOffset,
                    finalVertexCount,
                    finalIndexCount,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (renderChain))?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StampRectangleWithSubRect(
        &mut self,
        rectParams: crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams,
        targetRect: crate::UnityEngine::Rect,
        targetUV: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StampRectangleWithSubRect", (rectParams, targetRect, targetUV))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryAtlasTexture(
        &mut self,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        flags: crate::UnityEngine::UIElements::MeshGenerationContext_MeshFlags,
        outUVRegion: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
        outIsAtlas: quest_hook::libil2cpp::ByRefMut<bool>,
        outTextureId: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::TextureId,
        >,
        outAddFlags: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::VertexFlags,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "TryAtlasTexture",
                (texture, flags, outUVRegion, outIsAtlas, outTextureId, outAddFlags),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateMeshWriteData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateMeshWriteData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (renderChain))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_closingInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_ClosingInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_ClosingInfo = __cordl_object
            .invoke("get_closingInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_currentElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("get_currentElement", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_entries(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_Entry,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_Entry,
            >,
        > = __cordl_object.invoke("get_entries", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_meshGenerationContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MeshGenerationContext>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshGenerationContext,
        > = __cordl_object.invoke("get_meshGenerationContext", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_totalIndices(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_totalIndices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_totalVertices(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_totalVertices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_visualElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = __cordl_object.invoke("get_visualElement", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_currentElement(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_currentElement", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_totalIndices(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_totalIndices", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_totalVertices(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_totalVertices", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter")]
impl AsRef<crate::UnityEngine::UIElements::IStylePainter>
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IStylePainter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter")]
impl AsMut<crate::UnityEngine::UIElements::IStylePainter>
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IStylePainter {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+ClosingInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct UIRStylePainter_ClosingInfo {
    pub needsClosing: bool,
    pub popViewMatrix: bool,
    pub popScissorClip: bool,
    pub blitAndPopRenderTexture: bool,
    pub PopDefaultMaterial: bool,
    pub clipUnregisterDrawCommand: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::RenderChainCommand,
    >,
    pub clipperRegisterVertices: crate::Unity::Collections::NativeSlice_1<
        crate::UnityEngine::UIElements::Vertex,
    >,
    pub clipperRegisterIndices: crate::Unity::Collections::NativeSlice_1<u16>,
    pub clipperRegisterIndexOffset: i32,
    pub maskStencilRef: i32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+ClosingInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_ClosingInfo =>
    "UnityEngine.UIElements.UIR.Implementation"."UIRStylePainter/ClosingInfo"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+ClosingInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_ClosingInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+ClosingInfo")]
impl crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_ClosingInfo {}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+Entry")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct UIRStylePainter_Entry {
    pub vertices: crate::Unity::Collections::NativeSlice_1<
        crate::UnityEngine::UIElements::Vertex,
    >,
    pub indices: crate::Unity::Collections::NativeSlice_1<u16>,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub fontTexSDFScale: f32,
    pub texture: crate::UnityEngine::UIElements::TextureId,
    pub customCommand: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::RenderChainCommand,
    >,
    pub clipRectID: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    pub addFlags: crate::UnityEngine::UIElements::UIR::VertexFlags,
    pub uvIsDisplacement: bool,
    pub isTextEntry: bool,
    pub isClipRegisterEntry: bool,
    pub stencilRef: i32,
    pub maskDepth: i32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+Entry")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_Entry =>
    "UnityEngine.UIElements.UIR.Implementation"."UIRStylePainter/Entry"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+Entry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_Entry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+Entry")]
impl crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_Entry {}
#[cfg(
    feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+RepeatRectUV"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct UIRStylePainter_RepeatRectUV {
    pub rect: crate::UnityEngine::Rect,
    pub uv: crate::UnityEngine::Rect,
}
#[cfg(
    feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+RepeatRectUV"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_RepeatRectUV =>
    "UnityEngine.UIElements.UIR.Implementation"."UIRStylePainter/RepeatRectUV"
);
#[cfg(
    feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+RepeatRectUV"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_RepeatRectUV {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+RepeatRectUV"
)]
impl crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_RepeatRectUV {}
