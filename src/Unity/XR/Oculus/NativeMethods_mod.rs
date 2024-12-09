#[cfg(feature = "Unity+XR+Oculus+NativeMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeMethods {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::NativeMethods =>
    "Unity.XR.Oculus"."NativeMethods"
);
#[cfg(feature = "Unity+XR+Oculus+NativeMethods")]
impl std::ops::Deref for crate::Unity::XR::Oculus::NativeMethods {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::NativeMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods")]
impl crate::Unity::XR::Oculus::NativeMethods {
    #[cfg(feature = "Unity+XR+Oculus+NativeMethods+Internal")]
    pub type Internal = crate::Unity::XR::Oculus::NativeMethods_Internal;
    #[cfg(feature = "Unity+XR+Oculus+NativeMethods+UserDefinedSettings")]
    pub type UserDefinedSettings = crate::Unity::XR::Oculus::NativeMethods_UserDefinedSettings;
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::XR::Oculus::NativeMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods+Internal")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeMethods_Internal {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods+Internal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::XR::Oculus::NativeMethods_Internal =>
    "Unity.XR.Oculus"."NativeMethods/Internal"
);
#[cfg(feature = "Unity+XR+Oculus+NativeMethods+Internal")]
impl std::ops::Deref for crate::Unity::XR::Oculus::NativeMethods_Internal {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods+Internal")]
impl std::ops::DerefMut for crate::Unity::XR::Oculus::NativeMethods_Internal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods+Internal")]
impl crate::Unity::XR::Oculus::NativeMethods_Internal {}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods+Internal")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::XR::Oculus::NativeMethods_Internal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods+UserDefinedSettings")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NativeMethods_UserDefinedSettings {
    pub sharedDepthBuffer: u16,
    pub dashSupport: u16,
    pub stereoRenderingMode: u16,
    pub colorSpace: u16,
    pub lowOverheadMode: u16,
    pub optimizeBufferDiscards: u16,
    pub phaseSync: u16,
    pub symmetricProjection: u16,
    pub subsampledLayout: u16,
    pub lateLatching: u16,
    pub lateLatchingDebug: u16,
    pub enableTrackingOriginStageMode: u16,
    pub spaceWarp: u16,
    pub depthSubmission: u16,
    pub foveatedRenderingMethod: u16,
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods+UserDefinedSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::XR::Oculus::NativeMethods_UserDefinedSettings => "Unity.XR.Oculus"
    ."NativeMethods/UserDefinedSettings"
);
#[cfg(feature = "Unity+XR+Oculus+NativeMethods+UserDefinedSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::XR::Oculus::NativeMethods_UserDefinedSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+XR+Oculus+NativeMethods+UserDefinedSettings")]
impl crate::Unity::XR::Oculus::NativeMethods_UserDefinedSettings {}
