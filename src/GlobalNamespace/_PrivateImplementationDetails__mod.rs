#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_10854")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_10854 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_10854")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_10854 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=10854";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_10854")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_10854 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_10854")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_10854 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_10854")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_10854 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_10854")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_10854 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_10854")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_10854 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_10854")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_10854 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_7319")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_7319 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_7319")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_7319 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=7319";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_7319")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_7319 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_7319")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_7319 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_7319")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_7319 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_7319")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_7319 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_7319")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_7319 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_7319")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_7319 {}
