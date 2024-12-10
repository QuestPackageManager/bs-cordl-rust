#[cfg(feature = "UnityEngine+Rendering+CameraProperties")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CameraProperties {
    pub screenRect: crate::UnityEngine::Rect,
    pub viewDir: crate::UnityEngine::Vector3,
    pub projectionNear: f32,
    pub projectionFar: f32,
    pub cameraNear: f32,
    pub cameraFar: f32,
    pub cameraAspect: f32,
    pub cameraToWorld: crate::UnityEngine::Matrix4x4,
    pub actualWorldToClip: crate::UnityEngine::Matrix4x4,
    pub cameraClipToWorld: crate::UnityEngine::Matrix4x4,
    pub cameraWorldToClip: crate::UnityEngine::Matrix4x4,
    pub implicitProjection: crate::UnityEngine::Matrix4x4,
    pub stereoWorldToClipLeft: crate::UnityEngine::Matrix4x4,
    pub stereoWorldToClipRight: crate::UnityEngine::Matrix4x4,
    pub worldToCamera: crate::UnityEngine::Matrix4x4,
    pub up: crate::UnityEngine::Vector3,
    pub right: crate::UnityEngine::Vector3,
    pub transformDirection: crate::UnityEngine::Vector3,
    pub cameraEuler: crate::UnityEngine::Vector3,
    pub velocity: crate::UnityEngine::Vector3,
    pub farPlaneWorldSpaceLength: f32,
    pub rendererCount: u32,
    pub m_ShadowCullPlanes: crate::UnityEngine::Rendering::CameraProperties__m_ShadowCullPlanes_e__FixedBuffer,
    pub m_CameraCullPlanes: crate::UnityEngine::Rendering::CameraProperties__m_CameraCullPlanes_e__FixedBuffer,
    pub baseFarDistance: f32,
    pub shadowCullCenter: crate::UnityEngine::Vector3,
    pub layerCullDistances: crate::UnityEngine::Rendering::CameraProperties__layerCullDistances_e__FixedBuffer,
    pub layerCullSpherical: i32,
    pub coreCameraValues: crate::UnityEngine::Rendering::CoreCameraValues,
    pub cameraType: u32,
    pub projectionIsOblique: i32,
    pub isImplicitProjectionMatrix: i32,
}
#[cfg(feature = "UnityEngine+Rendering+CameraProperties")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::CameraProperties =>
    "UnityEngine.Rendering"."CameraProperties"
);
#[cfg(feature = "UnityEngine+Rendering+CameraProperties")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::CameraProperties {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+CameraProperties")]
impl crate::UnityEngine::Rendering::CameraProperties {
    #[cfg(
        feature = "UnityEngine+Rendering+CameraProperties+_layerCullDistances_e__FixedBuffer"
    )]
    pub type _layerCullDistances_e__FixedBuffer = crate::UnityEngine::Rendering::CameraProperties__layerCullDistances_e__FixedBuffer;
    #[cfg(
        feature = "UnityEngine+Rendering+CameraProperties+_m_CameraCullPlanes_e__FixedBuffer"
    )]
    pub type _m_CameraCullPlanes_e__FixedBuffer = crate::UnityEngine::Rendering::CameraProperties__m_CameraCullPlanes_e__FixedBuffer;
    #[cfg(
        feature = "UnityEngine+Rendering+CameraProperties+_m_ShadowCullPlanes_e__FixedBuffer"
    )]
    pub type _m_ShadowCullPlanes_e__FixedBuffer = crate::UnityEngine::Rendering::CameraProperties__m_ShadowCullPlanes_e__FixedBuffer;
    pub fn Equals_CameraProperties0(
        &mut self,
        other: crate::UnityEngine::Rendering::CameraProperties,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCameraCullingPlane(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Plane> {
        let __cordl_ret: crate::UnityEngine::Plane = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetCameraCullingPlane",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetShadowCullingPlane(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Plane> {
        let __cordl_ret: crate::UnityEngine::Plane = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetShadowCullingPlane",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+CameraProperties")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::Rendering::CameraProperties>>
for crate::UnityEngine::Rendering::CameraProperties {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::Rendering::CameraProperties> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+CameraProperties")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::Rendering::CameraProperties>>
for crate::UnityEngine::Rendering::CameraProperties {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::Rendering::CameraProperties,
    > {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+CameraProperties+_layerCullDistances_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CameraProperties__layerCullDistances_e__FixedBuffer {
    pub FixedElementField: f32,
}
#[cfg(
    feature = "UnityEngine+Rendering+CameraProperties+_layerCullDistances_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Rendering::CameraProperties__layerCullDistances_e__FixedBuffer =>
    "UnityEngine.Rendering"."CameraProperties/<layerCullDistances>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+Rendering+CameraProperties+_layerCullDistances_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::CameraProperties__layerCullDistances_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+CameraProperties+_layerCullDistances_e__FixedBuffer"
)]
impl crate::UnityEngine::Rendering::CameraProperties__layerCullDistances_e__FixedBuffer {}
#[cfg(
    feature = "UnityEngine+Rendering+CameraProperties+_m_CameraCullPlanes_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CameraProperties__m_CameraCullPlanes_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "UnityEngine+Rendering+CameraProperties+_m_CameraCullPlanes_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Rendering::CameraProperties__m_CameraCullPlanes_e__FixedBuffer =>
    "UnityEngine.Rendering"."CameraProperties/<m_CameraCullPlanes>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+Rendering+CameraProperties+_m_CameraCullPlanes_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::CameraProperties__m_CameraCullPlanes_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+CameraProperties+_m_CameraCullPlanes_e__FixedBuffer"
)]
impl crate::UnityEngine::Rendering::CameraProperties__m_CameraCullPlanes_e__FixedBuffer {}
#[cfg(
    feature = "UnityEngine+Rendering+CameraProperties+_m_ShadowCullPlanes_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CameraProperties__m_ShadowCullPlanes_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "UnityEngine+Rendering+CameraProperties+_m_ShadowCullPlanes_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Rendering::CameraProperties__m_ShadowCullPlanes_e__FixedBuffer =>
    "UnityEngine.Rendering"."CameraProperties/<m_ShadowCullPlanes>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+Rendering+CameraProperties+_m_ShadowCullPlanes_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::CameraProperties__m_ShadowCullPlanes_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+CameraProperties+_m_ShadowCullPlanes_e__FixedBuffer"
)]
impl crate::UnityEngine::Rendering::CameraProperties__m_ShadowCullPlanes_e__FixedBuffer {}
