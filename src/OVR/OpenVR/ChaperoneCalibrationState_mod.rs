#[cfg(feature = "OVR+OpenVR+ChaperoneCalibrationState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChaperoneCalibrationState {
    #[default]
    Error = 200i32,
    Error_BaseStationConflict = 202i32,
    Error_BaseStationUninitialized = 201i32,
    Error_CollisionBoundsInvalid = 204i32,
    Error_PlayAreaInvalid = 203i32,
    OK = 1i32,
    Warning = 100i32,
    Warning_BaseStationMayHaveMoved = 101i32,
    Warning_BaseStationRemoved = 102i32,
    Warning_SeatedBoundsInvalid = 103i32,
}
#[cfg(feature = "OVR+OpenVR+ChaperoneCalibrationState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OVR::OpenVR::ChaperoneCalibrationState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OVR.OpenVR";
    const CLASS_NAME: &'static str = "ChaperoneCalibrationState";
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
#[cfg(feature = "OVR+OpenVR+ChaperoneCalibrationState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::OVR::OpenVR::ChaperoneCalibrationState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVR+OpenVR+ChaperoneCalibrationState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::OVR::OpenVR::ChaperoneCalibrationState {
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
#[cfg(feature = "OVR+OpenVR+ChaperoneCalibrationState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::OVR::OpenVR::ChaperoneCalibrationState {
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
#[cfg(feature = "OVR+OpenVR+ChaperoneCalibrationState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::OVR::OpenVR::ChaperoneCalibrationState {
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
