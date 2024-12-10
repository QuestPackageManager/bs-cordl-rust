#[cfg(feature = "UnityEngine+Rendering+ScriptableCullingParameters")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ScriptableCullingParameters {
    pub m_IsOrthographic: i32,
    pub m_LODParameters: crate::UnityEngine::Rendering::LODParameters,
    pub m_CullingPlanes: crate::UnityEngine::Rendering::ScriptableCullingParameters__m_CullingPlanes_e__FixedBuffer,
    pub m_CullingPlaneCount: i32,
    pub m_CullingMask: u32,
    pub m_SceneMask: u64,
    pub m_ViewID: u64,
    pub m_LayerFarCullDistances: crate::UnityEngine::Rendering::ScriptableCullingParameters__m_LayerFarCullDistances_e__FixedBuffer,
    pub m_LayerCull: i32,
    pub m_CullingMatrix: crate::UnityEngine::Matrix4x4,
    pub m_Origin: crate::UnityEngine::Vector3,
    pub m_ShadowDistance: f32,
    pub m_ShadowNearPlaneOffset: f32,
    pub m_CullingOptions: crate::UnityEngine::Rendering::CullingOptions,
    pub m_ReflectionProbeSortingCriteria: crate::UnityEngine::Rendering::ReflectionProbeSortingCriteria,
    pub m_CameraProperties: crate::UnityEngine::Rendering::CameraProperties,
    pub m_AccurateOcclusionThreshold: f32,
    pub m_MaximumPortalCullingJobs: i32,
    pub m_StereoViewMatrix: crate::UnityEngine::Matrix4x4,
    pub m_StereoProjectionMatrix: crate::UnityEngine::Matrix4x4,
    pub m_StereoSeparationDistance: f32,
    pub m_maximumVisibleLights: i32,
    pub m_ConservativeEnclosingSphere: bool,
    pub m_NumIterationsEnclosingSphere: i32,
}
#[cfg(feature = "UnityEngine+Rendering+ScriptableCullingParameters")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Rendering::ScriptableCullingParameters => "UnityEngine.Rendering"
    ."ScriptableCullingParameters"
);
#[cfg(feature = "UnityEngine+Rendering+ScriptableCullingParameters")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::ScriptableCullingParameters {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+ScriptableCullingParameters")]
impl crate::UnityEngine::Rendering::ScriptableCullingParameters {
    #[cfg(
        feature = "UnityEngine+Rendering+ScriptableCullingParameters+_m_CullingPlanes_e__FixedBuffer"
    )]
    pub type _m_CullingPlanes_e__FixedBuffer = crate::UnityEngine::Rendering::ScriptableCullingParameters__m_CullingPlanes_e__FixedBuffer;
    #[cfg(
        feature = "UnityEngine+Rendering+ScriptableCullingParameters+_m_LayerFarCullDistances_e__FixedBuffer"
    )]
    pub type _m_LayerFarCullDistances_e__FixedBuffer = crate::UnityEngine::Rendering::ScriptableCullingParameters__m_LayerFarCullDistances_e__FixedBuffer;
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
    pub fn Equals_ScriptableCullingParameters0(
        &mut self,
        other: crate::UnityEngine::Rendering::ScriptableCullingParameters,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCullingPlane(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Plane> {
        let __cordl_ret: crate::UnityEngine::Plane = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetCullingPlane",
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
    pub fn GetLayerCullingDistance(
        &mut self,
        layerIndex: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetLayerCullingDistance",
            (layerIndex),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_cullingPlaneCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_cullingPlaneCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+ScriptableCullingParameters")]
impl AsRef<
    crate::System::IEquatable_1<
        crate::UnityEngine::Rendering::ScriptableCullingParameters,
    >,
> for crate::UnityEngine::Rendering::ScriptableCullingParameters {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        crate::UnityEngine::Rendering::ScriptableCullingParameters,
    > {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Rendering+ScriptableCullingParameters")]
impl AsMut<
    crate::System::IEquatable_1<
        crate::UnityEngine::Rendering::ScriptableCullingParameters,
    >,
> for crate::UnityEngine::Rendering::ScriptableCullingParameters {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::Rendering::ScriptableCullingParameters,
    > {
        todo!()
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+ScriptableCullingParameters+_m_CullingPlanes_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ScriptableCullingParameters__m_CullingPlanes_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "UnityEngine+Rendering+ScriptableCullingParameters+_m_CullingPlanes_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Rendering::ScriptableCullingParameters__m_CullingPlanes_e__FixedBuffer
    => "UnityEngine.Rendering"
    ."ScriptableCullingParameters/<m_CullingPlanes>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+Rendering+ScriptableCullingParameters+_m_CullingPlanes_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::ScriptableCullingParameters__m_CullingPlanes_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+ScriptableCullingParameters+_m_CullingPlanes_e__FixedBuffer"
)]
impl crate::UnityEngine::Rendering::ScriptableCullingParameters__m_CullingPlanes_e__FixedBuffer {}
#[cfg(
    feature = "UnityEngine+Rendering+ScriptableCullingParameters+_m_LayerFarCullDistances_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ScriptableCullingParameters__m_LayerFarCullDistances_e__FixedBuffer {
    pub FixedElementField: f32,
}
#[cfg(
    feature = "UnityEngine+Rendering+ScriptableCullingParameters+_m_LayerFarCullDistances_e__FixedBuffer"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Rendering::ScriptableCullingParameters__m_LayerFarCullDistances_e__FixedBuffer
    => "UnityEngine.Rendering"
    ."ScriptableCullingParameters/<m_LayerFarCullDistances>e__FixedBuffer"
);
#[cfg(
    feature = "UnityEngine+Rendering+ScriptableCullingParameters+_m_LayerFarCullDistances_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::ScriptableCullingParameters__m_LayerFarCullDistances_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+ScriptableCullingParameters+_m_LayerFarCullDistances_e__FixedBuffer"
)]
impl crate::UnityEngine::Rendering::ScriptableCullingParameters__m_LayerFarCullDistances_e__FixedBuffer {}
