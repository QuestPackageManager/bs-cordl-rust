#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IVRTrackedCamera {
    pub GetCameraErrorNameFromEnum: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRTrackedCamera__GetCameraErrorNameFromEnum,
    >,
    pub HasCamera: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRTrackedCamera__HasCamera,
    >,
    pub GetCameraFrameSize: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRTrackedCamera__GetCameraFrameSize,
    >,
    pub GetCameraIntrinsics: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRTrackedCamera__GetCameraIntrinsics,
    >,
    pub GetCameraProjection: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRTrackedCamera__GetCameraProjection,
    >,
    pub AcquireVideoStreamingService: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRTrackedCamera__AcquireVideoStreamingService,
    >,
    pub ReleaseVideoStreamingService: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRTrackedCamera__ReleaseVideoStreamingService,
    >,
    pub GetVideoStreamFrameBuffer: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamFrameBuffer,
    >,
    pub GetVideoStreamTextureSize: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureSize,
    >,
    pub GetVideoStreamTextureD3D11: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureD3D11,
    >,
    pub GetVideoStreamTextureGL: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureGL,
    >,
    pub ReleaseVideoStreamTextureGL: quest_hook::libil2cpp::Gc<
        crate::OVR::OpenVR::IVRTrackedCamera__ReleaseVideoStreamTextureGL,
    >,
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::IVRTrackedCamera {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRTrackedCamera";
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
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera")]
unsafe impl quest_hook::libil2cpp::Argument for crate::OVR::OpenVR::IVRTrackedCamera {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::OVR::OpenVR::IVRTrackedCamera {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera")]
unsafe impl quest_hook::libil2cpp::Returned for crate::OVR::OpenVR::IVRTrackedCamera {
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
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera")]
unsafe impl quest_hook::libil2cpp::Return for crate::OVR::OpenVR::IVRTrackedCamera {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OVR::OpenVR::IVRTrackedCamera {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera")]
impl crate::OVR::OpenVR::IVRTrackedCamera {
    #[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_AcquireVideoStreamingService")]
    pub type _AcquireVideoStreamingService = crate::OVR::OpenVR::IVRTrackedCamera__AcquireVideoStreamingService;
    #[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraErrorNameFromEnum")]
    pub type _GetCameraErrorNameFromEnum = crate::OVR::OpenVR::IVRTrackedCamera__GetCameraErrorNameFromEnum;
    #[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraFrameSize")]
    pub type _GetCameraFrameSize = crate::OVR::OpenVR::IVRTrackedCamera__GetCameraFrameSize;
    #[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraIntrinsics")]
    pub type _GetCameraIntrinsics = crate::OVR::OpenVR::IVRTrackedCamera__GetCameraIntrinsics;
    #[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraProjection")]
    pub type _GetCameraProjection = crate::OVR::OpenVR::IVRTrackedCamera__GetCameraProjection;
    #[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamFrameBuffer")]
    pub type _GetVideoStreamFrameBuffer = crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamFrameBuffer;
    #[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureD3D11")]
    pub type _GetVideoStreamTextureD3D11 = crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureD3D11;
    #[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureGL")]
    pub type _GetVideoStreamTextureGL = crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureGL;
    #[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureSize")]
    pub type _GetVideoStreamTextureSize = crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureSize;
    #[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_HasCamera")]
    pub type _HasCamera = crate::OVR::OpenVR::IVRTrackedCamera__HasCamera;
    #[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_ReleaseVideoStreamTextureGL")]
    pub type _ReleaseVideoStreamTextureGL = crate::OVR::OpenVR::IVRTrackedCamera__ReleaseVideoStreamTextureGL;
    #[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_ReleaseVideoStreamingService")]
    pub type _ReleaseVideoStreamingService = crate::OVR::OpenVR::IVRTrackedCamera__ReleaseVideoStreamingService;
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_AcquireVideoStreamingService")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRTrackedCamera__AcquireVideoStreamingService {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_AcquireVideoStreamingService")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRTrackedCamera__AcquireVideoStreamingService {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRTrackedCamera/_AcquireVideoStreamingService";
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
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_AcquireVideoStreamingService")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRTrackedCamera__AcquireVideoStreamingService {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_AcquireVideoStreamingService")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRTrackedCamera__AcquireVideoStreamingService {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_AcquireVideoStreamingService")]
impl crate::OVR::OpenVR::IVRTrackedCamera__AcquireVideoStreamingService {
    pub fn BeginInvoke(
        &mut self,
        nDeviceIndex: u32,
        pHandle: quest_hook::libil2cpp::ByRefMut<u64>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u32,
                            quest_hook::libil2cpp::ByRefMut<u64>,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        4usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method.invoke_unchecked(self, (nDeviceIndex, pHandle, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pHandle: quest_hook::libil2cpp::ByRefMut<u64>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<u64>,
                            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        ),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        2usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method.invoke_unchecked(self, (pHandle, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        nDeviceIndex: u32,
        pHandle: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u32, quest_hook::libil2cpp::ByRefMut<u64>),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        2usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method.invoke_unchecked(self, (nDeviceIndex, pHandle))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_AcquireVideoStreamingService")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRTrackedCamera__AcquireVideoStreamingService {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraErrorNameFromEnum")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRTrackedCamera__GetCameraErrorNameFromEnum {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraErrorNameFromEnum")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRTrackedCamera__GetCameraErrorNameFromEnum {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRTrackedCamera/_GetCameraErrorNameFromEnum";
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
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraErrorNameFromEnum")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRTrackedCamera__GetCameraErrorNameFromEnum {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraErrorNameFromEnum")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRTrackedCamera__GetCameraErrorNameFromEnum {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraErrorNameFromEnum")]
impl crate::OVR::OpenVR::IVRTrackedCamera__GetCameraErrorNameFromEnum {
    pub fn BeginInvoke(
        &mut self,
        eCameraError: crate::OVR::OpenVR::EVRTrackedCameraError,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::OVR::OpenVR::EVRTrackedCameraError,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        3usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method.invoke_unchecked(self, (eCameraError, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                        crate::System::IntPtr,
                        1usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked(self, (result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        eCameraError: crate::OVR::OpenVR::EVRTrackedCameraError,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::OVR::OpenVR::EVRTrackedCameraError),
                        crate::System::IntPtr,
                        1usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked(self, (eCameraError))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraErrorNameFromEnum")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRTrackedCamera__GetCameraErrorNameFromEnum {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraFrameSize")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRTrackedCamera__GetCameraFrameSize {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraFrameSize")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRTrackedCamera__GetCameraFrameSize {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRTrackedCamera/_GetCameraFrameSize";
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
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraFrameSize")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRTrackedCamera__GetCameraFrameSize {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraFrameSize")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRTrackedCamera__GetCameraFrameSize {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraFrameSize")]
impl crate::OVR::OpenVR::IVRTrackedCamera__GetCameraFrameSize {
    pub fn BeginInvoke(
        &mut self,
        nDeviceIndex: u32,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
        pnFrameBufferSize: quest_hook::libil2cpp::ByRefMut<u32>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u32,
                            crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        7usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        nDeviceIndex,
                        eFrameType,
                        pnWidth,
                        pnHeight,
                        pnFrameBufferSize,
                        callback,
                        object,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
        pnFrameBufferSize: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        ),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        4usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method
                .invoke_unchecked(self, (pnWidth, pnHeight, pnFrameBufferSize, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        nDeviceIndex: u32,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
        pnFrameBufferSize: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u32,
                            crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                        ),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        5usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (nDeviceIndex, eFrameType, pnWidth, pnHeight, pnFrameBufferSize),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraFrameSize")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRTrackedCamera__GetCameraFrameSize {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraIntrinsics")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRTrackedCamera__GetCameraIntrinsics {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraIntrinsics")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRTrackedCamera__GetCameraIntrinsics {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRTrackedCamera/_GetCameraIntrinsics";
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
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraIntrinsics")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRTrackedCamera__GetCameraIntrinsics {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraIntrinsics")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRTrackedCamera__GetCameraIntrinsics {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraIntrinsics")]
impl crate::OVR::OpenVR::IVRTrackedCamera__GetCameraIntrinsics {
    pub fn BeginInvoke(
        &mut self,
        nDeviceIndex: u32,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        pFocalLength: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
        pCenter: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u32,
                            crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdVector2_t,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdVector2_t,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        6usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (nDeviceIndex, eFrameType, pFocalLength, pCenter, callback, object),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pFocalLength: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
        pCenter: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdVector2_t,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdVector2_t,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        ),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        3usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method.invoke_unchecked(self, (pFocalLength, pCenter, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        nDeviceIndex: u32,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        pFocalLength: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
        pCenter: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdVector2_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u32,
                            crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdVector2_t,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdVector2_t,
                            >,
                        ),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        4usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            4usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (nDeviceIndex, eFrameType, pFocalLength, pCenter),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraIntrinsics")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRTrackedCamera__GetCameraIntrinsics {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraProjection")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRTrackedCamera__GetCameraProjection {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraProjection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRTrackedCamera__GetCameraProjection {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRTrackedCamera/_GetCameraProjection";
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
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraProjection")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRTrackedCamera__GetCameraProjection {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraProjection")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRTrackedCamera__GetCameraProjection {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraProjection")]
impl crate::OVR::OpenVR::IVRTrackedCamera__GetCameraProjection {
    pub fn BeginInvoke(
        &mut self,
        nDeviceIndex: u32,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        flZNear: f32,
        flZFar: f32,
        pProjection: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdMatrix44_t>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u32,
                            crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                            f32,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdMatrix44_t,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        7usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        nDeviceIndex,
                        eFrameType,
                        flZNear,
                        flZFar,
                        pProjection,
                        callback,
                        object,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pProjection: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdMatrix44_t>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdMatrix44_t,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        ),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        2usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method.invoke_unchecked(self, (pProjection, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        nDeviceIndex: u32,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        flZNear: f32,
        flZFar: f32,
        pProjection: quest_hook::libil2cpp::ByRefMut<crate::OVR::OpenVR::HmdMatrix44_t>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u32,
                            crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                            f32,
                            f32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::HmdMatrix44_t,
                            >,
                        ),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        5usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (nDeviceIndex, eFrameType, flZNear, flZFar, pProjection),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetCameraProjection")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRTrackedCamera__GetCameraProjection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamFrameBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRTrackedCamera__GetVideoStreamFrameBuffer {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamFrameBuffer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamFrameBuffer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRTrackedCamera/_GetVideoStreamFrameBuffer";
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
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamFrameBuffer")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamFrameBuffer {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamFrameBuffer")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamFrameBuffer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamFrameBuffer")]
impl crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamFrameBuffer {
    pub fn BeginInvoke(
        &mut self,
        hTrackedCamera: u64,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        pFrameBuffer: crate::System::IntPtr,
        nFrameBufferSize: u32,
        pFrameHeader: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
        >,
        nFrameHeaderSize: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                            crate::System::IntPtr,
                            u32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
                            >,
                            u32,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        8usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        hTrackedCamera,
                        eFrameType,
                        pFrameBuffer,
                        nFrameBufferSize,
                        pFrameHeader,
                        nFrameHeaderSize,
                        callback,
                        object,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pFrameHeader: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        ),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        2usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method.invoke_unchecked(self, (pFrameHeader, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                            crate::System::IntPtr,
                            u32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
                            >,
                            u32,
                        ),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        6usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            6usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamFrameBuffer")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamFrameBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureD3D11")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRTrackedCamera__GetVideoStreamTextureD3D11 {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureD3D11")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureD3D11 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRTrackedCamera/_GetVideoStreamTextureD3D11";
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
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureD3D11")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureD3D11 {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureD3D11")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureD3D11 {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureD3D11")]
impl crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureD3D11 {
    pub fn BeginInvoke(
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
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
                            >,
                            u32,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        8usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        hTrackedCamera,
                        eFrameType,
                        pD3D11DeviceOrResource,
                        ppD3D11ShaderResourceView,
                        pFrameHeader,
                        nFrameHeaderSize,
                        callback,
                        object,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        ppD3D11ShaderResourceView: quest_hook::libil2cpp::ByRefMut<
            crate::System::IntPtr,
        >,
        pFrameHeader: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        ),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        3usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (ppD3D11ShaderResourceView, pFrameHeader, result),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::ByRefMut<crate::System::IntPtr>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
                            >,
                            u32,
                        ),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        6usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            6usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureD3D11")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureD3D11 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureGL")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRTrackedCamera__GetVideoStreamTextureGL {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureGL")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureGL {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRTrackedCamera/_GetVideoStreamTextureGL";
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
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureGL")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureGL {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureGL")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureGL {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureGL")]
impl crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureGL {
    pub fn BeginInvoke(
        &mut self,
        hTrackedCamera: u64,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        pglTextureId: quest_hook::libil2cpp::ByRefMut<u32>,
        pFrameHeader: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
        >,
        nFrameHeaderSize: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
                            >,
                            u32,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        7usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        hTrackedCamera,
                        eFrameType,
                        pglTextureId,
                        pFrameHeader,
                        nFrameHeaderSize,
                        callback,
                        object,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pglTextureId: quest_hook::libil2cpp::ByRefMut<u32>,
        pFrameHeader: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
        >,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        ),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        3usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method.invoke_unchecked(self, (pglTextureId, pFrameHeader, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        hTrackedCamera: u64,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        pglTextureId: quest_hook::libil2cpp::ByRefMut<u32>,
        pFrameHeader: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
        >,
        nFrameHeaderSize: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::CameraVideoStreamFrameHeader_t,
                            >,
                            u32,
                        ),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        5usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method
                .invoke_unchecked(
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureGL")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureGL {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureSize")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRTrackedCamera__GetVideoStreamTextureSize {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureSize")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureSize {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRTrackedCamera/_GetVideoStreamTextureSize";
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
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureSize")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureSize {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureSize")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureSize {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureSize")]
impl crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureSize {
    pub fn BeginInvoke(
        &mut self,
        nDeviceIndex: u32,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        pTextureBounds: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRTextureBounds_t,
        >,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u32,
                            crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::VRTextureBounds_t,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        7usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        nDeviceIndex,
                        eFrameType,
                        pTextureBounds,
                        pnWidth,
                        pnHeight,
                        callback,
                        object,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pTextureBounds: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRTextureBounds_t,
        >,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::VRTextureBounds_t,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        ),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        4usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method.invoke_unchecked(self, (pTextureBounds, pnWidth, pnHeight, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        nDeviceIndex: u32,
        eFrameType: crate::OVR::OpenVR::EVRTrackedCameraFrameType,
        pTextureBounds: quest_hook::libil2cpp::ByRefMut<
            crate::OVR::OpenVR::VRTextureBounds_t,
        >,
        pnWidth: quest_hook::libil2cpp::ByRefMut<u32>,
        pnHeight: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u32,
                            crate::OVR::OpenVR::EVRTrackedCameraFrameType,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::OVR::OpenVR::VRTextureBounds_t,
                            >,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                            quest_hook::libil2cpp::ByRefMut<u32>,
                        ),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        5usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (nDeviceIndex, eFrameType, pTextureBounds, pnWidth, pnHeight),
                )?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_GetVideoStreamTextureSize")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRTrackedCamera__GetVideoStreamTextureSize {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_HasCamera")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRTrackedCamera__HasCamera {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_HasCamera")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRTrackedCamera__HasCamera {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRTrackedCamera/_HasCamera";
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
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_HasCamera")]
impl std::ops::Deref for crate::OVR::OpenVR::IVRTrackedCamera__HasCamera {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_HasCamera")]
impl std::ops::DerefMut for crate::OVR::OpenVR::IVRTrackedCamera__HasCamera {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_HasCamera")]
impl crate::OVR::OpenVR::IVRTrackedCamera__HasCamera {
    pub fn BeginInvoke(
        &mut self,
        nDeviceIndex: u32,
        pHasCamera: quest_hook::libil2cpp::ByRefMut<bool>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u32,
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        4usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method.invoke_unchecked(self, (nDeviceIndex, pHasCamera, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        pHasCamera: quest_hook::libil2cpp::ByRefMut<bool>,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<bool>,
                            quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        ),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        2usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method.invoke_unchecked(self, (pHasCamera, result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        nDeviceIndex: u32,
        pHasCamera: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u32, quest_hook::libil2cpp::ByRefMut<bool>),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        2usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method.invoke_unchecked(self, (nDeviceIndex, pHasCamera))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_HasCamera")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRTrackedCamera__HasCamera {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_ReleaseVideoStreamTextureGL")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRTrackedCamera__ReleaseVideoStreamTextureGL {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_ReleaseVideoStreamTextureGL")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRTrackedCamera__ReleaseVideoStreamTextureGL {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRTrackedCamera/_ReleaseVideoStreamTextureGL";
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
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_ReleaseVideoStreamTextureGL")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRTrackedCamera__ReleaseVideoStreamTextureGL {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_ReleaseVideoStreamTextureGL")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRTrackedCamera__ReleaseVideoStreamTextureGL {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_ReleaseVideoStreamTextureGL")]
impl crate::OVR::OpenVR::IVRTrackedCamera__ReleaseVideoStreamTextureGL {
    pub fn BeginInvoke(
        &mut self,
        hTrackedCamera: u64,
        glTextureId: u32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            u32,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        4usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method
                .invoke_unchecked(self, (hTrackedCamera, glTextureId, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        1usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method.invoke_unchecked(self, (result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        hTrackedCamera: u64,
        glTextureId: u32,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64, u32),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        2usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method.invoke_unchecked(self, (hTrackedCamera, glTextureId))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_ReleaseVideoStreamTextureGL")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRTrackedCamera__ReleaseVideoStreamTextureGL {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_ReleaseVideoStreamingService")]
#[repr(C)]
#[derive(Debug)]
pub struct IVRTrackedCamera__ReleaseVideoStreamingService {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_ReleaseVideoStreamingService")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::IVRTrackedCamera__ReleaseVideoStreamingService {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "IVRTrackedCamera/_ReleaseVideoStreamingService";
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
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_ReleaseVideoStreamingService")]
impl std::ops::Deref
for crate::OVR::OpenVR::IVRTrackedCamera__ReleaseVideoStreamingService {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_ReleaseVideoStreamingService")]
impl std::ops::DerefMut
for crate::OVR::OpenVR::IVRTrackedCamera__ReleaseVideoStreamingService {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_ReleaseVideoStreamingService")]
impl crate::OVR::OpenVR::IVRTrackedCamera__ReleaseVideoStreamingService {
    pub fn BeginInvoke(
        &mut self,
        hTrackedCamera: u64,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            u64,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        3usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            method.invoke_unchecked(self, (hTrackedCamera, callback, object))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        1usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method.invoke_unchecked(self, (result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        hTrackedCamera: u64,
    ) -> quest_hook::libil2cpp::Result<crate::OVR::OpenVR::EVRTrackedCameraError> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u64),
                        crate::OVR::OpenVR::EVRTrackedCameraError,
                        1usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::OVR::OpenVR::EVRTrackedCameraError = unsafe {
            method.invoke_unchecked(self, (hTrackedCamera))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVR+OpenVR+IVRTrackedCamera+_ReleaseVideoStreamingService")]
impl quest_hook::libil2cpp::ObjectType
for crate::OVR::OpenVR::IVRTrackedCamera__ReleaseVideoStreamingService {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
