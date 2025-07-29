#[cfg(feature = "cordl_class_System+Security+AccessControl+AceType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AceType {
    #[default]
    AccessAllowed = 0u8,
    AccessAllowedCallback = 9u8,
    AccessAllowedCallbackObject = 11u8,
    AccessAllowedCompound = 4u8,
    AccessAllowedObject = 5u8,
    AccessDenied = 1u8,
    AccessDeniedCallback = 10u8,
    AccessDeniedCallbackObject = 12u8,
    AccessDeniedObject = 6u8,
    MaxDefinedAceType = 16u8,
    SystemAlarm = 3u8,
    SystemAlarmCallback = 14u8,
    SystemAlarmObject = 8u8,
    SystemAudit = 2u8,
    SystemAuditCallback = 13u8,
    SystemAuditCallbackObject = 15u8,
    SystemAuditObject = 7u8,
}
#[cfg(feature = "cordl_class_System+Security+AccessControl+AceType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::AccessControl::AceType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Security.AccessControl";
    const CLASS_NAME: &'static str = "AceType";
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
#[cfg(feature = "cordl_class_System+Security+AccessControl+AceType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Security::AccessControl::AceType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Security+AccessControl+AceType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Security::AccessControl::AceType {
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
#[cfg(feature = "cordl_class_System+Security+AccessControl+AceType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Security::AccessControl::AceType {
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
#[cfg(feature = "cordl_class_System+Security+AccessControl+AceType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Security::AccessControl::AceType {
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
