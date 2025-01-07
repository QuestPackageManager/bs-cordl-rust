#[cfg(feature = "UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UIRVEShaderInfoAllocator {
    pub m_Storage: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::BaseShaderInfoStorage,
    >,
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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "UIRVEShaderInfoAllocator";
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
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator {
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
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator {
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
#[cfg(feature = "UnityEngine+UIElements+UIR+UIRVEShaderInfoAllocator")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::UIR::UIRVEShaderInfoAllocator {
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
        Ok(__cordl_ret.into())
    }
    pub fn AllocColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::BMPAlloc> {
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::BMPAlloc = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AllocColor",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AllocOpacity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::BMPAlloc> {
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::BMPAlloc = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AllocOpacity",
            (),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn AllocToConstantBufferIndex(
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllocToConstantBufferIndex", (alloc))?;
        Ok(__cordl_ret.into())
    }
    pub fn AllocToTexelCoord(
        allocator: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
        >,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2Int> {
        let __cordl_ret: crate::UnityEngine::Vector2Int = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllocToTexelCoord", (allocator, alloc))?;
        Ok(__cordl_ret.into())
    }
    pub fn AllocTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::BMPAlloc> {
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::BMPAlloc = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AllocTransform",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AtlasRectMatchesPage(
        allocator: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::BitmapAllocator32,
        >,
        defAlloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
        atlasRect: crate::UnityEngine::RectInt,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AtlasRectMatchesPage", (allocator, defAlloc, atlasRect))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Construct(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Construct",
            (),
        )?;
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
    pub fn FreeClipRect(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "FreeClipRect",
            (alloc),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn IssuePendingStorageChanges(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IssuePendingStorageChanges",
            (),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn ReallyCreateStorage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ReallyCreateStorage",
            (),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_atlas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_atlas",
            (),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_pageHeight() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_pageHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_pageWidth() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_pageWidth", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
