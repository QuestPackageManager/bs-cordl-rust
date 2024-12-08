#[cfg(feature = "OVR+OpenVR+CVRTrackedCamera")]
#[repr(C)]
#[derive(Debug)]
pub struct CVRTrackedCamera {
    __cordl_parent: crate::System::Object,
    pub FnTable: crate::OVR::OpenVR::IVRTrackedCamera,
}
#[cfg(feature = "OVR+OpenVR+CVRTrackedCamera")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::CVRTrackedCamera => "OVR.OpenVR"
    ."CVRTrackedCamera"
);
#[cfg(feature = "OVR+OpenVR+CVRTrackedCamera")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRTrackedCamera {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRTrackedCamera")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRTrackedCamera {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRTrackedCamera")]
impl crate::OVR::OpenVR::CVRTrackedCamera {
    pub fn AcquireVideoStreamingService(
        &mut self,
        nDeviceIndex: u32,
        pHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = __cordl_object
            .invoke("AcquireVideoStreamingService", (nDeviceIndex, pHandle))?;
        Ok(__cordl_ret)
    }
    pub fn GetCameraErrorNameFromEnum(
        &mut self,
        eCameraError: crate::OVR::OpenVR::EVRTrackedCameraError,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetCameraErrorNameFromEnum", (eCameraError))?;
        Ok(__cordl_ret)
    }
    pub fn GetCameraFrameSize(
        &mut self,
        nDeviceIndex: u32,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
        pnFrameBufferSize: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = __cordl_object
            .invoke(
                "GetCameraFrameSize",
                (nDeviceIndex, eFrameType, pnWidth, pnHeight, pnFrameBufferSize),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetCameraIntrinsics(
        &mut self,
        nDeviceIndex: u32,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        pFocalLength: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
        pCenter: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = __cordl_object
            .invoke(
                "GetCameraIntrinsics",
                (nDeviceIndex, eFrameType, pFocalLength, pCenter),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetCameraProjection(
        &mut self,
        nDeviceIndex: u32,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        flZNear: f32,
        flZFar: f32,
        pProjection: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdMatrix44_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = __cordl_object
            .invoke(
                "GetCameraProjection",
                (nDeviceIndex, eFrameType, flZNear, flZFar, pProjection),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetVideoStreamFrameBuffer(
        &mut self,
        hTrackedCamera: u64,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        pFrameBuffer: crate::System::IntPtr,
        nFrameBufferSize: u32,
        pFrameHeader: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
        >,
        nFrameHeaderSize: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = __cordl_object
            .invoke(
                "GetVideoStreamFrameBuffer",
                (
                    hTrackedCamera,
                    eFrameType,
                    pFrameBuffer,
                    nFrameBufferSize,
                    pFrameHeader,
                    nFrameHeaderSize,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetVideoStreamTextureD3D11(
        &mut self,
        hTrackedCamera: u64,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        pD3D11DeviceOrResource: crate::System::IntPtr,
        ppD3D11ShaderResourceView: quest_hook::libil2cpp::ByRefMut<
            crate::System::IntPtr,
        >,
        pFrameHeader: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
        >,
        nFrameHeaderSize: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = __cordl_object
            .invoke(
                "GetVideoStreamTextureD3D11",
                (
                    hTrackedCamera,
                    eFrameType,
                    pD3D11DeviceOrResource,
                    ppD3D11ShaderResourceView,
                    pFrameHeader,
                    nFrameHeaderSize,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetVideoStreamTextureGL(
        &mut self,
        hTrackedCamera: u64,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        pglTextureId: quest_hook::libil2cpp::ByRefMut<u32>,
        pFrameHeader: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
        >,
        nFrameHeaderSize: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = __cordl_object
            .invoke(
                "GetVideoStreamTextureGL",
                (
                    hTrackedCamera,
                    eFrameType,
                    pglTextureId,
                    pFrameHeader,
                    nFrameHeaderSize,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetVideoStreamTextureSize(
        &mut self,
        nDeviceIndex: u32,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        pTextureBounds: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRTextureBounds_t,
        >,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = __cordl_object
            .invoke(
                "GetVideoStreamTextureSize",
                (nDeviceIndex, eFrameType, pTextureBounds, pnWidth, pnHeight),
            )?;
        Ok(__cordl_ret)
    }
    pub fn HasCamera(
        &mut self,
        nDeviceIndex: u32,
        pHasCamera: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = __cordl_object
            .invoke("HasCamera", (nDeviceIndex, pHasCamera))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pInterface))?;
        Ok(__cordl_object)
    }
    pub fn ReleaseVideoStreamTextureGL(
        &mut self,
        hTrackedCamera: u64,
        glTextureId: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = __cordl_object
            .invoke("ReleaseVideoStreamTextureGL", (hTrackedCamera, glTextureId))?;
        Ok(__cordl_ret)
    }
    pub fn ReleaseVideoStreamingService(
        &mut self,
        hTrackedCamera: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = __cordl_object
            .invoke("ReleaseVideoStreamingService", (hTrackedCamera))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (pInterface))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVR+OpenVR+CVRTrackedCamera")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRTrackedCamera {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
