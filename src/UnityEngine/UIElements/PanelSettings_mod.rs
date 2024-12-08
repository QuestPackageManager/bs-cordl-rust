#[cfg(feature = "UnityEngine+UIElements+PanelSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct PanelSettings {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub themeUss: *mut crate::UnityEngine::UIElements::ThemeStyleSheet,
    pub m_TargetTexture: *mut crate::UnityEngine::RenderTexture,
    pub m_ScaleMode: crate::UnityEngine::UIElements::PanelScaleMode,
    pub m_ReferenceSpritePixelsPerUnit: f32,
    pub m_Scale: f32,
    pub m_ReferenceDpi: f32,
    pub m_FallbackDpi: f32,
    pub m_ReferenceResolution: crate::UnityEngine::Vector2Int,
    pub m_ScreenMatchMode: crate::UnityEngine::UIElements::PanelScreenMatchMode,
    pub m_Match: f32,
    pub m_SortingOrder: f32,
    pub m_TargetDisplay: i32,
    pub m_ClearDepthStencil: bool,
    pub m_ClearColor: bool,
    pub m_ColorClearValue: crate::UnityEngine::Color,
    pub m_PanelAccess: *mut crate::UnityEngine::UIElements::PanelSettings_RuntimePanelAccess,
    pub m_AttachedUIDocumentsList: *mut crate::UnityEngine::UIElements::UIDocumentList,
    pub m_DynamicAtlasSettings: *mut crate::UnityEngine::UIElements::DynamicAtlasSettings,
    pub m_AtlasBlitShader: *mut crate::UnityEngine::Shader,
    pub m_RuntimeShader: *mut crate::UnityEngine::Shader,
    pub m_RuntimeWorldShader: *mut crate::UnityEngine::Shader,
    pub textSettings: *mut crate::UnityEngine::UIElements::PanelTextSettings,
    pub m_TargetRect: crate::UnityEngine::Rect,
    pub m_ResolvedScale: f32,
    pub m_OldThemeUss: *mut crate::UnityEngine::UIElements::StyleSheet,
    pub _ScreenDPI_k__BackingField: f32,
    pub m_AssignedScreenToPanel: *mut crate::System::Func_2<
        crate::UnityEngine::Vector2,
        crate::UnityEngine::Vector2,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+PanelSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::PanelSettings =>
    "UnityEngine.UIElements"."PanelSettings"
);
#[cfg(feature = "UnityEngine+UIElements+PanelSettings")]
impl std::ops::Deref for crate::UnityEngine::UIElements::PanelSettings {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PanelSettings")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::PanelSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PanelSettings")]
impl crate::UnityEngine::UIElements::PanelSettings {
    pub const DefaultDpi: f32 = 96f32;
    pub const k_DefaultScaleValue: f32 = 1f32;
    pub const k_DefaultSortingOrder: i32 = 0i32;
    pub const k_DefaultStyleSheetPath: &'static str = "Packages/com.unity.ui/PackageResources/StyleSheets/Generated/Default.tss.asset";
    #[cfg(feature = "UnityEngine+UIElements+PanelSettings+RuntimePanelAccess")]
    pub type RuntimePanelAccess = crate::UnityEngine::UIElements::PanelSettings_RuntimePanelAccess;
    pub fn ApplyPanelSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyPanelSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn ApplySortingOrder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplySortingOrder", ())?;
        Ok(__cordl_ret)
    }
    pub fn ApplyThemeStyleSheet(
        &mut self,
        root: *mut crate::UnityEngine::UIElements::VisualElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyThemeStyleSheet", (root))?;
        Ok(__cordl_ret)
    }
    pub fn AttachAndInsertUIDocumentToVisualTree(
        &mut self,
        uiDocument: *mut crate::UnityEngine::UIElements::UIDocument,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AttachAndInsertUIDocumentToVisualTree", (uiDocument))?;
        Ok(__cordl_ret)
    }
    pub fn DetachUIDocument(
        &mut self,
        uiDocument: *mut crate::UnityEngine::UIElements::UIDocument,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DetachUIDocument", (uiDocument))?;
        Ok(__cordl_ret)
    }
    pub fn DisposePanel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisposePanel", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDisplayRect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("GetDisplayRect", ())?;
        Ok(__cordl_ret)
    }
    pub fn InitializeShaders(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeShaders", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn ResolveScale(
        &mut self,
        targetRect: crate::UnityEngine::Rect,
        screenDpi: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("ResolveScale", (targetRect, screenDpi))?;
        Ok(__cordl_ret)
    }
    pub fn SetScreenToPanelSpaceFunction(
        &mut self,
        screentoPanelSpaceFunction: *mut crate::System::Func_2<
            crate::UnityEngine::Vector2,
            crate::UnityEngine::Vector2,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetScreenToPanelSpaceFunction", (screentoPanelSpaceFunction))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateScreenDPI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateScreenDPI", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ScreenDPI(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_ScreenDPI", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_clearColor(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_clearColor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_clearDepthStencil(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_clearDepthStencil", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_colorClearValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_colorClearValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_depthClearValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_depthClearValue", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_dynamicAtlasSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::DynamicAtlasSettings,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::DynamicAtlasSettings = __cordl_object
            .invoke("get_dynamicAtlasSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_fallbackDpi(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_fallbackDpi", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isInitialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isInitialized", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_match(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_match", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_panel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::BaseRuntimePanel,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::BaseRuntimePanel = __cordl_object
            .invoke("get_panel", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_referenceDpi(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_referenceDpi", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_referenceResolution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2Int> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2Int = __cordl_object
            .invoke("get_referenceResolution", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_referenceSpritePixelsPerUnit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_referenceSpritePixelsPerUnit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_scale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_scale", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_scaleMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::PanelScaleMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::PanelScaleMode = __cordl_object
            .invoke("get_scaleMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_screenMatchMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::PanelScreenMatchMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::PanelScreenMatchMode = __cordl_object
            .invoke("get_screenMatchMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_sortingOrder(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_sortingOrder", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_targetDisplay(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_targetDisplay", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_targetTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::RenderTexture> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::RenderTexture = __cordl_object
            .invoke("get_targetTexture", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_themeStyleSheet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::ThemeStyleSheet,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::ThemeStyleSheet = __cordl_object
            .invoke("get_themeStyleSheet", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_visualTree(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::VisualElement,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::VisualElement = __cordl_object
            .invoke("get_visualTree", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ScreenDPI(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ScreenDPI", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_clearColor(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_clearColor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_clearDepthStencil(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_clearDepthStencil", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_colorClearValue(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_colorClearValue", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_dynamicAtlasSettings(
        &mut self,
        value: *mut crate::UnityEngine::UIElements::DynamicAtlasSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_dynamicAtlasSettings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_fallbackDpi(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fallbackDpi", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_match(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_match", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_referenceDpi(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_referenceDpi", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_referenceResolution(
        &mut self,
        value: crate::UnityEngine::Vector2Int,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_referenceResolution", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_referenceSpritePixelsPerUnit(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_referenceSpritePixelsPerUnit", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_scale(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_scale", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_scaleMode(
        &mut self,
        value: crate::UnityEngine::UIElements::PanelScaleMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_scaleMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_screenMatchMode(
        &mut self,
        value: crate::UnityEngine::UIElements::PanelScreenMatchMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_screenMatchMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_sortingOrder(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sortingOrder", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_targetDisplay(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_targetDisplay", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_targetTexture(
        &mut self,
        value: *mut crate::UnityEngine::RenderTexture,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_targetTexture", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_themeStyleSheet(
        &mut self,
        value: *mut crate::UnityEngine::UIElements::ThemeStyleSheet,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_themeStyleSheet", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+PanelSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::PanelSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+PanelSettings+RuntimePanelAccess")]
#[repr(C)]
#[derive(Debug)]
pub struct PanelSettings_RuntimePanelAccess {
    __cordl_parent: crate::System::Object,
    pub m_Settings: *mut crate::UnityEngine::UIElements::PanelSettings,
    pub m_RuntimePanel: *mut crate::UnityEngine::UIElements::BaseRuntimePanel,
}
#[cfg(feature = "UnityEngine+UIElements+PanelSettings+RuntimePanelAccess")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::PanelSettings_RuntimePanelAccess =>
    "UnityEngine.UIElements"."PanelSettings/RuntimePanelAccess"
);
#[cfg(feature = "UnityEngine+UIElements+PanelSettings+RuntimePanelAccess")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::PanelSettings_RuntimePanelAccess {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PanelSettings+RuntimePanelAccess")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::PanelSettings_RuntimePanelAccess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+PanelSettings+RuntimePanelAccess")]
impl crate::UnityEngine::UIElements::PanelSettings_RuntimePanelAccess {
    pub fn CreateRelatedRuntimePanel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::BaseRuntimePanel,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::BaseRuntimePanel = __cordl_object
            .invoke("CreateRelatedRuntimePanel", ())?;
        Ok(__cordl_ret)
    }
    pub fn DisposePanel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisposePanel", ())?;
        Ok(__cordl_ret)
    }
    pub fn DisposeRelatedPanel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisposeRelatedPanel", ())?;
        Ok(__cordl_ret)
    }
    pub fn MarkPotentiallyEmpty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkPotentiallyEmpty", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        settings: *mut crate::UnityEngine::UIElements::PanelSettings,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (settings))?;
        Ok(__cordl_object)
    }
    pub fn SetSortingPriority(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSortingPriority", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetTargetDisplay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTargetDisplay", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetTargetTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTargetTexture", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        settings: *mut crate::UnityEngine::UIElements::PanelSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (settings))?;
        Ok(__cordl_ret)
    }
    pub fn get_isInitialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isInitialized", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_panel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::BaseRuntimePanel,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::BaseRuntimePanel = __cordl_object
            .invoke("get_panel", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+PanelSettings+RuntimePanelAccess")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::PanelSettings_RuntimePanelAccess {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
