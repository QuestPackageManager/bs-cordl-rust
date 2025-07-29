#[cfg(feature = "cordl_class_System+Net+NetworkInformation+AixStructs+ifreq_mtu")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ifreq_mtu {
    padding: quest_hook::libil2cpp::ValueTypePadding<20usize>,
}
#[cfg(feature = "cordl_class_System+Net+NetworkInformation+AixStructs+ifreq_mtu")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net.NetworkInformation.AixStructs";
    const CLASS_NAME: &'static str = "ifreq_mtu";
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
#[cfg(feature = "cordl_class_System+Net+NetworkInformation+AixStructs+ifreq_mtu")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Net+NetworkInformation+AixStructs+ifreq_mtu")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu {
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
#[cfg(feature = "cordl_class_System+Net+NetworkInformation+AixStructs+ifreq_mtu")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu {
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
#[cfg(feature = "cordl_class_System+Net+NetworkInformation+AixStructs+ifreq_mtu")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu {
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
#[cfg(feature = "cordl_class_System+Net+NetworkInformation+AixStructs+ifreq_mtu")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+AixStructs+ifreq_mtu")]
impl crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu {
    #[cfg(
        feature = "System+Net+NetworkInformation+AixStructs+ifreq_mtu+_ifr_name_e__FixedBuffer"
    )]
    pub type _ifr_name_e__FixedBuffer = crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu__ifr_name_e__FixedBuffer;
}
#[cfg(
    feature = "cordl_class_System+Net+NetworkInformation+AixStructs+ifreq_mtu+_ifr_name_e__FixedBuffer"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ifreq_mtu__ifr_name_e__FixedBuffer {
    pub FixedElementField: u8,
}
#[cfg(
    feature = "cordl_class_System+Net+NetworkInformation+AixStructs+ifreq_mtu+_ifr_name_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu__ifr_name_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net.NetworkInformation.AixStructs";
    const CLASS_NAME: &'static str = "ifreq_mtu/<ifr_name>e__FixedBuffer";
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
#[cfg(
    feature = "cordl_class_System+Net+NetworkInformation+AixStructs+ifreq_mtu+_ifr_name_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu__ifr_name_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_System+Net+NetworkInformation+AixStructs+ifreq_mtu+_ifr_name_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu__ifr_name_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_System+Net+NetworkInformation+AixStructs+ifreq_mtu+_ifr_name_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu__ifr_name_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_System+Net+NetworkInformation+AixStructs+ifreq_mtu+_ifr_name_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu__ifr_name_e__FixedBuffer {
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
#[cfg(
    feature = "cordl_class_System+Net+NetworkInformation+AixStructs+ifreq_mtu+_ifr_name_e__FixedBuffer"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu__ifr_name_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "System+Net+NetworkInformation+AixStructs+ifreq_mtu+_ifr_name_e__FixedBuffer"
)]
impl crate::System::Net::NetworkInformation::AixStructs::ifreq_mtu__ifr_name_e__FixedBuffer {}
