#[cfg(feature = "cordl_class_OVR+OpenVR+VRVulkanTextureData_t")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct VRVulkanTextureData_t {
    pub m_nImage: u64,
    pub m_pDevice: crate::System::IntPtr,
    pub m_pPhysicalDevice: crate::System::IntPtr,
    pub m_pInstance: crate::System::IntPtr,
    pub m_pQueue: crate::System::IntPtr,
    pub m_nQueueFamilyIndex: u32,
    pub m_nWidth: u32,
    pub m_nHeight: u32,
    pub m_nFormat: u32,
    pub m_nSampleCount: u32,
}
#[cfg(feature = "cordl_class_OVR+OpenVR+VRVulkanTextureData_t")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::VRVulkanTextureData_t {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "VRVulkanTextureData_t";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+VRVulkanTextureData_t")]
unsafe impl quest_hook::libil2cpp::Argument for crate::OVR::OpenVR::VRVulkanTextureData_t {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+VRVulkanTextureData_t")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::OVR::OpenVR::VRVulkanTextureData_t {
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
#[cfg(feature = "cordl_class_OVR+OpenVR+VRVulkanTextureData_t")]
unsafe impl quest_hook::libil2cpp::Returned for crate::OVR::OpenVR::VRVulkanTextureData_t {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_OVR+OpenVR+VRVulkanTextureData_t")]
unsafe impl quest_hook::libil2cpp::Return for crate::OVR::OpenVR::VRVulkanTextureData_t {
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
#[cfg(feature = "cordl_class_OVR+OpenVR+VRVulkanTextureData_t")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::OVR::OpenVR::VRVulkanTextureData_t {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVR+OpenVR+VRVulkanTextureData_t")]
impl crate::OVR::OpenVR::VRVulkanTextureData_t {}
