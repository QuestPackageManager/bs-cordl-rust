#[cfg(feature = "OVR+OpenVR+EVRApplicationProperty")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRApplicationProperty {
    #[default]
    ActionManifestURL_String = 54i32,
    Arguments_String = 14i32,
    BinaryPath_String = 13i32,
    Description_String = 50i32,
    ImagePath_String = 52i32,
    IsDashboardOverlay_Bool = 60i32,
    IsInstanced_Bool = 62i32,
    IsInternal_Bool = 63i32,
    IsTemplate_Bool = 61i32,
    LastLaunchTime_Uint64 = 70i32,
    LaunchType_String = 11i32,
    Name_String = 0i32,
    NewsURL_String = 51i32,
    Source_String = 53i32,
    URL_String = 15i32,
    WantsCompositorPauseInStandby_Bool = 64i32,
    WorkingDirectory_String = 12i32,
}
#[cfg(feature = "OVR+OpenVR+EVRApplicationProperty")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::EVRApplicationProperty {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "EVRApplicationProperty";
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
#[cfg(feature = "OVR+OpenVR+EVRApplicationProperty")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::OVR::OpenVR::EVRApplicationProperty {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVR+OpenVR+EVRApplicationProperty")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::OVR::OpenVR::EVRApplicationProperty {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OVR+OpenVR+EVRApplicationProperty")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::OVR::OpenVR::EVRApplicationProperty {
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
#[cfg(feature = "OVR+OpenVR+EVRApplicationProperty")]
unsafe impl quest_hook::libil2cpp::Return
for crate::OVR::OpenVR::EVRApplicationProperty {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
