#[cfg(feature = "Oculus+Platform+MultiplayerErrorErrorKey")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MultiplayerErrorErrorKey {
    #[default]
    DestinationUnavailable = 1i32,
    DlcRequired = 2i32,
    General = 3i32,
    GroupFull = 4i32,
    InviterNotJoinable = 5i32,
    LevelNotHighEnough = 6i32,
    LevelNotUnlocked = 7i32,
    NetworkTimeout = 8i32,
    NoLongerAvailable = 9i32,
    TutorialRequired = 11i32,
    Unknown = 0i32,
    UpdateRequired = 10i32,
}
#[cfg(feature = "Oculus+Platform+MultiplayerErrorErrorKey")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Oculus::Platform::MultiplayerErrorErrorKey {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "MultiplayerErrorErrorKey";
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
#[cfg(feature = "Oculus+Platform+MultiplayerErrorErrorKey")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Oculus::Platform::MultiplayerErrorErrorKey {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Oculus+Platform+MultiplayerErrorErrorKey")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Oculus::Platform::MultiplayerErrorErrorKey {
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
#[cfg(feature = "Oculus+Platform+MultiplayerErrorErrorKey")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Oculus::Platform::MultiplayerErrorErrorKey {
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
#[cfg(feature = "Oculus+Platform+MultiplayerErrorErrorKey")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Oculus::Platform::MultiplayerErrorErrorKey {
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
