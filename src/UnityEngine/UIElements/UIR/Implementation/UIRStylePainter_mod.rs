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
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_RepeatRectUV,
                >,
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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR.Implementation";
    const CLASS_NAME: &'static str = "UIRStylePainter";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u16>>,
                2usize,
            >("AdjustSpriteWinding")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "AdjustSpriteWinding",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u16>,
        > = unsafe { method.invoke_unchecked(self, (vertices, indices))? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::UIR::MeshBuilder_AllocMeshData,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MeshWriteData>,
                3usize,
            >("AllocRawVertsIndices")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "AllocRawVertsIndices",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshWriteData,
        > = unsafe {
            method.invoke_unchecked(self, (vertexCount, indexCount, allocatorData))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::UIR::MeshBuilder_AllocMeshData,
                    >,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MeshWriteData>,
                3usize,
            >("AllocThroughDrawMesh")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "AllocThroughDrawMesh",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshWriteData,
        > = unsafe {
            method.invoke_unchecked(self, (vertexCount, indexCount, allocatorData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyInset(
        &mut self,
        rectParams: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams,
        >,
        tex: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::MeshBuilderNative_NativeRectParams,
                    >,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ApplyInset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "ApplyInset", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (rectParams, tex))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ApplyVisualElementClipping(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ApplyVisualElementClipping")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(),
                    "ApplyVisualElementClipping", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Begin(
        &mut self,
        ve: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElement,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Begin")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "Begin", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ve))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::UIElements::MeshWriteDataInterface,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                    crate::UnityEngine::UIElements::TextureId,
                    bool,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    crate::UnityEngine::UIElements::MeshGenerationContext_MeshFlags,
                    crate::UnityEngine::Rect,
                    crate::UnityEngine::UIElements::UIR::VertexFlags,
                ),
                quest_hook::libil2cpp::Void,
                8usize,
            >("BuildEntryFromNativeMesh")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(),
                    "BuildEntryFromNativeMesh", 8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
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
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BuildGradientEntryFromNativeMesh(
        &mut self,
        meshData: crate::UnityEngine::UIElements::MeshWriteDataInterface,
        svgTextureId: crate::UnityEngine::UIElements::TextureId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::UIElements::MeshWriteDataInterface,
                    crate::UnityEngine::UIElements::TextureId,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("BuildGradientEntryFromNativeMesh")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(),
                    "BuildGradientEntryFromNativeMesh", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (meshData, svgTextureId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BuildRawEntryFromNativeMesh(
        &mut self,
        meshData: crate::UnityEngine::UIElements::MeshWriteDataInterface,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::MeshWriteDataInterface),
                quest_hook::libil2cpp::Void,
                1usize,
            >("BuildRawEntryFromNativeMesh")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(),
                    "BuildRawEntryFromNativeMesh", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (meshData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawBorder(
        &mut self,
        borderParams: crate::UnityEngine::UIElements::MeshGenerationContextUtils_BorderParams,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::MeshGenerationContextUtils_BorderParams),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DrawBorder")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "DrawBorder", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (borderParams))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawImmediate(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action>,
        cullingEnabled: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("DrawImmediate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "DrawImmediate", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (callback, cullingEnabled))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                    crate::UnityEngine::UIElements::MeshGenerationContext_MeshFlags,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MeshWriteData>,
                5usize,
            >("DrawMesh")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "DrawMesh", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshWriteData,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (vertexCount, indexCount, texture, material, flags),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawRectangle(
        &mut self,
        rectParams: crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DrawRectangle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "DrawRectangle", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (rectParams))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawRectangleRepeat(
        &mut self,
        rectParams: crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams,
        totalRect: crate::UnityEngine::Rect,
        scaledPixelsPerPoint: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams,
                    crate::UnityEngine::Rect,
                    f32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("DrawRectangleRepeat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "DrawRectangleRepeat",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (rectParams, totalRect, scaledPixelsPerPoint))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawSprite(
        &mut self,
        rectParams: crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DrawSprite")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "DrawSprite", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (rectParams))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawText(
        &mut self,
        te: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TextElement>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DrawText")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "DrawText", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (te))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::TextCore::Text::TextInfo,
                    >,
                    crate::UnityEngine::Vector2,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("DrawTextInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "DrawTextInfo", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (textInfo, offset, useHints))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawVectorImage(
        &mut self,
        rectParams: crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DrawVectorImage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "DrawVectorImage", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (rectParams))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawVisualElementBackground(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("DrawVisualElementBackground")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(),
                    "DrawVisualElementBackground", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawVisualElementBorder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("DrawVisualElementBorder")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(),
                    "DrawVisualElementBorder", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateStencilClipEntryForRoundedRectBackground(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("GenerateStencilClipEntryForRoundedRectBackground")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(),
                    "GenerateStencilClipEntryForRoundedRectBackground", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateStencilClipEntryForSVGBackground(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("GenerateStencilClipEntryForSVGBackground")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(),
                    "GenerateStencilClipEntryForSVGBackground", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPooledMeshWriteData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MeshWriteData>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MeshWriteData>,
                0usize,
            >("GetPooledMeshWriteData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "GetPooledMeshWriteData",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshWriteData,
        > = unsafe { method.invoke_unchecked(self, ())? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::Unity::Collections::NativeSlice_1<
                        crate::UnityEngine::UIElements::Vertex,
                    >,
                    crate::Unity::Collections::NativeSlice_1<u16>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("LandClipRegisterMesh")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "LandClipRegisterMesh",
                    3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (vertices, indices, indexOffset))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LandClipUnregisterMeshDrawCommand(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChainCommand,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::RenderChainCommand,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("LandClipUnregisterMeshDrawCommand")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(),
                    "LandClipUnregisterMeshDrawCommand", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (cmd))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams,
                    bool,
                    crate::UnityEngine::UIElements::TextureId,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("MakeVectorGraphics")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "MakeVectorGraphics",
                    6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        rectParams,
                        isUsingGradients,
                        svgTexture,
                        settingIndexOffset,
                        finalVertexCount,
                        finalIndexCount,
                    ),
                )?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "Reset", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StampRectangleWithSubRect(
        &mut self,
        rectParams: crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams,
        targetRect: crate::UnityEngine::Rect,
        targetUV: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::UIElements::MeshGenerationContextUtils_RectangleParams,
                    crate::UnityEngine::Rect,
                    crate::UnityEngine::Rect,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("StampRectangleWithSubRect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(),
                    "StampRectangleWithSubRect", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (rectParams, targetRect, targetUV))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
                    crate::UnityEngine::UIElements::MeshGenerationContext_MeshFlags,
                    quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rect>,
                    quest_hook::libil2cpp::ByRefMut<bool>,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::TextureId,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::UIElements::UIR::VertexFlags,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("TryAtlasTexture")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "TryAtlasTexture", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (texture, flags, outUVRegion, outIsAtlas, outTextureId, outAddFlags),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ValidateMeshWriteData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("ValidateMeshWriteData")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "ValidateMeshWriteData",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        renderChain: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::RenderChain,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::UIR::RenderChain,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (renderChain))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_closingInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_ClosingInfo,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_ClosingInfo,
                0usize,
            >("get_closingInfo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "get_closingInfo", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_ClosingInfo = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_currentElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                0usize,
            >("get_currentElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "get_currentElement",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = unsafe { method.invoke_unchecked(self, ())? };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::List_1<
                        crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_Entry,
                    >,
                >,
                0usize,
            >("get_entries")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "get_entries", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_Entry,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_meshGenerationContext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MeshGenerationContext>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::MeshGenerationContext,
                >,
                0usize,
            >("get_meshGenerationContext")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(),
                    "get_meshGenerationContext", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshGenerationContext,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_totalIndices(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_totalIndices")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "get_totalIndices",
                    0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_totalVertices(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_totalVertices")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "get_totalVertices",
                    0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_visualElement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
                0usize,
            >("get_visualElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "get_visualElement",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::VisualElement,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_currentElement(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VisualElement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::VisualElement,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_currentElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "set_currentElement",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_totalIndices(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_totalIndices")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "set_totalIndices",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_totalVertices(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_totalVertices")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter
                    as quest_hook::libil2cpp::Type > ::class(), "set_totalVertices",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
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
#[derive(Debug, Clone, Default, PartialEq)]
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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_ClosingInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR.Implementation";
    const CLASS_NAME: &'static str = "UIRStylePainter/ClosingInfo";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+ClosingInfo")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_ClosingInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+ClosingInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_ClosingInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+ClosingInfo")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_ClosingInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+ClosingInfo")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_ClosingInfo {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
#[derive(Debug, Clone, Default, PartialEq)]
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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_Entry {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR.Implementation";
    const CLASS_NAME: &'static str = "UIRStylePainter/Entry";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+Entry")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_Entry {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+Entry")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_Entry {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+Entry")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_Entry {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+Entry")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_Entry {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UIRStylePainter_RepeatRectUV {
    pub rect: crate::UnityEngine::Rect,
    pub uv: crate::UnityEngine::Rect,
}
#[cfg(
    feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+RepeatRectUV"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_RepeatRectUV {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR.Implementation";
    const CLASS_NAME: &'static str = "UIRStylePainter/RepeatRectUV";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+RepeatRectUV"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_RepeatRectUV {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+RepeatRectUV"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_RepeatRectUV {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+RepeatRectUV"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_RepeatRectUV {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(
    feature = "UnityEngine+UIElements+UIR+Implementation+UIRStylePainter+RepeatRectUV"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::UIR::Implementation::UIRStylePainter_RepeatRectUV {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
