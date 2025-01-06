#[cfg(feature = "UnityEngine+InputSystem+XR+XRFeatureDescriptor")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XRFeatureDescriptor {
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub usageHints: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::InputSystem::XR::UsageHint,
        >,
    >,
    pub featureType: crate::UnityEngine::InputSystem::XR::FeatureType,
    pub customSize: u32,
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRFeatureDescriptor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::XR::XRFeatureDescriptor => "UnityEngine.InputSystem.XR"
    ."XRFeatureDescriptor"
);
#[cfg(feature = "UnityEngine+InputSystem+XR+XRFeatureDescriptor")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::XR::XRFeatureDescriptor {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+XR+XRFeatureDescriptor")]
impl crate::UnityEngine::InputSystem::XR::XRFeatureDescriptor {}
