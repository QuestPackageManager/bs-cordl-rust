#[cfg(feature = "OVROverlay")]
#[repr(C)]
#[derive(Debug)]
pub struct OVROverlay {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub currentOverlayType: crate::GlobalNamespace::OVROverlay_OverlayType,
    pub isDynamic: bool,
    pub isProtectedContent: bool,
    pub srcRectLeft: crate::UnityEngine::Rect,
    pub srcRectRight: crate::UnityEngine::Rect,
    pub destRectLeft: crate::UnityEngine::Rect,
    pub destRectRight: crate::UnityEngine::Rect,
    pub invertTextureRects: bool,
    pub textureRectMatrix: crate::GlobalNamespace::OVRPlugin_TextureRectMatrixf,
    pub overrideTextureRectMatrix: bool,
    pub overridePerLayerColorScaleAndOffset: bool,
    pub colorScale: crate::UnityEngine::Vector4,
    pub colorOffset: crate::UnityEngine::Vector4,
    pub useExpensiveSuperSample: bool,
    pub useExpensiveSharpen: bool,
    pub hidden: bool,
    pub isExternalSurface: bool,
    pub externalSurfaceWidth: i32,
    pub externalSurfaceHeight: i32,
    pub compositionDepth: i32,
    pub layerCompositionDepth: i32,
    pub noDepthBufferTesting: bool,
    pub layerTextureFormat: crate::GlobalNamespace::OVRPlugin_EyeTextureFormat,
    pub currentOverlayShape: crate::GlobalNamespace::OVROverlay_OverlayShape,
    pub prevOverlayShape: crate::GlobalNamespace::OVROverlay_OverlayShape,
    pub textures: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        >,
    >,
    pub isAlphaPremultiplied: bool,
    pub useBicubicFiltering: bool,
    pub useLegacyCubemapRotation: bool,
    pub useEfficientSupersample: bool,
    pub useEfficientSharpen: bool,
    pub useAutomaticFiltering: bool,
    pub _previewInEditor: bool,
    pub texturePtrs: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
    >,
    pub externalSurfaceObject: crate::System::IntPtr,
    pub externalSurfaceObjectCreated: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVROverlay_ExternalSurfaceObjectCreated,
    >,
    pub isOverridePending: bool,
    pub _layerId_k__BackingField: i32,
    pub layerTextures: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::OVROverlay_LayerTexture,
        >,
    >,
    pub layerDesc: crate::GlobalNamespace::OVRPlugin_LayerDesc,
    pub stageCount: i32,
    pub layerIndex: i32,
    pub layerIdHandle: crate::System::Runtime::InteropServices::GCHandle,
    pub layerIdPtr: crate::System::IntPtr,
    pub frameIndex: i32,
    pub prevFrameIndex: i32,
    pub rend: quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
    pub OpenVROverlayHandle: u64,
    pub OpenVRUVOffsetAndScale: crate::UnityEngine::Vector4,
    pub OpenVRMouseScale: crate::UnityEngine::Vector2,
    pub constructedOverlayXRDevice: crate::GlobalNamespace::OVRManager_XRDevice,
    pub xrDeviceConstructed: bool,
}
#[cfg(feature = "OVROverlay")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVROverlay => ""."OVROverlay"
);
#[cfg(feature = "OVROverlay")]
impl std::ops::Deref for crate::GlobalNamespace::OVROverlay {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVROverlay")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVROverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVROverlay")]
impl crate::GlobalNamespace::OVROverlay {
    pub const maxInstances: i32 = 15i32;
    #[cfg(feature = "OVROverlay+ExternalSurfaceObjectCreated")]
    pub type ExternalSurfaceObjectCreated = crate::GlobalNamespace::OVROverlay_ExternalSurfaceObjectCreated;
    #[cfg(feature = "OVROverlay+LayerTexture")]
    pub type LayerTexture = crate::GlobalNamespace::OVROverlay_LayerTexture;
    #[cfg(feature = "OVROverlay+OverlayShape")]
    pub type OverlayShape = crate::GlobalNamespace::OVROverlay_OverlayShape;
    #[cfg(feature = "OVROverlay+OverlayType")]
    pub type OverlayType = crate::GlobalNamespace::OVROverlay_OverlayType;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn BlitSubImage(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        dst: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        rect: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BlitSubImage", (src, dst, mat, rect))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputePoseAndScale(
        &mut self,
        pose: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPose>,
        scale: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        overlay: quest_hook::libil2cpp::ByRefMut<bool>,
        headLocked: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ComputePoseAndScale", (pose, scale, overlay, headLocked))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeSubmit(
        &mut self,
        pose: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::OVRPose>,
        scale: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        overlay: quest_hook::libil2cpp::ByRefMut<bool>,
        headLocked: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ComputeSubmit", (pose, scale, overlay, headLocked))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateLayer(
        &mut self,
        mipLevels: i32,
        sampleCount: i32,
        etFormat: crate::GlobalNamespace::OVRPlugin_EyeTextureFormat,
        flags: i32,
        _cordl_size: crate::GlobalNamespace::OVRPlugin_Sizei,
        shape: crate::GlobalNamespace::OVRPlugin_OverlayShape,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CreateLayer",
                (mipLevels, sampleCount, etFormat, flags, _cordl_size, shape),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateLayerTextures(
        &mut self,
        useMipmaps: bool,
        _cordl_size: crate::GlobalNamespace::OVRPlugin_Sizei,
        isHdr: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CreateLayerTextures", (useMipmaps, _cordl_size, isHdr))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyLayer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyLayer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyLayerTextures(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyLayerTextures", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBlitRect(
        &mut self,
        eyeId: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rect> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rect = __cordl_object
            .invoke("GetBlitRect", (eyeId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCurrentLayerDesc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_LayerDesc> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_LayerDesc = __cordl_object
            .invoke("GetCurrentLayerDesc", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InitOVROverlay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitOVROverlay", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPassthroughShape(
        shape: crate::GlobalNamespace::OVROverlay_OverlayShape,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPassthroughShape", (shape))?;
        Ok(__cordl_ret.into())
    }
    pub fn LatchLayerTextures(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("LatchLayerTextures", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn NeedsTexturesForShape(
        shape: crate::GlobalNamespace::OVROverlay_OverlayShape,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("NeedsTexturesForShape", (shape))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OpenVROverlayUpdate(
        &mut self,
        scale: crate::UnityEngine::Vector3,
        pose: crate::GlobalNamespace::OVRPose,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OpenVROverlayUpdate", (scale, pose))?;
        Ok(__cordl_ret.into())
    }
    pub fn OverrideOverlayTextureInfo(
        &mut self,
        srcTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        nativePtr: crate::System::IntPtr,
        node: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OverrideOverlayTextureInfo", (srcTexture, nativePtr, node))?;
        Ok(__cordl_ret.into())
    }
    pub fn PopulateLayer(
        &mut self,
        mipLevels: i32,
        isHdr: bool,
        _cordl_size: crate::GlobalNamespace::OVRPlugin_Sizei,
        sampleCount: i32,
        stage: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "PopulateLayer",
                (mipLevels, isHdr, _cordl_size, sampleCount, stage),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPerLayerColorScaleAndOffset(
        &mut self,
        scale: crate::UnityEngine::Vector4,
        offset: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPerLayerColorScaleAndOffset", (scale, offset))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetSrcDestRects(
        &mut self,
        srcLeft: crate::UnityEngine::Rect,
        srcRight: crate::UnityEngine::Rect,
        destLeft: crate::UnityEngine::Rect,
        destRight: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSrcDestRects", (srcLeft, srcRight, destLeft, destRight))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetupEditorPreview(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetupEditorPreview", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SubmitLayer(
        &mut self,
        overlay: bool,
        headLocked: bool,
        noDepthBufferTesting: bool,
        pose: crate::GlobalNamespace::OVRPose,
        scale: crate::UnityEngine::Vector3,
        frameIndex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "SubmitLayer",
                (overlay, headLocked, noDepthBufferTesting, pose, scale, frameIndex),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateTextureRectMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateTextureRectMatrix", ())?;
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
    pub fn get_OpenVROverlayKey() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_OpenVROverlayKey", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_layerId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_layerId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_layout(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRPlugin_LayerLayout> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_LayerLayout = __cordl_object
            .invoke("get_layout", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_previewInEditor(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_previewInEditor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_texturesPerStage(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_texturesPerStage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_layerId(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_layerId", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_previewInEditor(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_previewInEditor", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVROverlay")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVROverlay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVROverlay+ExternalSurfaceObjectCreated")]
#[repr(C)]
#[derive(Debug)]
pub struct OVROverlay_ExternalSurfaceObjectCreated {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVROverlay+ExternalSurfaceObjectCreated")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVROverlay_ExternalSurfaceObjectCreated => ""
    ."OVROverlay/ExternalSurfaceObjectCreated"
);
#[cfg(feature = "OVROverlay+ExternalSurfaceObjectCreated")]
impl std::ops::Deref
for crate::GlobalNamespace::OVROverlay_ExternalSurfaceObjectCreated {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVROverlay+ExternalSurfaceObjectCreated")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVROverlay_ExternalSurfaceObjectCreated {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVROverlay+ExternalSurfaceObjectCreated")]
impl crate::GlobalNamespace::OVROverlay_ExternalSurfaceObjectCreated {
    pub fn BeginInvoke(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVROverlay+ExternalSurfaceObjectCreated")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVROverlay_ExternalSurfaceObjectCreated {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVROverlay+LayerTexture")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct OVROverlay_LayerTexture {
    pub appTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub appTexturePtr: crate::System::IntPtr,
    pub swapChain: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
        >,
    >,
    pub swapChainPtr: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::System::IntPtr>,
    >,
}
#[cfg(feature = "OVROverlay+LayerTexture")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVROverlay_LayerTexture => ""
    ."OVROverlay/LayerTexture"
);
#[cfg(feature = "OVROverlay+LayerTexture")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVROverlay_LayerTexture {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVROverlay+LayerTexture")]
impl crate::GlobalNamespace::OVROverlay_LayerTexture {}
#[cfg(feature = "OVROverlay+OverlayShape")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVROverlay_OverlayShape {
    #[default]
    Cubemap = 2i32,
    Cylinder = 1i32,
    Equirect = 5i32,
    Fisheye = 9i32,
    KeyboardHandsPassthrough = 10i32,
    KeyboardMaskedHandsPassthrough = 11i32,
    OffcenterCubemap = 4i32,
    Quad = 0i32,
    ReconstructionPassthrough = 7i32,
    SurfaceProjectedPassthrough = 8i32,
}
#[cfg(feature = "OVROverlay+OverlayShape")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVROverlay_OverlayShape => ""
    ."OVROverlay/OverlayShape"
);
#[cfg(feature = "OVROverlay+OverlayType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVROverlay_OverlayType {
    #[default]
    None = 0i32,
    Overlay = 2i32,
    Underlay = 1i32,
}
#[cfg(feature = "OVROverlay+OverlayType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVROverlay_OverlayType => ""
    ."OVROverlay/OverlayType"
);
