#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_112")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_112 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_112")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_112 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=112";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_112")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_112 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_112")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_112 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_112")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_112 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_112")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_112 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_112")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_112 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_112")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_112 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_12")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_12 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_12")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_12 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=12";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_12")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_12 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_12")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_12 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_12")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_12 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_12")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_12 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_12")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_12 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_12")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_12 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_1212")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_1212 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_1212")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_1212 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=1212";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_1212")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_1212 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_1212")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_1212 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_1212")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_1212 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_1212")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_1212 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_1212")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_1212 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_1212")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_1212 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_144")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_144 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_144")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_144 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=144";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_144")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_144 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_144")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_144 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_144")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_144 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_144")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_144 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_144")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_144 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_144")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_144 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_16")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_16 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_16")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_16 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=16";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_16")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_16 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_16")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_16 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_16")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_16 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_16")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_16 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_16")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_16 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_16")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_16 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_20")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_20 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_20")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_20 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=20";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_20")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_20 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_20")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_20 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_20")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_20 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_20")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_20 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_20")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_20 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_20")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_20 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_24")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_24 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_24")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_24 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=24";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_24")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_24 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_24")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_24 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_24")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_24 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_24")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_24 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_24")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_24 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_24")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_24 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_28")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_28 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_28")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_28 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=28";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_28")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_28 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_28")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_28 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_28")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_28 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_28")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_28 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_28")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_28 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_28")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_28 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_32")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_32 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_32")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_32 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=32";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_32")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_32 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_32")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_32 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_32")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_32 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_32")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_32 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_32")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_32 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_32")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_32 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_36")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_36 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_36")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_36 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=36";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_36")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_36 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_36")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_36 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_36")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_36 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_36")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_36 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_36")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_36 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_36")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_36 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_38")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_38 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_38")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_38 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=38";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_38")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_38 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_38")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_38 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_38")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_38 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_38")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_38 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_38")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_38 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_38")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_38 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_40")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_40 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_40")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_40 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=40";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_40")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_40 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_40")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_40 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_40")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_40 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_40")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_40 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_40")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_40 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_40")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_40 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_416")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_416 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_416")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_416 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=416";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_416")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_416 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_416")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_416 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_416")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_416 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_416")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_416 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_416")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_416 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_416")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_416 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_44")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_44 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_44")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_44 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=44";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_44")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_44 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_44")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_44 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_44")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_44 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_44")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_44 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_44")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_44 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_44")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_44 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_56")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_56 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_56")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_56 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=56";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_56")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_56 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_56")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_56 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_56")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_56 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_56")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_56 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_56")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_56 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_56")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_56 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_6")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_6 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_6")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_6 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=6";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_6")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_6 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_6")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_6 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_6")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_6 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_6")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_6 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_6")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_6 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_6")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_6 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_64")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_64 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_64")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_64 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=64";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_64")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_64 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_64")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_64 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_64")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_64 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_64")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_64 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_64")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_64 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_64")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_64 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_664")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_664 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_664")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_664 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=664";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_664")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_664 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_664")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_664 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_664")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_664 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_664")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_664 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_664")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_664 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_664")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_664 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_68")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_68 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_68")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_68 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=68";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_68")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_68 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_68")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_68 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_68")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_68 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_68")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_68 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_68")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_68 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_68")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_68 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_960")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_960 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_960")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_960 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<PrivateImplementationDetails>/__StaticArrayInitTypeSize=960";
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_960")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_960 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_960")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_960 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_960")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_960 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_960")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_960 {
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
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_960")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_960 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_960")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_960 {}
