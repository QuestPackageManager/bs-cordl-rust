#[cfg(feature = "cordl_class_Mono+RuntimeStructs")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeStructs {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Mono+RuntimeStructs")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::RuntimeStructs {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono";
    const CLASS_NAME: &'static str = "RuntimeStructs";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Mono+RuntimeStructs")]
impl std::ops::Deref for crate::Mono::RuntimeStructs {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+RuntimeStructs")]
impl std::ops::DerefMut for crate::Mono::RuntimeStructs {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+RuntimeStructs")]
impl crate::Mono::RuntimeStructs {
    #[cfg(feature = "Mono+RuntimeStructs+GPtrArray")]
    pub type GPtrArray = crate::Mono::RuntimeStructs_GPtrArray;
    #[cfg(feature = "Mono+RuntimeStructs+GenericParamInfo")]
    pub type GenericParamInfo = crate::Mono::RuntimeStructs_GenericParamInfo;
    #[cfg(feature = "Mono+RuntimeStructs+MonoClass")]
    pub type MonoClass = crate::Mono::RuntimeStructs_MonoClass;
    #[cfg(feature = "Mono+RuntimeStructs+RemoteClass")]
    pub type RemoteClass = crate::Mono::RuntimeStructs_RemoteClass;
}
#[cfg(feature = "cordl_class_Mono+RuntimeStructs")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::RuntimeStructs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+GPtrArray")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct RuntimeStructs_GPtrArray {
    pub data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub len: i32,
}
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+GPtrArray")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::RuntimeStructs_GPtrArray {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono";
    const CLASS_NAME: &'static str = "RuntimeStructs/GPtrArray";
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
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+GPtrArray")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Mono::RuntimeStructs_GPtrArray {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+GPtrArray")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Mono::RuntimeStructs_GPtrArray {
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
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+GPtrArray")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Mono::RuntimeStructs_GPtrArray {
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
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+GPtrArray")]
unsafe impl quest_hook::libil2cpp::Return for crate::Mono::RuntimeStructs_GPtrArray {
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
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+GPtrArray")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Mono::RuntimeStructs_GPtrArray {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+RuntimeStructs+GPtrArray")]
impl crate::Mono::RuntimeStructs_GPtrArray {}
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+GenericParamInfo")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct RuntimeStructs_GenericParamInfo {
    pub pklass: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub name: crate::System::IntPtr,
    pub flags: u16,
    pub token: u32,
    pub constraints: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+GenericParamInfo")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::RuntimeStructs_GenericParamInfo {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono";
    const CLASS_NAME: &'static str = "RuntimeStructs/GenericParamInfo";
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
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+GenericParamInfo")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Mono::RuntimeStructs_GenericParamInfo {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+GenericParamInfo")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Mono::RuntimeStructs_GenericParamInfo {
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
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+GenericParamInfo")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Mono::RuntimeStructs_GenericParamInfo {
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
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+GenericParamInfo")]
unsafe impl quest_hook::libil2cpp::Return for crate::Mono::RuntimeStructs_GenericParamInfo {
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
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+GenericParamInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Mono::RuntimeStructs_GenericParamInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+RuntimeStructs+GenericParamInfo")]
impl crate::Mono::RuntimeStructs_GenericParamInfo {}
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+MonoClass")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct RuntimeStructs_MonoClass {}
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+MonoClass")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::RuntimeStructs_MonoClass {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono";
    const CLASS_NAME: &'static str = "RuntimeStructs/MonoClass";
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
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+MonoClass")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Mono::RuntimeStructs_MonoClass {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+MonoClass")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Mono::RuntimeStructs_MonoClass {
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
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+MonoClass")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Mono::RuntimeStructs_MonoClass {
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
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+MonoClass")]
unsafe impl quest_hook::libil2cpp::Return for crate::Mono::RuntimeStructs_MonoClass {
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
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+MonoClass")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Mono::RuntimeStructs_MonoClass {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+RuntimeStructs+MonoClass")]
impl crate::Mono::RuntimeStructs_MonoClass {}
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+RemoteClass")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct RuntimeStructs_RemoteClass {
    pub default_vtable: crate::System::IntPtr,
    pub xdomain_vtable: crate::System::IntPtr,
    pub proxy_class: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub proxy_class_name: crate::System::IntPtr,
    pub interface_count: u32,
}
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+RemoteClass")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::RuntimeStructs_RemoteClass {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono";
    const CLASS_NAME: &'static str = "RuntimeStructs/RemoteClass";
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
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+RemoteClass")]
unsafe impl quest_hook::libil2cpp::Argument for crate::Mono::RuntimeStructs_RemoteClass {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+RemoteClass")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::Mono::RuntimeStructs_RemoteClass {
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
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+RemoteClass")]
unsafe impl quest_hook::libil2cpp::Returned for crate::Mono::RuntimeStructs_RemoteClass {
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
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+RemoteClass")]
unsafe impl quest_hook::libil2cpp::Return for crate::Mono::RuntimeStructs_RemoteClass {
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
#[cfg(feature = "cordl_class_Mono+RuntimeStructs+RemoteClass")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Mono::RuntimeStructs_RemoteClass {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+RuntimeStructs+RemoteClass")]
impl crate::Mono::RuntimeStructs_RemoteClass {}
