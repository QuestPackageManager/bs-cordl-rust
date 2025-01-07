#[cfg(feature = "LIV+SDK+Unity+SDKRender")]
#[repr(C)]
#[derive(Debug)]
pub struct SDKRender {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _clipPlaneCommandBuffer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::CommandBuffer,
    >,
    pub _combineAlphaCommandBuffer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::CommandBuffer,
    >,
    pub _captureTextureCommandBuffer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::CommandBuffer,
    >,
    pub _applyTextureCommandBuffer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::CommandBuffer,
    >,
    pub _optimizedRenderingCommandBuffer: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::CommandBuffer,
    >,
    pub _clipPlaneCameraEvent: crate::UnityEngine::Rendering::CameraEvent,
    pub _optimizedRenderingCameraEvent: crate::UnityEngine::Rendering::CameraEvent,
    pub _clipPlaneMesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    pub _clipPlaneSimpleMaterial: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Material,
    >,
    pub _clipPlaneSimpleDebugMaterial: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Material,
    >,
    pub _clipPlaneComplexMaterial: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Material,
    >,
    pub _clipPlaneComplexDebugMaterial: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Material,
    >,
    pub _writeOpaqueToAlphaMaterial: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Material,
    >,
    pub _combineAlphaMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _writeMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _forceForwardRenderingMaterial: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Material,
    >,
    pub _backgroundRenderTexture: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::RenderTexture,
    >,
    pub _foregroundRenderTexture: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::RenderTexture,
    >,
    pub _optimizedRenderTexture: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::RenderTexture,
    >,
    pub _complexClipPlaneRenderTexture: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::RenderTexture,
    >,
    pub _liv: quest_hook::libil2cpp::Gc<crate::LIV::SDK::Unity::LIV>,
    pub _outputFrame: crate::LIV::SDK::Unity::SDKOutputFrame,
    pub _inputFrame: crate::LIV::SDK::Unity::SDKInputFrame,
    pub _resolution: crate::LIV::SDK::Unity::SDKResolution,
    pub _cameraInstance: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    pub _requestedPose: crate::LIV::SDK::Unity::SDKPose,
    pub _requestedPoseFrameIndex: i32,
}
#[cfg(feature = "LIV+SDK+Unity+SDKRender")]
unsafe impl quest_hook::libil2cpp::Type for crate::LIV::SDK::Unity::SDKRender {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "LIV.SDK.Unity";
    const CLASS_NAME: &'static str = "SDKRender";
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
#[cfg(feature = "LIV+SDK+Unity+SDKRender")]
impl std::ops::Deref for crate::LIV::SDK::Unity::SDKRender {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKRender")]
impl std::ops::DerefMut for crate::LIV::SDK::Unity::SDKRender {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKRender")]
impl crate::LIV::SDK::Unity::SDKRender {
    pub fn CreateAssets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateAssets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateBackgroundTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateBackgroundTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateComplexClipPlaneTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateComplexClipPlaneTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateForegroundTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateForegroundTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateOptimizedTexture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateOptimizedTexture", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyAssets(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyAssets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClipPlaneMaterial(
        &mut self,
        debugClipPlane: bool,
        complexClipPlane: bool,
        colorWriteMask: crate::UnityEngine::Rendering::ColorWriteMask,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke(
                "GetClipPlaneMaterial",
                (debugClipPlane, complexClipPlane, colorWriteMask),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGroundClipPlaneMaterial(
        &mut self,
        debugClipPlane: bool,
        colorWriteMask: crate::UnityEngine::Rendering::ColorWriteMask,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> = __cordl_object
            .invoke("GetGroundClipPlaneMaterial", (debugClipPlane, colorWriteMask))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokePostRenderBackground(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokePostRenderBackground", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokePostRenderForeground(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokePostRenderForeground", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokePreRender(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokePreRender", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokePreRenderBackground(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokePreRenderBackground", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokePreRenderForeground(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokePreRenderForeground", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IvokePostRender(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IvokePostRender", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        liv: quest_hook::libil2cpp::Gc<crate::LIV::SDK::Unity::LIV>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (liv))?;
        Ok(__cordl_object.into())
    }
    pub fn ReleaseBridgePoseControl(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReleaseBridgePoseControl", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Render(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Render", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderBackground(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RenderBackground", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderForeground(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RenderForeground", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RenderOptimized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RenderOptimized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SendTextureToBridge(
        &mut self,
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        id: crate::LIV::SDK::Unity::TEXTURE_ID,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendTextureToBridge", (texture, id))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGroundPlane_Plane__cordl_bool1(
        &mut self,
        plane: crate::UnityEngine::Plane,
        useLocalSpace: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGroundPlane", (plane, useLocalSpace))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGroundPlane_Transform__cordl_bool2(
        &mut self,
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        useLocalSpace: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGroundPlane", (transform, useLocalSpace))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGroundPlane_f32_Vector3__cordl_bool0(
        &mut self,
        distance: f32,
        normal: crate::UnityEngine::Vector3,
        useLocalSpace: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGroundPlane", (distance, normal, useLocalSpace))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPose(
        &mut self,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        verticalFieldOfView: f32,
        useLocalSpace: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "SetPose",
                (position, rotation, verticalFieldOfView, useLocalSpace),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateBridgeInputFrame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateBridgeInputFrame", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateBridgeResolution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateBridgeResolution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateCameraSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateCameraSettings", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateTextures(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateTextures", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        liv: quest_hook::libil2cpp::Gc<crate::LIV::SDK::Unity::LIV>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (liv))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cameraInstance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera> = __cordl_object
            .invoke("get_cameraInstance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cameraReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera> = __cordl_object
            .invoke("get_cameraReference", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_canRenderBackground(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_canRenderBackground", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_canRenderForeground(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_canRenderForeground", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_canRenderOptimized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_canRenderOptimized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_canSetPose(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_canSetPose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disableStandardAssets(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_disableStandardAssets", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hmdCamera(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera> = __cordl_object
            .invoke("get_hmdCamera", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_inputFrame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKInputFrame> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::LIV::SDK::Unity::SDKInputFrame = __cordl_object
            .invoke("get_inputFrame", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_interlacedRendering(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_interlacedRendering", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_liv(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LIV::SDK::Unity::LIV>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::LIV::SDK::Unity::LIV> = __cordl_object
            .invoke("get_liv", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_localToWorldMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("get_localToWorldMatrix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_outputFrame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKOutputFrame> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::LIV::SDK::Unity::SDKOutputFrame = __cordl_object
            .invoke("get_outputFrame", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_resolution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKResolution> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::LIV::SDK::Unity::SDKResolution = __cordl_object
            .invoke("get_resolution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_spectatorLayerMask(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_spectatorLayerMask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = __cordl_object
            .invoke("get_stage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stageLocalToWorldMatrix(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("get_stageLocalToWorldMatrix", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_stageTransform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform> = __cordl_object
            .invoke("get_stageTransform", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useDeferredRendering(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useDeferredRendering", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKRender")]
impl quest_hook::libil2cpp::ObjectType for crate::LIV::SDK::Unity::SDKRender {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKRender")]
impl AsRef<crate::System::IDisposable> for crate::LIV::SDK::Unity::SDKRender {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKRender")]
impl AsMut<crate::System::IDisposable> for crate::LIV::SDK::Unity::SDKRender {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
