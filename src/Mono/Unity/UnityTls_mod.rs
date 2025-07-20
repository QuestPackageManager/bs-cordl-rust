#[cfg(feature = "Mono+Unity+UnityTls")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityTls {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Unity+UnityTls")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::Unity::UnityTls {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Mono+Unity+UnityTls")]
impl std::ops::Deref for crate::Mono::Unity::UnityTls {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls")]
impl std::ops::DerefMut for crate::Mono::Unity::UnityTls {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls")]
impl crate::Mono::Unity::UnityTls {
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_ciphersuite")]
    pub type unitytls_ciphersuite = crate::Mono::Unity::UnityTls_unitytls_ciphersuite;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_error_code")]
    pub type unitytls_error_code = crate::Mono::Unity::UnityTls_unitytls_error_code;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_errorstate")]
    pub type unitytls_errorstate = crate::Mono::Unity::UnityTls_unitytls_errorstate;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct")]
    pub type unitytls_interface_struct = crate::Mono::Unity::UnityTls_unitytls_interface_struct;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_key")]
    pub type unitytls_key = crate::Mono::Unity::UnityTls_unitytls_key;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_key_ref")]
    pub type unitytls_key_ref = crate::Mono::Unity::UnityTls_unitytls_key_ref;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_protocol")]
    pub type unitytls_protocol = crate::Mono::Unity::UnityTls_unitytls_protocol;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx")]
    pub type unitytls_tlsctx = crate::Mono::Unity::UnityTls_unitytls_tlsctx;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_callbacks")]
    pub type unitytls_tlsctx_callbacks = crate::Mono::Unity::UnityTls_unitytls_tlsctx_callbacks;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_certificate_callback")]
    pub type unitytls_tlsctx_certificate_callback = crate::Mono::Unity::UnityTls_unitytls_tlsctx_certificate_callback;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_protocolrange")]
    pub type unitytls_tlsctx_protocolrange = crate::Mono::Unity::UnityTls_unitytls_tlsctx_protocolrange;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_read_callback")]
    pub type unitytls_tlsctx_read_callback = crate::Mono::Unity::UnityTls_unitytls_tlsctx_read_callback;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_trace_callback")]
    pub type unitytls_tlsctx_trace_callback = crate::Mono::Unity::UnityTls_unitytls_tlsctx_trace_callback;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_write_callback")]
    pub type unitytls_tlsctx_write_callback = crate::Mono::Unity::UnityTls_unitytls_tlsctx_write_callback;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_x509verify_callback")]
    pub type unitytls_tlsctx_x509verify_callback = crate::Mono::Unity::UnityTls_unitytls_tlsctx_x509verify_callback;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509_ref")]
    pub type unitytls_x509_ref = crate::Mono::Unity::UnityTls_unitytls_x509_ref;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list")]
    pub type unitytls_x509list = crate::Mono::Unity::UnityTls_unitytls_x509list;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list_ref")]
    pub type unitytls_x509list_ref = crate::Mono::Unity::UnityTls_unitytls_x509list_ref;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509name")]
    pub type unitytls_x509name = crate::Mono::Unity::UnityTls_unitytls_x509name;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509verify_callback")]
    pub type unitytls_x509verify_callback = crate::Mono::Unity::UnityTls_unitytls_x509verify_callback;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509verify_result")]
    pub type unitytls_x509verify_result = crate::Mono::Unity::UnityTls_unitytls_x509verify_result;
    pub fn GetUnityTlsInterface() -> quest_hook::libil2cpp::Result<
        crate::System::IntPtr,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (),
                        crate::System::IntPtr,
                        0usize,
                    >("GetUnityTlsInterface")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetUnityTlsInterface", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSupported() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(), bool, 0usize>("get_IsSupported")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_IsSupported", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_NativeInterface() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::Unity::UnityTls_unitytls_interface_struct>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Mono::Unity::UnityTls_unitytls_interface_struct,
                        >,
                        0usize,
                    >("get_NativeInterface")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_NativeInterface", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::Unity::UnityTls_unitytls_interface_struct,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Unity+UnityTls")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Unity::UnityTls {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_ciphersuite")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnityTls_unitytls_ciphersuite {
    #[default]
    UNITYTLS_CIPHERSUITE_INVALID = 16777215u32,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_ciphersuite")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::UnityTls_unitytls_ciphersuite {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_ciphersuite";
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_ciphersuite")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Unity::UnityTls_unitytls_ciphersuite {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_ciphersuite")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Unity::UnityTls_unitytls_ciphersuite {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_ciphersuite")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Unity::UnityTls_unitytls_ciphersuite {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_ciphersuite")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::Unity::UnityTls_unitytls_ciphersuite {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_error_code")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnityTls_unitytls_error_code {
    #[default]
    UNITYTLS_BUFFER_OVERFLOW = 5u32,
    UNITYTLS_ENTROPY_SOURCE_FAILED = 9u32,
    UNITYTLS_INTERNAL_ERROR = 7u32,
    UNITYTLS_INVALID_ARGUMENT = 1u32,
    UNITYTLS_INVALID_FORMAT = 2u32,
    UNITYTLS_INVALID_PASSWORD = 3u32,
    UNITYTLS_INVALID_STATE = 4u32,
    UNITYTLS_NOT_SUPPORTED = 8u32,
    UNITYTLS_OUT_OF_MEMORY = 6u32,
    UNITYTLS_STREAM_CLOSED = 10u32,
    UNITYTLS_SUCCESS = 0u32,
    UNITYTLS_USER_CUSTOM_ERROR_END = 2097152u32,
    UNITYTLS_USER_CUSTOM_ERROR_START = 1048576u32,
    UNITYTLS_USER_READ_FAILED = 1048578u32,
    UNITYTLS_USER_UNKNOWN_ERROR = 1048580u32,
    UNITYTLS_USER_WOULD_BLOCK = 1048577u32,
    UNITYTLS_USER_WRITE_FAILED = 1048579u32,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_error_code")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::UnityTls_unitytls_error_code {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_error_code";
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_error_code")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Unity::UnityTls_unitytls_error_code {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_error_code")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Unity::UnityTls_unitytls_error_code {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_error_code")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Unity::UnityTls_unitytls_error_code {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_error_code")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::Unity::UnityTls_unitytls_error_code {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_errorstate")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnityTls_unitytls_errorstate {
    pub magic: u32,
    pub code: crate::Mono::Unity::UnityTls_unitytls_error_code,
    pub reserved: u64,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_errorstate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::UnityTls_unitytls_errorstate {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_errorstate";
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_errorstate")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Unity::UnityTls_unitytls_errorstate {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_errorstate")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Unity::UnityTls_unitytls_errorstate {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_errorstate")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Unity::UnityTls_unitytls_errorstate {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_errorstate")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::Unity::UnityTls_unitytls_errorstate {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_errorstate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::Unity::UnityTls_unitytls_errorstate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_errorstate")]
impl crate::Mono::Unity::UnityTls_unitytls_errorstate {}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityTls_unitytls_interface_struct {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub UNITYTLS_INVALID_HANDLE: u64,
    pub UNITYTLS_TLSCTX_PROTOCOLRANGE_DEFAULT: crate::Mono::Unity::UnityTls_unitytls_tlsctx_protocolrange,
    pub unitytls_errorstate_create: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_errorstate_create_t,
    >,
    pub unitytls_errorstate_raise_error: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_errorstate_raise_error_t,
    >,
    pub unitytls_key_get_ref: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_get_ref_t,
    >,
    pub unitytls_key_parse_der: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_parse_der_t,
    >,
    pub unitytls_key_parse_pem: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_parse_pem_t,
    >,
    pub unitytls_key_free: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_free_t,
    >,
    pub unitytls_x509_export_der: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509_export_der_t,
    >,
    pub unitytls_x509list_get_ref: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_get_ref_t,
    >,
    pub unitytls_x509list_get_x509: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_get_x509_t,
    >,
    pub unitytls_x509list_create: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_create_t,
    >,
    pub unitytls_x509list_append: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_append_t,
    >,
    pub unitytls_x509list_append_der: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_append_der_t,
    >,
    pub unitytls_x509list_append_pem: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_append_der_t,
    >,
    pub unitytls_x509list_free: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_free_t,
    >,
    pub unitytls_x509verify_default_ca: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509verify_default_ca_t,
    >,
    pub unitytls_x509verify_explicit_ca: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509verify_explicit_ca_t,
    >,
    pub unitytls_tlsctx_create_server: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_create_server_t,
    >,
    pub unitytls_tlsctx_create_client: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_create_client_t,
    >,
    pub unitytls_tlsctx_server_require_client_authentication: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_server_require_client_authentication_t,
    >,
    pub unitytls_tlsctx_set_certificate_callback: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_certificate_callback_t,
    >,
    pub unitytls_tlsctx_set_trace_callback: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_trace_callback_t,
    >,
    pub unitytls_tlsctx_set_x509verify_callback: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_x509verify_callback_t,
    >,
    pub unitytls_tlsctx_set_supported_ciphersuites: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_supported_ciphersuites_t,
    >,
    pub unitytls_tlsctx_get_ciphersuite: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_get_ciphersuite_t,
    >,
    pub unitytls_tlsctx_get_protocol: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_get_protocol_t,
    >,
    pub unitytls_tlsctx_process_handshake: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_process_handshake_t,
    >,
    pub unitytls_tlsctx_read: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_read_t,
    >,
    pub unitytls_tlsctx_write: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_write_t,
    >,
    pub unitytls_tlsctx_notify_close: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_notify_close_t,
    >,
    pub unitytls_tlsctx_free: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_free_t,
    >,
    pub unitytls_random_generate_bytes: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_random_generate_bytes_t,
    >,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::UnityTls_unitytls_interface_struct {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct")]
impl std::ops::Deref for crate::Mono::Unity::UnityTls_unitytls_interface_struct {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct")]
impl std::ops::DerefMut for crate::Mono::Unity::UnityTls_unitytls_interface_struct {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct")]
impl crate::Mono::Unity::UnityTls_unitytls_interface_struct {
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_create_t"
    )]
    pub type unitytls_errorstate_create_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_errorstate_create_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_raise_error_t"
    )]
    pub type unitytls_errorstate_raise_error_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_errorstate_raise_error_t;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_free_t")]
    pub type unitytls_key_free_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_free_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_get_ref_t"
    )]
    pub type unitytls_key_get_ref_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_get_ref_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_der_t"
    )]
    pub type unitytls_key_parse_der_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_parse_der_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_pem_t"
    )]
    pub type unitytls_key_parse_pem_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_parse_pem_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_random_generate_bytes_t"
    )]
    pub type unitytls_random_generate_bytes_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_random_generate_bytes_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_client_t"
    )]
    pub type unitytls_tlsctx_create_client_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_create_client_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_server_t"
    )]
    pub type unitytls_tlsctx_create_server_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_create_server_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_free_t"
    )]
    pub type unitytls_tlsctx_free_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_free_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_ciphersuite_t"
    )]
    pub type unitytls_tlsctx_get_ciphersuite_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_get_ciphersuite_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_protocol_t"
    )]
    pub type unitytls_tlsctx_get_protocol_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_get_protocol_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_notify_close_t"
    )]
    pub type unitytls_tlsctx_notify_close_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_notify_close_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_process_handshake_t"
    )]
    pub type unitytls_tlsctx_process_handshake_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_process_handshake_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_read_t"
    )]
    pub type unitytls_tlsctx_read_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_read_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_server_require_client_authentication_t"
    )]
    pub type unitytls_tlsctx_server_require_client_authentication_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_server_require_client_authentication_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_certificate_callback_t"
    )]
    pub type unitytls_tlsctx_set_certificate_callback_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_certificate_callback_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_supported_ciphersuites_t"
    )]
    pub type unitytls_tlsctx_set_supported_ciphersuites_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_supported_ciphersuites_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_trace_callback_t"
    )]
    pub type unitytls_tlsctx_set_trace_callback_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_trace_callback_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_x509verify_callback_t"
    )]
    pub type unitytls_tlsctx_set_x509verify_callback_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_x509verify_callback_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_write_t"
    )]
    pub type unitytls_tlsctx_write_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_write_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509_export_der_t"
    )]
    pub type unitytls_x509_export_der_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509_export_der_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_der_t"
    )]
    pub type unitytls_x509list_append_der_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_append_der_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_t"
    )]
    pub type unitytls_x509list_append_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_append_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_create_t"
    )]
    pub type unitytls_x509list_create_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_create_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_free_t"
    )]
    pub type unitytls_x509list_free_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_free_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_ref_t"
    )]
    pub type unitytls_x509list_get_ref_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_get_ref_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_x509_t"
    )]
    pub type unitytls_x509list_get_x509_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_get_x509_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_default_ca_t"
    )]
    pub type unitytls_x509verify_default_ca_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509verify_default_ca_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_explicit_ca_t"
    )]
    pub type unitytls_x509verify_explicit_ca_t = crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509verify_explicit_ca_t;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::UnityTls_unitytls_interface_struct {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_key")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnityTls_unitytls_key {}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_key")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::Unity::UnityTls_unitytls_key {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_key";
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_key")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Unity::UnityTls_unitytls_key {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_key")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Unity::UnityTls_unitytls_key {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_key")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Unity::UnityTls_unitytls_key {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_key")]
unsafe impl quest_hook::libil2cpp::Return for crate::Mono::Unity::UnityTls_unitytls_key {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_key")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::Unity::UnityTls_unitytls_key {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_key")]
impl crate::Mono::Unity::UnityTls_unitytls_key {}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_key_ref")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnityTls_unitytls_key_ref {
    pub handle: u64,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_key_ref")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::UnityTls_unitytls_key_ref {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_key_ref";
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_key_ref")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Unity::UnityTls_unitytls_key_ref {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_key_ref")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Unity::UnityTls_unitytls_key_ref {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_key_ref")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Unity::UnityTls_unitytls_key_ref {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_key_ref")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::Unity::UnityTls_unitytls_key_ref {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_key_ref")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::Unity::UnityTls_unitytls_key_ref {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_key_ref")]
impl crate::Mono::Unity::UnityTls_unitytls_key_ref {}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_protocol")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnityTls_unitytls_protocol {
    #[default]
    UNITYTLS_PROTOCOL_INVALID = 3u32,
    UNITYTLS_PROTOCOL_TLS_1_0 = 0u32,
    UNITYTLS_PROTOCOL_TLS_1_1 = 1u32,
    UNITYTLS_PROTOCOL_TLS_1_2 = 2u32,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_protocol")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::UnityTls_unitytls_protocol {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_protocol";
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_protocol")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Unity::UnityTls_unitytls_protocol {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_protocol")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Unity::UnityTls_unitytls_protocol {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_protocol")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Unity::UnityTls_unitytls_protocol {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_protocol")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::Unity::UnityTls_unitytls_protocol {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnityTls_unitytls_tlsctx {}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::UnityTls_unitytls_tlsctx {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_tlsctx";
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Unity::UnityTls_unitytls_tlsctx {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Unity::UnityTls_unitytls_tlsctx {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Unity::UnityTls_unitytls_tlsctx {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::Unity::UnityTls_unitytls_tlsctx {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::Unity::UnityTls_unitytls_tlsctx {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx")]
impl crate::Mono::Unity::UnityTls_unitytls_tlsctx {}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_callbacks")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnityTls_unitytls_tlsctx_callbacks {
    pub read: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::UnityTls_unitytls_tlsctx_read_callback,
    >,
    pub write: quest_hook::libil2cpp::Gc<
        crate::Mono::Unity::UnityTls_unitytls_tlsctx_write_callback,
    >,
    pub data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_callbacks")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_callbacks {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_tlsctx_callbacks";
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_callbacks")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_callbacks {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_callbacks")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_callbacks {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_callbacks")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_callbacks {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_callbacks")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_callbacks {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_callbacks")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_callbacks {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_callbacks")]
impl crate::Mono::Unity::UnityTls_unitytls_tlsctx_callbacks {}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_certificate_callback")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityTls_unitytls_tlsctx_certificate_callback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_certificate_callback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_certificate_callback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_tlsctx_certificate_callback";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_certificate_callback")]
impl std::ops::Deref
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_certificate_callback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_certificate_callback")]
impl std::ops::DerefMut
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_certificate_callback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_certificate_callback")]
impl crate::Mono::Unity::UnityTls_unitytls_tlsctx_certificate_callback {
    pub fn Invoke(
        &mut self,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cn: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cnLen: crate::System::IntPtr,
        caList: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        caListLen: crate::System::IntPtr,
        chain: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        9usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (userData, ctx, cn, cnLen, caList, caListLen, chain, key, errorState),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_certificate_callback")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_certificate_callback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_protocolrange")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnityTls_unitytls_tlsctx_protocolrange {
    pub min: crate::Mono::Unity::UnityTls_unitytls_protocol,
    pub max: crate::Mono::Unity::UnityTls_unitytls_protocol,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_protocolrange")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_protocolrange {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_tlsctx_protocolrange";
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_protocolrange")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_protocolrange {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_protocolrange")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_protocolrange {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_protocolrange")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_protocolrange {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_protocolrange")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_protocolrange {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_protocolrange")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_protocolrange {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_protocolrange")]
impl crate::Mono::Unity::UnityTls_unitytls_tlsctx_protocolrange {}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_read_callback")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityTls_unitytls_tlsctx_read_callback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_read_callback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_read_callback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_tlsctx_read_callback";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_read_callback")]
impl std::ops::Deref for crate::Mono::Unity::UnityTls_unitytls_tlsctx_read_callback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_read_callback")]
impl std::ops::DerefMut for crate::Mono::Unity::UnityTls_unitytls_tlsctx_read_callback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_read_callback")]
impl crate::Mono::Unity::UnityTls_unitytls_tlsctx_read_callback {
    pub fn Invoke(
        &mut self,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferLen: crate::System::IntPtr,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        crate::System::IntPtr,
                        4usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked(self, (userData, buffer, bufferLen, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_read_callback")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_read_callback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_trace_callback")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityTls_unitytls_tlsctx_trace_callback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_trace_callback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_trace_callback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_tlsctx_trace_callback";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_trace_callback")]
impl std::ops::Deref for crate::Mono::Unity::UnityTls_unitytls_tlsctx_trace_callback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_trace_callback")]
impl std::ops::DerefMut for crate::Mono::Unity::UnityTls_unitytls_tlsctx_trace_callback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_trace_callback")]
impl crate::Mono::Unity::UnityTls_unitytls_tlsctx_trace_callback {
    pub fn Invoke(
        &mut self,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        traceMessage: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        traceMessageLen: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(self, (userData, ctx, traceMessage, traceMessageLen))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_trace_callback")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_trace_callback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_write_callback")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityTls_unitytls_tlsctx_write_callback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_write_callback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_write_callback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_tlsctx_write_callback";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_write_callback")]
impl std::ops::Deref for crate::Mono::Unity::UnityTls_unitytls_tlsctx_write_callback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_write_callback")]
impl std::ops::DerefMut for crate::Mono::Unity::UnityTls_unitytls_tlsctx_write_callback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_write_callback")]
impl crate::Mono::Unity::UnityTls_unitytls_tlsctx_write_callback {
    pub fn Invoke(
        &mut self,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferLen: crate::System::IntPtr,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        crate::System::IntPtr,
                        4usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked(self, (userData, data, bufferLen, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_write_callback")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_write_callback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_x509verify_callback")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityTls_unitytls_tlsctx_x509verify_callback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_x509verify_callback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_x509verify_callback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_tlsctx_x509verify_callback";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_x509verify_callback")]
impl std::ops::Deref
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_x509verify_callback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_x509verify_callback")]
impl std::ops::DerefMut
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_x509verify_callback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_x509verify_callback")]
impl crate::Mono::Unity::UnityTls_unitytls_tlsctx_x509verify_callback {
    pub fn Invoke(
        &mut self,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        chain: crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
                        3usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_x509verify_result = unsafe {
            method.invoke_unchecked(self, (userData, chain, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_x509verify_callback")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::UnityTls_unitytls_tlsctx_x509verify_callback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509_ref")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnityTls_unitytls_x509_ref {
    pub handle: u64,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509_ref")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::UnityTls_unitytls_x509_ref {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_x509_ref";
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509_ref")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Unity::UnityTls_unitytls_x509_ref {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509_ref")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Unity::UnityTls_unitytls_x509_ref {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509_ref")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Unity::UnityTls_unitytls_x509_ref {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509_ref")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::Unity::UnityTls_unitytls_x509_ref {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509_ref")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::Unity::UnityTls_unitytls_x509_ref {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509_ref")]
impl crate::Mono::Unity::UnityTls_unitytls_x509_ref {}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnityTls_unitytls_x509list {}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::UnityTls_unitytls_x509list {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_x509list";
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Unity::UnityTls_unitytls_x509list {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Unity::UnityTls_unitytls_x509list {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Unity::UnityTls_unitytls_x509list {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::Unity::UnityTls_unitytls_x509list {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::Unity::UnityTls_unitytls_x509list {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list")]
impl crate::Mono::Unity::UnityTls_unitytls_x509list {}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list_ref")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnityTls_unitytls_x509list_ref {
    pub handle: u64,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list_ref")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::UnityTls_unitytls_x509list_ref {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_x509list_ref";
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list_ref")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Unity::UnityTls_unitytls_x509list_ref {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list_ref")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Unity::UnityTls_unitytls_x509list_ref {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list_ref")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Unity::UnityTls_unitytls_x509list_ref {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list_ref")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::Unity::UnityTls_unitytls_x509list_ref {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list_ref")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::Unity::UnityTls_unitytls_x509list_ref {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list_ref")]
impl crate::Mono::Unity::UnityTls_unitytls_x509list_ref {}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509name")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnityTls_unitytls_x509name {}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509name")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::UnityTls_unitytls_x509name {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_x509name";
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509name")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Unity::UnityTls_unitytls_x509name {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509name")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Unity::UnityTls_unitytls_x509name {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509name")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Unity::UnityTls_unitytls_x509name {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509name")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::Unity::UnityTls_unitytls_x509name {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509name")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Mono::Unity::UnityTls_unitytls_x509name {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509name")]
impl crate::Mono::Unity::UnityTls_unitytls_x509name {}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509verify_callback")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityTls_unitytls_x509verify_callback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509verify_callback")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::UnityTls_unitytls_x509verify_callback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_x509verify_callback";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509verify_callback")]
impl std::ops::Deref for crate::Mono::Unity::UnityTls_unitytls_x509verify_callback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509verify_callback")]
impl std::ops::DerefMut for crate::Mono::Unity::UnityTls_unitytls_x509verify_callback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509verify_callback")]
impl crate::Mono::Unity::UnityTls_unitytls_x509verify_callback {
    pub fn Invoke(
        &mut self,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cert: crate::Mono::Unity::UnityTls_unitytls_x509_ref,
        result: crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::Mono::Unity::UnityTls_unitytls_x509_ref,
                            crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
                        4usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_x509verify_result = unsafe {
            method.invoke_unchecked(self, (userData, cert, result, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509verify_callback")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::UnityTls_unitytls_x509verify_callback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509verify_result")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnityTls_unitytls_x509verify_result {
    #[default]
    UNITYTLS_X509VERIFY_FATAL_ERROR = 4294967295u32,
    UNITYTLS_X509VERIFY_FLAG_CN_MISMATCH = 4u32,
    UNITYTLS_X509VERIFY_FLAG_EXPIRED = 1u32,
    UNITYTLS_X509VERIFY_FLAG_NOT_TRUSTED = 8u32,
    UNITYTLS_X509VERIFY_FLAG_REVOKED = 2u32,
    UNITYTLS_X509VERIFY_FLAG_UNKNOWN_ERROR = 134217728u32,
    UNITYTLS_X509VERIFY_FLAG_USER_ERROR1 = 65536u32,
    UNITYTLS_X509VERIFY_FLAG_USER_ERROR2 = 131072u32,
    UNITYTLS_X509VERIFY_FLAG_USER_ERROR3 = 262144u32,
    UNITYTLS_X509VERIFY_FLAG_USER_ERROR4 = 524288u32,
    UNITYTLS_X509VERIFY_FLAG_USER_ERROR5 = 1048576u32,
    UNITYTLS_X509VERIFY_FLAG_USER_ERROR6 = 2097152u32,
    UNITYTLS_X509VERIFY_FLAG_USER_ERROR7 = 4194304u32,
    UNITYTLS_X509VERIFY_FLAG_USER_ERROR8 = 8388608u32,
    UNITYTLS_X509VERIFY_NOT_DONE = 2147483648u32,
    UNITYTLS_X509VERIFY_SUCCESS = 0u32,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509verify_result")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::UnityTls_unitytls_x509verify_result {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_x509verify_result";
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509verify_result")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Mono::Unity::UnityTls_unitytls_x509verify_result {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509verify_result")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Mono::Unity::UnityTls_unitytls_x509verify_result {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509verify_result")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Mono::Unity::UnityTls_unitytls_x509verify_result {
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509verify_result")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Mono::Unity::UnityTls_unitytls_x509verify_result {
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_create_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_errorstate_create_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_create_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_errorstate_create_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_errorstate_create_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_create_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_errorstate_create_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_create_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_errorstate_create_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_create_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_errorstate_create_t {
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Unity::UnityTls_unitytls_errorstate,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (),
                        crate::Mono::Unity::UnityTls_unitytls_errorstate,
                        0usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_errorstate = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_create_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_errorstate_create_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_raise_error_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_errorstate_raise_error_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_raise_error_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_errorstate_raise_error_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_errorstate_raise_error_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_raise_error_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_errorstate_raise_error_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_raise_error_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_errorstate_raise_error_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_raise_error_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_errorstate_raise_error_t {
    pub fn Invoke(
        &mut self,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        errorCode: crate::Mono::Unity::UnityTls_unitytls_error_code,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::Mono::Unity::UnityTls_unitytls_error_code,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (errorState, errorCode))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_raise_error_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_errorstate_raise_error_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_free_t")]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_key_free_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_free_t")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_free_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_key_free_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_free_t")]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_free_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_free_t")]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_free_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_free_t")]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_free_t {
    pub fn Invoke(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (key))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_free_t")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_free_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_get_ref_t")]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_key_get_ref_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_get_ref_t")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_get_ref_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_key_get_ref_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_get_ref_t")]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_get_ref_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_get_ref_t")]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_get_ref_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_get_ref_t")]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_get_ref_t {
    pub fn Invoke(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Mono::Unity::UnityTls_unitytls_key_ref> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        crate::Mono::Unity::UnityTls_unitytls_key_ref,
                        2usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_key_ref = unsafe {
            method.invoke_unchecked(self, (key, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_get_ref_t")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_get_ref_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_der_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_key_parse_der_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_der_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_parse_der_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_key_parse_der_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_der_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_parse_der_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_der_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_parse_der_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_der_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_parse_der_t {
    pub fn Invoke(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferLen: crate::System::IntPtr,
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        passwordLen: crate::System::IntPtr,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        5usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (buffer, bufferLen, password, passwordLen, errorState),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_der_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_parse_der_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_pem_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_key_parse_pem_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_pem_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_parse_pem_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_key_parse_pem_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_pem_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_parse_pem_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_pem_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_parse_pem_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_pem_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_parse_pem_t {
    pub fn Invoke(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferLen: crate::System::IntPtr,
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        passwordLen: crate::System::IntPtr,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        5usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (buffer, bufferLen, password, passwordLen, errorState),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_pem_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_key_parse_pem_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_random_generate_bytes_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_random_generate_bytes_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_random_generate_bytes_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_random_generate_bytes_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_random_generate_bytes_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_random_generate_bytes_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_random_generate_bytes_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_random_generate_bytes_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_random_generate_bytes_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_random_generate_bytes_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_random_generate_bytes_t {
    pub fn Invoke(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferLen: crate::System::IntPtr,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (buffer, bufferLen, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_random_generate_bytes_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_random_generate_bytes_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_client_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_tlsctx_create_client_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_client_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_create_client_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_tlsctx_create_client_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_client_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_create_client_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_client_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_create_client_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_client_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_create_client_t {
    pub fn Invoke(
        &mut self,
        supportedProtocols: crate::Mono::Unity::UnityTls_unitytls_tlsctx_protocolrange,
        callbacks: crate::Mono::Unity::UnityTls_unitytls_tlsctx_callbacks,
        cn: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cnLen: crate::System::IntPtr,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::Mono::Unity::UnityTls_unitytls_tlsctx_protocolrange,
                            crate::Mono::Unity::UnityTls_unitytls_tlsctx_callbacks,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        5usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (supportedProtocols, callbacks, cn, cnLen, errorState),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_client_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_create_client_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_server_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_tlsctx_create_server_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_server_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_create_server_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_tlsctx_create_server_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_server_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_create_server_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_server_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_create_server_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_server_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_create_server_t {
    pub fn Invoke(
        &mut self,
        supportedProtocols: crate::Mono::Unity::UnityTls_unitytls_tlsctx_protocolrange,
        callbacks: crate::Mono::Unity::UnityTls_unitytls_tlsctx_callbacks,
        certChain: u64,
        leafCertificateKey: u64,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::Mono::Unity::UnityTls_unitytls_tlsctx_protocolrange,
                            crate::Mono::Unity::UnityTls_unitytls_tlsctx_callbacks,
                            u64,
                            u64,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        5usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        supportedProtocols,
                        callbacks,
                        certChain,
                        leafCertificateKey,
                        errorState,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_server_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_create_server_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_free_t")]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_tlsctx_free_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_free_t")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_free_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_tlsctx_free_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_free_t")]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_free_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_free_t")]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_free_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_free_t")]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_free_t {
    pub fn Invoke(
        &mut self,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ctx))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_free_t")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_free_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_ciphersuite_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_tlsctx_get_ciphersuite_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_ciphersuite_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_get_ciphersuite_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_tlsctx_get_ciphersuite_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_ciphersuite_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_get_ciphersuite_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_ciphersuite_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_get_ciphersuite_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_ciphersuite_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_get_ciphersuite_t {
    pub fn Invoke(
        &mut self,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Unity::UnityTls_unitytls_ciphersuite,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        crate::Mono::Unity::UnityTls_unitytls_ciphersuite,
                        2usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_ciphersuite = unsafe {
            method.invoke_unchecked(self, (ctx, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_ciphersuite_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_get_ciphersuite_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_protocol_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_tlsctx_get_protocol_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_protocol_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_get_protocol_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_tlsctx_get_protocol_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_protocol_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_get_protocol_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_protocol_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_get_protocol_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_protocol_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_get_protocol_t {
    pub fn Invoke(
        &mut self,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Mono::Unity::UnityTls_unitytls_protocol> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        crate::Mono::Unity::UnityTls_unitytls_protocol,
                        2usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_protocol = unsafe {
            method.invoke_unchecked(self, (ctx, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_protocol_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_get_protocol_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_notify_close_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_tlsctx_notify_close_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_notify_close_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_notify_close_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_tlsctx_notify_close_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_notify_close_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_notify_close_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_notify_close_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_notify_close_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_notify_close_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_notify_close_t {
    pub fn Invoke(
        &mut self,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ctx, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_notify_close_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_notify_close_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_process_handshake_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_tlsctx_process_handshake_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_process_handshake_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_process_handshake_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_tlsctx_process_handshake_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_process_handshake_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_process_handshake_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_process_handshake_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_process_handshake_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_process_handshake_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_process_handshake_t {
    pub fn Invoke(
        &mut self,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
                        2usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_x509verify_result = unsafe {
            method.invoke_unchecked(self, (ctx, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_process_handshake_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_process_handshake_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_read_t")]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_tlsctx_read_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_read_t")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_read_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_tlsctx_read_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_read_t")]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_read_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_read_t")]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_read_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_read_t")]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_read_t {
    pub fn Invoke(
        &mut self,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferLen: crate::System::IntPtr,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        crate::System::IntPtr,
                        4usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked(self, (ctx, buffer, bufferLen, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_read_t")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_read_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_server_require_client_authentication_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_tlsctx_server_require_client_authentication_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_server_require_client_authentication_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_server_require_client_authentication_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_tlsctx_server_require_client_authentication_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_server_require_client_authentication_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_server_require_client_authentication_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_server_require_client_authentication_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_server_require_client_authentication_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_server_require_client_authentication_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_server_require_client_authentication_t {
    pub fn Invoke(
        &mut self,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        clientAuthCAList: crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ctx, clientAuthCAList, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_server_require_client_authentication_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_server_require_client_authentication_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_certificate_callback_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_certificate_callback_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_certificate_callback_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_certificate_callback_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_tlsctx_set_certificate_callback_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_certificate_callback_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_certificate_callback_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_certificate_callback_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_certificate_callback_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_certificate_callback_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_certificate_callback_t {
    pub fn Invoke(
        &mut self,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cb: quest_hook::libil2cpp::Gc<
            crate::Mono::Unity::UnityTls_unitytls_tlsctx_certificate_callback,
        >,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Mono::Unity::UnityTls_unitytls_tlsctx_certificate_callback,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ctx, cb, userData, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_certificate_callback_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_certificate_callback_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_supported_ciphersuites_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_supported_ciphersuites_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_supported_ciphersuites_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_supported_ciphersuites_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_tlsctx_set_supported_ciphersuites_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_supported_ciphersuites_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_supported_ciphersuites_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_supported_ciphersuites_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_supported_ciphersuites_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_supported_ciphersuites_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_supported_ciphersuites_t {
    pub fn Invoke(
        &mut self,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        supportedCiphersuites: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        supportedCiphersuitesLen: crate::System::IntPtr,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (ctx, supportedCiphersuites, supportedCiphersuitesLen, errorState),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_supported_ciphersuites_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_supported_ciphersuites_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_trace_callback_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_trace_callback_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_trace_callback_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_trace_callback_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_tlsctx_set_trace_callback_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_trace_callback_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_trace_callback_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_trace_callback_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_trace_callback_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_trace_callback_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_trace_callback_t {
    pub fn Invoke(
        &mut self,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cb: quest_hook::libil2cpp::Gc<
            crate::Mono::Unity::UnityTls_unitytls_tlsctx_trace_callback,
        >,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Mono::Unity::UnityTls_unitytls_tlsctx_trace_callback,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ctx, cb, userData, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_trace_callback_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_trace_callback_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_x509verify_callback_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_x509verify_callback_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_x509verify_callback_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_x509verify_callback_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_tlsctx_set_x509verify_callback_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_x509verify_callback_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_x509verify_callback_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_x509verify_callback_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_x509verify_callback_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_x509verify_callback_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_x509verify_callback_t {
    pub fn Invoke(
        &mut self,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cb: quest_hook::libil2cpp::Gc<
            crate::Mono::Unity::UnityTls_unitytls_tlsctx_x509verify_callback,
        >,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Mono::Unity::UnityTls_unitytls_tlsctx_x509verify_callback,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (ctx, cb, userData, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_x509verify_callback_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_set_x509verify_callback_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_write_t")]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_tlsctx_write_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_write_t")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_write_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_tlsctx_write_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_write_t")]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_write_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_write_t")]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_write_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_write_t")]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_write_t {
    pub fn Invoke(
        &mut self,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferLen: crate::System::IntPtr,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        crate::System::IntPtr,
                        4usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked(self, (ctx, data, bufferLen, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_write_t")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_tlsctx_write_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509_export_der_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_x509_export_der_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509_export_der_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509_export_der_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_x509_export_der_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509_export_der_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509_export_der_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509_export_der_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509_export_der_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509_export_der_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509_export_der_t {
    pub fn Invoke(
        &mut self,
        cert: crate::Mono::Unity::UnityTls_unitytls_x509_ref,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferLen: crate::System::IntPtr,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::Mono::Unity::UnityTls_unitytls_x509_ref,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        crate::System::IntPtr,
                        4usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::System::IntPtr = unsafe {
            method.invoke_unchecked(self, (cert, buffer, bufferLen, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509_export_der_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509_export_der_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_der_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_x509list_append_der_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_der_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_append_der_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_x509list_append_der_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_der_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_append_der_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_der_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_append_der_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_der_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_append_der_t {
    pub fn Invoke(
        &mut self,
        list: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferLen: crate::System::IntPtr,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (list, buffer, bufferLen, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_der_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_append_der_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_x509list_append_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_append_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_x509list_append_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_append_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_append_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_append_t {
    pub fn Invoke(
        &mut self,
        list: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cert: crate::Mono::Unity::UnityTls_unitytls_x509_ref,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::Mono::Unity::UnityTls_unitytls_x509_ref,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (list, cert, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_append_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_create_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_x509list_create_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_create_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_create_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_x509list_create_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_create_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_create_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_create_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_create_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_create_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_create_t {
    pub fn Invoke(
        &mut self,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        1usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, (errorState))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_create_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_create_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_free_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_x509list_free_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_free_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_free_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_x509list_free_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_free_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_free_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_free_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_free_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_free_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_free_t {
    pub fn Invoke(
        &mut self,
        list: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (list))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_free_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_free_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_ref_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_x509list_get_ref_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_ref_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_get_ref_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_x509list_get_ref_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_ref_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_get_ref_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_ref_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_get_ref_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_ref_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_get_ref_t {
    pub fn Invoke(
        &mut self,
        list: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
                        2usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 2usize
                        )
                    })
            });
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_x509list_ref = unsafe {
            method.invoke_unchecked(self, (list, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_ref_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_get_ref_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_x509_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_x509list_get_x509_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_x509_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_get_x509_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_x509list_get_x509_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_x509_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_get_x509_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_x509_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_get_x509_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_x509_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_get_x509_t {
    pub fn Invoke(
        &mut self,
        list: crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
        index: crate::System::IntPtr,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Mono::Unity::UnityTls_unitytls_x509_ref> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        crate::Mono::Unity::UnityTls_unitytls_x509_ref,
                        3usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 3usize
                        )
                    })
            });
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_x509_ref = unsafe {
            method.invoke_unchecked(self, (list, index, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_x509_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509list_get_x509_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_default_ca_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_x509verify_default_ca_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_default_ca_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509verify_default_ca_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_x509verify_default_ca_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_default_ca_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509verify_default_ca_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_default_ca_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509verify_default_ca_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_default_ca_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509verify_default_ca_t {
    pub fn Invoke(
        &mut self,
        chain: crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
        cn: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cnLen: crate::System::IntPtr,
        cb: quest_hook::libil2cpp::Gc<
            crate::Mono::Unity::UnityTls_unitytls_x509verify_callback,
        >,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<
                                crate::Mono::Unity::UnityTls_unitytls_x509verify_callback,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
                        6usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 6usize
                        )
                    })
            });
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_x509verify_result = unsafe {
            method.invoke_unchecked(self, (chain, cn, cnLen, cb, userData, errorState))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_default_ca_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509verify_default_ca_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_explicit_ca_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_UnityTls_unitytls_x509verify_explicit_ca_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_explicit_ca_t"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509verify_explicit_ca_t {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTls/unitytls_interface_struct/unitytls_x509verify_explicit_ca_t";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_explicit_ca_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509verify_explicit_ca_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_explicit_ca_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509verify_explicit_ca_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_explicit_ca_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509verify_explicit_ca_t {
    pub fn Invoke(
        &mut self,
        chain: crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
        trustCA: crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
        cn: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cnLen: crate::System::IntPtr,
        cb: quest_hook::libil2cpp::Gc<
            crate::Mono::Unity::UnityTls_unitytls_x509verify_callback,
        >,
        userData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
                            crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                            quest_hook::libil2cpp::Gc<
                                crate::Mono::Unity::UnityTls_unitytls_x509verify_callback,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
                        7usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 7usize
                        )
                    })
            });
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_x509verify_result = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (chain, trustCA, cn, cnLen, cb, userData, errorState),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_explicit_ca_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_UnityTls_unitytls_x509verify_explicit_ca_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
