#[cfg(feature = "OVR+OpenVR+EVRButtonId")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EVRButtonId {
    #[default]
    k_EButton_A = 7i32,
    k_EButton_ApplicationMenu = 1i32,
    k_EButton_Axis0 = 32i32,
    k_EButton_Axis1 = 33i32,
    k_EButton_Axis2 = 34i32,
    k_EButton_Axis3 = 35i32,
    k_EButton_Axis4 = 36i32,
    k_EButton_DPad_Down = 6i32,
    k_EButton_DPad_Left = 3i32,
    k_EButton_DPad_Right = 5i32,
    k_EButton_DPad_Up = 4i32,
    k_EButton_Dashboard_Back = 2i32,
    k_EButton_Max = 64i32,
    k_EButton_ProximitySensor = 31i32,
    k_EButton_System = 0i32,
}
#[cfg(feature = "OVR+OpenVR+EVRButtonId")]
unsafe impl quest_hook::libil2cpp::Type for crate::OVR::OpenVR::EVRButtonId {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "EVRButtonId";
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
#[cfg(feature = "OVR+OpenVR+EVRButtonId")]
unsafe impl quest_hook::libil2cpp::Argument for crate::OVR::OpenVR::EVRButtonId {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVR+OpenVR+EVRButtonId")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::OVR::OpenVR::EVRButtonId {
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
#[cfg(feature = "OVR+OpenVR+EVRButtonId")]
unsafe impl quest_hook::libil2cpp::Returned for crate::OVR::OpenVR::EVRButtonId {
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
#[cfg(feature = "OVR+OpenVR+EVRButtonId")]
unsafe impl quest_hook::libil2cpp::Return for crate::OVR::OpenVR::EVRButtonId {
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
