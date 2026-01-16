#[cfg(feature = "cordl_class_UnityEngine+Rendering+CameraProperties")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
    pub useInteractiveLightBakingData: bool,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CameraProperties")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::CameraProperties {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CameraProperties";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CameraProperties")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::CameraProperties {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CameraProperties")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::CameraProperties {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CameraProperties")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::CameraProperties {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CameraProperties")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::CameraProperties {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+CameraProperties")]
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
    pub const k_NumLayers: i32 = 32i32;
    pub const k_PlaneCount: i32 = 6i32;
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::CameraProperties),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (other))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        bool,
                        1usize,
                    >("Equals")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Equals",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (obj))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCameraCullingPlane(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Plane> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        crate::UnityEngine::Plane,
                        1usize,
                    >("GetCameraCullingPlane")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetCameraCullingPlane", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Plane = unsafe {
            cordl_method_info.invoke_unchecked(self, (index))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("GetHashCode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetHashCode", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetShadowCullingPlane(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Plane> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        crate::UnityEngine::Plane,
                        1usize,
                    >("GetShadowCullingPlane")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetShadowCullingPlane", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Plane = unsafe {
            cordl_method_info.invoke_unchecked(self, (index))?
        };
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
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_layerCullDistances_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CameraProperties__layerCullDistances_e__FixedBuffer {
    pub FixedElementField: f32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_layerCullDistances_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::CameraProperties__layerCullDistances_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CameraProperties/<layerCullDistances>e__FixedBuffer";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_layerCullDistances_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::CameraProperties__layerCullDistances_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_layerCullDistances_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::CameraProperties__layerCullDistances_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_layerCullDistances_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::CameraProperties__layerCullDistances_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_layerCullDistances_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::CameraProperties__layerCullDistances_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_layerCullDistances_e__FixedBuffer"
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
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_m_CameraCullPlanes_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CameraProperties__m_CameraCullPlanes_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_m_CameraCullPlanes_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::CameraProperties__m_CameraCullPlanes_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CameraProperties/<m_CameraCullPlanes>e__FixedBuffer";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_m_CameraCullPlanes_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::CameraProperties__m_CameraCullPlanes_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_m_CameraCullPlanes_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::CameraProperties__m_CameraCullPlanes_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_m_CameraCullPlanes_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::CameraProperties__m_CameraCullPlanes_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_m_CameraCullPlanes_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::CameraProperties__m_CameraCullPlanes_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_m_CameraCullPlanes_e__FixedBuffer"
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
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_m_ShadowCullPlanes_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CameraProperties__m_ShadowCullPlanes_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_m_ShadowCullPlanes_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::CameraProperties__m_ShadowCullPlanes_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "CameraProperties/<m_ShadowCullPlanes>e__FixedBuffer";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_m_ShadowCullPlanes_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::CameraProperties__m_ShadowCullPlanes_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_m_ShadowCullPlanes_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::CameraProperties__m_ShadowCullPlanes_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_m_ShadowCullPlanes_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::CameraProperties__m_ShadowCullPlanes_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_m_ShadowCullPlanes_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::CameraProperties__m_ShadowCullPlanes_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+CameraProperties+_m_ShadowCullPlanes_e__FixedBuffer"
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
