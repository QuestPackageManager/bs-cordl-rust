#[cfg(feature = "LIV+SDK+Unity+SDKUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct SDKUtils {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "LIV+SDK+Unity+SDKUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LIV::SDK::Unity::SDKUtils => "LIV.SDK.Unity"
    ."SDKUtils"
);
#[cfg(feature = "LIV+SDK+Unity+SDKUtils")]
impl std::ops::Deref for crate::LIV::SDK::Unity::SDKUtils {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKUtils")]
impl std::ops::DerefMut for crate::LIV::SDK::Unity::SDKUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKUtils")]
impl crate::LIV::SDK::Unity::SDKUtils {
    pub fn ApplyUserSpaceTransform(
        render: quest_hook::libil2cpp::Gc<crate::LIV::SDK::Unity::SDKRender>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyUserSpaceTransform", (render))?;
        Ok(__cordl_ret.into())
    }
    pub fn CleanCameraBehaviours(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        excludeBehaviours: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CleanCameraBehaviours", (camera, excludeBehaviours))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsFlag(flags: u64, flag: u64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ContainsFlag", (flags, flag))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateBridgeOutputFrame(
        render: quest_hook::libil2cpp::Gc<crate::LIV::SDK::Unity::SDKRender>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateBridgeOutputFrame", (render))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateClipPlane(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        resX: i32,
        resY: i32,
        useQuads: bool,
        skirtLength: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateClipPlane", (mesh, resX, resY, useQuads, skirtLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTexture(
        renderTexture: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        >,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::UnityEngine::RenderTextureFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateTexture", (renderTexture, width, height, depth, format))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyObject<T>(
        reference: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyObject", (reference))?;
        Ok(__cordl_ret.into())
    }
    pub fn DestroyTexture(
        _renderTexture: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DestroyTexture", (_renderTexture))?;
        Ok(__cordl_ret.into())
    }
    pub fn DisableStandardAssets(
        cameraInstance: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        behaviours: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
                >,
            >,
        >,
        wasBehaviourEnabled: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "DisableStandardAssets",
                (cameraInstance, behaviours, wasBehaviourEnabled),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn DisposeObject<T>(
        reference: quest_hook::libil2cpp::ByRefMut<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DisposeObject", (reference))?;
        Ok(__cordl_ret.into())
    }
    pub fn FeatureEnabled(
        features: crate::LIV::SDK::Unity::FEATURES,
        feature: crate::LIV::SDK::Unity::FEATURES,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FeatureEnabled", (features, feature))?;
        Ok(__cordl_ret.into())
    }
    pub fn ForceForwardRendering(
        cameraInstance: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        clipPlaneMesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        forceForwardRenderingMaterial: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Material,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ForceForwardRendering",
                (cameraInstance, clipPlaneMesh, forceForwardRenderingMaterial),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCameraPositionAndRotation(
        pose: crate::LIV::SDK::Unity::SDKPose,
        originLocalToWorldMatrix: crate::UnityEngine::Matrix4x4,
        position: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        rotation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetCameraPositionAndRotation",
                (pose, originLocalToWorldMatrix, position, rotation),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetColorSpace(
        renderTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE> {
        let __cordl_ret: crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetColorSpace", (renderTexture))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDevice() -> quest_hook::libil2cpp::Result<
        crate::LIV::SDK::Unity::TEXTURE_DEVICE,
    > {
        let __cordl_ret: crate::LIV::SDK::Unity::TEXTURE_DEVICE = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDevice", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReadWriteFromColorSpace(
        colorSpace: crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::RenderTextureReadWrite> {
        let __cordl_ret: crate::UnityEngine::RenderTextureReadWrite = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetReadWriteFromColorSpace", (colorSpace))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRenderingPipeline(
        renderingPath: crate::UnityEngine::RenderingPath,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::RENDERING_PIPELINE> {
        let __cordl_ret: crate::LIV::SDK::Unity::RENDERING_PIPELINE = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRenderingPipeline", (renderingPath))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTrackedSpace(
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<crate::LIV::SDK::Unity::SDKTrackedSpace> {
        let __cordl_ret: crate::LIV::SDK::Unity::SDKTrackedSpace = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTrackedSpace", (transform))?;
        Ok(__cordl_ret.into())
    }
    pub fn RestoreStandardAssets(
        behaviours: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
                >,
            >,
        >,
        wasBehaviourEnabled: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<bool>>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RestoreStandardAssets", (behaviours, wasBehaviourEnabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn RotateQuaternionByMatrix(
        matrix: crate::UnityEngine::Matrix4x4,
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RotateQuaternionByMatrix", (matrix, rotation))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCamera(
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        cameraTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        inputFrame: crate::LIV::SDK::Unity::SDKInputFrame,
        originLocalToWorldMatrix: crate::UnityEngine::Matrix4x4,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetCamera",
                (
                    camera,
                    cameraTransform,
                    inputFrame,
                    originLocalToWorldMatrix,
                    layerMask,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFlag(
        flags: u64,
        flag: u64,
        enabled: bool,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetFlag", (flags, flag, enabled))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GetDefaultColorSpace() -> quest_hook::libil2cpp::Result<
        crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE,
    > {
        let __cordl_ret: crate::LIV::SDK::Unity::TEXTURE_COLOR_SPACE = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_GetDefaultColorSpace", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LIV+SDK+Unity+SDKUtils")]
impl quest_hook::libil2cpp::ObjectType for crate::LIV::SDK::Unity::SDKUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
