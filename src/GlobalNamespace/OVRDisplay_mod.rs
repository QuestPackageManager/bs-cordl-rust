#[cfg(feature = "OVRDisplay+EyeFov")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRDisplay_EyeFov {
    pub UpFov: f32,
    pub DownFov: f32,
    pub LeftFov: f32,
    pub RightFov: f32,
}
#[cfg(feature = "OVRDisplay+EyeFov")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRDisplay_EyeFov => ""
    ."OVRDisplay/EyeFov"
);
#[cfg(feature = "OVRDisplay+EyeFov")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDisplay_EyeFov {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDisplay+EyeFov")]
impl crate::GlobalNamespace::OVRDisplay_EyeFov {}
#[cfg(feature = "OVRDisplay+EyeRenderDesc")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRDisplay_EyeRenderDesc {
    pub resolution: crate::UnityEngine::Vector2,
    pub fov: crate::UnityEngine::Vector2,
    pub fullFov: crate::GlobalNamespace::OVRDisplay_EyeFov,
}
#[cfg(feature = "OVRDisplay+EyeRenderDesc")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRDisplay_EyeRenderDesc => ""
    ."OVRDisplay/EyeRenderDesc"
);
#[cfg(feature = "OVRDisplay+EyeRenderDesc")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDisplay_EyeRenderDesc {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDisplay+EyeRenderDesc")]
impl crate::GlobalNamespace::OVRDisplay_EyeRenderDesc {}
#[cfg(feature = "OVRDisplay+LatencyData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRDisplay_LatencyData {
    pub render: f32,
    pub timeWarp: f32,
    pub postPresent: f32,
    pub renderError: f32,
    pub timeWarpError: f32,
}
#[cfg(feature = "OVRDisplay+LatencyData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRDisplay_LatencyData => ""
    ."OVRDisplay/LatencyData"
);
#[cfg(feature = "OVRDisplay+LatencyData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRDisplay_LatencyData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRDisplay+LatencyData")]
impl crate::GlobalNamespace::OVRDisplay_LatencyData {}
#[cfg(feature = "OVRDisplay")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRDisplay {
    __cordl_parent: crate::System::Object,
    pub needsConfigureTexture: bool,
    pub eyeDescs: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::OVRDisplay_EyeRenderDesc,
    >,
    pub recenterRequested: bool,
    pub recenterRequestedFrameCount: i32,
    pub localTrackingSpaceRecenterCount: i32,
    pub RecenteredPose: *mut crate::System::Action,
}
#[cfg(feature = "OVRDisplay")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRDisplay => ""."OVRDisplay"
);
#[cfg(feature = "OVRDisplay")]
impl std::ops::Deref for crate::GlobalNamespace::OVRDisplay {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRDisplay")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRDisplay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRDisplay")]
impl crate::GlobalNamespace::OVRDisplay {
    #[cfg(feature = "OVRDisplay+EyeFov")]
    pub type EyeFov = crate::GlobalNamespace::OVRDisplay_EyeFov;
    #[cfg(feature = "OVRDisplay+EyeRenderDesc")]
    pub type EyeRenderDesc = crate::GlobalNamespace::OVRDisplay_EyeRenderDesc;
    #[cfg(feature = "OVRDisplay+LatencyData")]
    pub type LatencyData = crate::GlobalNamespace::OVRDisplay_LatencyData;
    pub fn ConfigureEyeDesc(
        &mut self,
        eye: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConfigureEyeDesc", (eye))?;
        Ok(__cordl_ret)
    }
    pub fn GetEyeRenderDesc(
        &mut self,
        eye: crate::UnityEngine::XR::XRNode,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRDisplay_EyeRenderDesc,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRDisplay_EyeRenderDesc = __cordl_object
            .invoke("GetEyeRenderDesc", (eye))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RecenterPose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecenterPose", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateTextures(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateTextures", ())?;
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
    pub fn add_RecenteredPose(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_RecenteredPose", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_acceleration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_acceleration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_angularAcceleration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_angularAcceleration", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_angularVelocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_angularVelocity", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_appFramerate(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_appFramerate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_displayFrequenciesAvailable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<f32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<f32> = __cordl_object
            .invoke("get_displayFrequenciesAvailable", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_displayFrequency(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_displayFrequency", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_latency(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRDisplay_LatencyData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRDisplay_LatencyData = __cordl_object
            .invoke("get_latency", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_recommendedMSAALevel(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_recommendedMSAALevel", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_velocity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_velocity", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_RecenteredPose(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_RecenteredPose", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_displayFrequency(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_displayFrequency", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRDisplay")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRDisplay {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
