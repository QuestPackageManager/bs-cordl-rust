#[cfg(feature = "cordl_class_OVR+OpenVR+CVRTrackedCamera")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct CVRTrackedCamera {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FnTable: crate::OVR::OpenVR::IVRTrackedCamera,
}
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRTrackedCamera")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::CVRTrackedCamera {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "CVRTrackedCamera";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "OVR+OpenVR+CVRTrackedCamera")]
impl std::ops::Deref for crate::OVR::OpenVR::CVRTrackedCamera {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+CVRTrackedCamera")]
impl std::ops::DerefMut for crate::OVR::OpenVR::CVRTrackedCamera {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u32, quest_hook::libil2cpp::ByRefMut<u64>),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        2usize,
                    >("AcquireVideoStreamingService")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AcquireVideoStreamingService", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError =
            unsafe { cordl_method_info.invoke_unchecked(self, (nDeviceIndex, pHandle))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCameraErrorNameFromEnum(
        &mut self,
        eCameraError: crate::OVR::OpenVR::EVRTrackedCameraError,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::OVR::OpenVR::EVRTrackedCameraError),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("GetCameraErrorNameFromEnum")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCameraErrorNameFromEnum", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, (eCameraError))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetCameraFrameSize(
        &mut self,
        nDeviceIndex: u32,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
        pnFrameBufferSize: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        u32,
                        crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                        quest_hook::libil2cpp::ByRefMut<u32>,
                        quest_hook::libil2cpp::ByRefMut<u32>,
                        quest_hook::libil2cpp::ByRefMut<u32>,
                    ), crate::OVR::OpenVR::EVRTrackedCameraError, 5usize>(
                        "GetCameraFrameSize"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCameraFrameSize",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    nDeviceIndex,
                    eFrameType,
                    pnWidth,
                    pnHeight,
                    pnFrameBufferSize,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCameraIntrinsics(
        &mut self,
        nDeviceIndex: u32,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        pFocalLength: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
        pCenter: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        u32,
                        crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                        quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
                        quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
                    ), crate::OVR::OpenVR::EVRTrackedCameraError, 4usize>(
                        "GetCameraIntrinsics"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCameraIntrinsics",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (nDeviceIndex, eFrameType, pFocalLength, pCenter))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCameraProjection(
        &mut self,
        nDeviceIndex: u32,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        flZNear: f32,
        flZFar: f32,
        pProjection: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdMatrix44_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        u32,
                        crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                        f32,
                        f32,
                        quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdMatrix44_t>,
                    ), crate::OVR::OpenVR::EVRTrackedCameraError, 5usize>(
                        "GetCameraProjection"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCameraProjection",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (nDeviceIndex, eFrameType, flZNear, flZFar, pProjection),
            )?
        };
        Ok(__cordl_ret.into())
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        u64,
                        crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                        crate::System::IntPtr,
                        u32,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
                        >,
                        u32,
                    ), crate::OVR::OpenVR::EVRTrackedCameraError, 6usize>(
                        "GetVideoStreamFrameBuffer",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetVideoStreamFrameBuffer",
                            6usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    hTrackedCamera,
                    eFrameType,
                    pFrameBuffer,
                    nFrameBufferSize,
                    pFrameHeader,
                    nFrameHeaderSize,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetVideoStreamTextureD3D11(
        &mut self,
        hTrackedCamera: u64,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        pD3D11DeviceOrResource: crate::System::IntPtr,
        ppD3D11ShaderResourceView: quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
        pFrameHeader: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
        >,
        nFrameHeaderSize: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        u64,
                        crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                        crate::System::IntPtr,
                        quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
                        >,
                        u32,
                    ), crate::OVR::OpenVR::EVRTrackedCameraError, 6usize>(
                        "GetVideoStreamTextureD3D11",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetVideoStreamTextureD3D11",
                            6usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    hTrackedCamera,
                    eFrameType,
                    pD3D11DeviceOrResource,
                    ppD3D11ShaderResourceView,
                    pFrameHeader,
                    nFrameHeaderSize,
                ),
            )?
        };
        Ok(__cordl_ret.into())
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        u64,
                        crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                        quest_hook::libil2cpp::ByRefMut<u32>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
                        >,
                        u32,
                    ), crate::OVR::OpenVR::EVRTrackedCameraError, 5usize>(
                        "GetVideoStreamTextureGL"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetVideoStreamTextureGL",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    hTrackedCamera,
                    eFrameType,
                    pglTextureId,
                    pFrameHeader,
                    nFrameHeaderSize,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetVideoStreamTextureSize(
        &mut self,
        nDeviceIndex: u32,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        pTextureBounds: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VRTextureBounds_t>,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        u32,
                        crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                        quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::VRTextureBounds_t>,
                        quest_hook::libil2cpp::ByRefMut<u32>,
                        quest_hook::libil2cpp::ByRefMut<u32>,
                    ), crate::OVR::OpenVR::EVRTrackedCameraError, 5usize>(
                        "GetVideoStreamTextureSize",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetVideoStreamTextureSize",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (nDeviceIndex, eFrameType, pTextureBounds, pnWidth, pnHeight),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HasCamera(
        &mut self,
        nDeviceIndex: u32,
        pHasCamera: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u32, quest_hook::libil2cpp::ByRefMut<bool>),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        2usize,
                    >("HasCamera")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HasCamera", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError =
            unsafe { cordl_method_info.invoke_unchecked(self, (nDeviceIndex, pHasCamera))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pInterface))?;
        Ok(__cordl_object.into())
    }
    pub fn ReleaseVideoStreamTextureGL(
        &mut self,
        hTrackedCamera: u64,
        glTextureId: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u64, u32), crate::OVR::OpenVR::EVRTrackedCameraError, 2usize>(
                        "ReleaseVideoStreamTextureGL",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReleaseVideoStreamTextureGL",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError =
            unsafe { cordl_method_info.invoke_unchecked(self, (hTrackedCamera, glTextureId))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReleaseVideoStreamingService(
        &mut self,
        hTrackedCamera: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(u64), crate::OVR::OpenVR::EVRTrackedCameraError, 1usize>(
                        "ReleaseVideoStreamingService",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReleaseVideoStreamingService",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError =
            unsafe { cordl_method_info.invoke_unchecked(self, (hTrackedCamera))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        pInterface: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(crate::System::IntPtr), quest_hook::libil2cpp::Void, 1usize>(
                        ".ctor",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (pInterface))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+CVRTrackedCamera")]
impl quest_hook::libil2cpp::ObjectType for crate::OVR::OpenVR::CVRTrackedCamera {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
