#[cfg(feature = "UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct UIRVEShaderInfoAllocator {
    pub m_Storage: *mut crate::UnityEngine::UIElements::UIR::BaseShaderInfoStorage,
    pub m_TransformAllocator: crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
    pub m_ClipRectAllocator: crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
    pub m_OpacityAllocator: crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
    pub m_ColorAllocator: crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
    pub m_TextSettingsAllocator: crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
    pub m_StorageReallyCreated: bool,
    pub m_VertexTexturingEnabled: bool,
    pub m_Transforms: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::UIElements::UIR::Transform3x4,
    >,
    pub m_ClipRects: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Vector4,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator =>
    "UnityEngine.UIElements.UIR"."UIRVEShaderInfoAllocator"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
impl crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator {
    pub fn AllocClipRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::BMPAlloc> {
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::BMPAlloc = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AllocClipRect",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AllocColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::BMPAlloc> {
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::BMPAlloc = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AllocColor",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AllocOpacity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::BMPAlloc> {
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::BMPAlloc = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AllocOpacity",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AllocTextCoreSettings(
        &mut self,
        settings: crate::UnityEngine::UIElements::UIR::TextCoreSettings,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::BMPAlloc> {
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::BMPAlloc = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AllocTextCoreSettings",
            (settings),
        )?;
        Ok(__cordl_ret)
    }
    pub fn AllocTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::BMPAlloc> {
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::BMPAlloc = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AllocTransform",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ClipRectAllocToVertexData(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_ret: crate::UnityEngine::Color32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ClipRectAllocToVertexData",
            (alloc),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ColorAllocToVertexData(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_ret: crate::UnityEngine::Color32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ColorAllocToVertexData",
            (alloc),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Construct(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Construct",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn FreeClipRect(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FreeClipRect",
            (alloc),
        )?;
        Ok(__cordl_ret)
    }
    pub fn FreeColor(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FreeColor",
            (alloc),
        )?;
        Ok(__cordl_ret)
    }
    pub fn FreeOpacity(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FreeOpacity",
            (alloc),
        )?;
        Ok(__cordl_ret)
    }
    pub fn FreeTextCoreSettings(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FreeTextCoreSettings",
            (alloc),
        )?;
        Ok(__cordl_ret)
    }
    pub fn FreeTransform(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FreeTransform",
            (alloc),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IssuePendingStorageChanges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IssuePendingStorageChanges",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn OpacityAllocToVertexData(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_ret: crate::UnityEngine::Color32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OpacityAllocToVertexData",
            (alloc),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ReallyCreateStorage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReallyCreateStorage",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetClipRectValue(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
        clipRect: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetClipRectValue",
            (alloc, clipRect),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetColorValue(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
        color: crate::UnityEngine::Color,
        isEditorContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetColorValue",
            (alloc, color, isEditorContext),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetOpacityValue(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
        opacity: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetOpacityValue",
            (alloc, opacity),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetTextCoreSettingValue(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
        settings: crate::UnityEngine::UIElements::UIR::TextCoreSettings,
        isEditorContext: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetTextCoreSettingValue",
            (alloc, settings, isEditorContext),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetTransformValue(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
        xform: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetTransformValue",
            (alloc, xform),
        )?;
        Ok(__cordl_ret)
    }
    pub fn TextCoreSettingsToVertexData(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_ret: crate::UnityEngine::Color32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TextCoreSettingsToVertexData",
            (alloc),
        )?;
        Ok(__cordl_ret)
    }
    pub fn TransformAllocToVertexData(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color32> {
        let __cordl_ret: crate::UnityEngine::Color32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TransformAllocToVertexData",
            (alloc),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_atlas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture> {
        let __cordl_ret: *mut crate::UnityEngine::Texture = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_atlas",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_clipRectConstants(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::NativeSlice_1<crate::UnityEngine::Vector4>,
    > {
        let __cordl_ret: crate::Unity::Collections::NativeSlice_1<
            crate::UnityEngine::Vector4,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_clipRectConstants",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_transformConstants(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Collections::NativeSlice_1<
            crate::UnityEngine::UIElements::UIR::Transform3x4,
        >,
    > {
        let __cordl_ret: crate::Unity::Collections::NativeSlice_1<
            crate::UnityEngine::UIElements::UIR::Transform3x4,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_transformConstants",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
