#[cfg(feature = "Mono+Unity+UnityTls")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityTls {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Mono+Unity+UnityTls")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::UnityTls => "Mono.Unity"."UnityTls"
);
#[cfg(feature = "Mono+Unity+UnityTls")]
impl std::ops::Deref for crate::Mono::Unity::UnityTls {
    type Target = crate::System::Object;
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
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_write_callback")]
    pub type unitytls_tlsctx_write_callback = crate::Mono::Unity::UnityTls_unitytls_tlsctx_write_callback;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx")]
    pub type unitytls_tlsctx = crate::Mono::Unity::UnityTls_unitytls_tlsctx;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_read_callback")]
    pub type unitytls_tlsctx_read_callback = crate::Mono::Unity::UnityTls_unitytls_tlsctx_read_callback;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_ciphersuite")]
    pub type unitytls_ciphersuite = crate::Mono::Unity::UnityTls_unitytls_ciphersuite;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509verify_result")]
    pub type unitytls_x509verify_result = crate::Mono::Unity::UnityTls_unitytls_x509verify_result;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list_ref")]
    pub type unitytls_x509list_ref = crate::Mono::Unity::UnityTls_unitytls_x509list_ref;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_callbacks")]
    pub type unitytls_tlsctx_callbacks = crate::Mono::Unity::UnityTls_unitytls_tlsctx_callbacks;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_key_ref")]
    pub type unitytls_key_ref = crate::Mono::Unity::UnityTls_unitytls_key_ref;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct")]
    pub type unitytls_interface_struct = crate::Mono::Unity::UnityTls_unitytls_interface_struct;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_error_code")]
    pub type unitytls_error_code = crate::Mono::Unity::UnityTls_unitytls_error_code;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509_ref")]
    pub type unitytls_x509_ref = crate::Mono::Unity::UnityTls_unitytls_x509_ref;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_trace_callback")]
    pub type unitytls_tlsctx_trace_callback = crate::Mono::Unity::UnityTls_unitytls_tlsctx_trace_callback;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_errorstate")]
    pub type unitytls_errorstate = crate::Mono::Unity::UnityTls_unitytls_errorstate;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_certificate_callback")]
    pub type unitytls_tlsctx_certificate_callback = crate::Mono::Unity::UnityTls_unitytls_tlsctx_certificate_callback;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_protocol")]
    pub type unitytls_protocol = crate::Mono::Unity::UnityTls_unitytls_protocol;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list")]
    pub type unitytls_x509list = crate::Mono::Unity::UnityTls_unitytls_x509list;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509verify_callback")]
    pub type unitytls_x509verify_callback = crate::Mono::Unity::UnityTls_unitytls_x509verify_callback;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_protocolrange")]
    pub type unitytls_tlsctx_protocolrange = crate::Mono::Unity::UnityTls_unitytls_tlsctx_protocolrange;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_x509verify_callback")]
    pub type unitytls_tlsctx_x509verify_callback = crate::Mono::Unity::UnityTls_unitytls_tlsctx_x509verify_callback;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_key")]
    pub type unitytls_key = crate::Mono::Unity::UnityTls_unitytls_key;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509name")]
    pub type unitytls_x509name = crate::Mono::Unity::UnityTls_unitytls_x509name;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnityTls_unitytls_ciphersuite {
    UNITYTLS_CIPHERSUITE_INVALID = 4294967232u32,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_ciphersuite")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::UnityTls_unitytls_ciphersuite =>
    "Mono.Unity"."UnityTls/unitytls_ciphersuite"
);
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_error_code")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnityTls_unitytls_error_code {
    UNITYTLS_BUFFER_OVERFLOW = 134678021u32,
    UNITYTLS_ENTROPY_SOURCE_FAILED = 281020937u32,
    UNITYTLS_INTERNAL_ERROR = 168364039u32,
    UNITYTLS_INVALID_ARGUMENT = 67305985u32,
    UNITYTLS_INVALID_FORMAT = 84148994u32,
    UNITYTLS_INVALID_PASSWORD = 100992003u32,
    UNITYTLS_INVALID_STATE = 117835012u32,
    UNITYTLS_NOT_SUPPORTED = 3221883144u32,
    UNITYTLS_OUT_OF_MEMORY = 151521030u32,
    UNITYTLS_STREAM_CLOSED = 1097738u32,
    UNITYTLS_SUCCESS = 50462976u32,
    UNITYTLS_USER_CUSTOM_ERROR_END = 8384u32,
    UNITYTLS_USER_CUSTOM_ERROR_START = 4288u32,
    UNITYTLS_USER_READ_FAILED = 33558720u32,
    UNITYTLS_USER_UNKNOWN_ERROR = 67113152u32,
    UNITYTLS_USER_WOULD_BLOCK = 16781504u32,
    UNITYTLS_USER_WRITE_FAILED = 50335936u32,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_error_code")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::UnityTls_unitytls_error_code =>
    "Mono.Unity"."UnityTls/unitytls_error_code"
);
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_errorstate")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct UnityTls_unitytls_errorstate {
    pub magic: u32,
    pub code: crate::Mono::Unity::UnityTls_unitytls_error_code,
    pub reserved: u64,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_errorstate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::UnityTls_unitytls_errorstate =>
    "Mono.Unity"."UnityTls/unitytls_errorstate"
);
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_create_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_unitytls_errorstate_create_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_create_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_errorstate_create_t => "Mono.Unity"
    ."UnityTls/unitytls_interface_struct/unitytls_errorstate_create_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_create_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_errorstate_create_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_create_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_errorstate_create_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_create_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_errorstate_create_t {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Unity::UnityTls_unitytls_errorstate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_errorstate = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_create_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_errorstate_create_t {
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
pub struct unitytls_interface_struct_unitytls_errorstate_raise_error_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_raise_error_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_errorstate_raise_error_t =>
    "Mono.Unity"."UnityTls/unitytls_interface_struct/unitytls_errorstate_raise_error_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_raise_error_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_errorstate_raise_error_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_raise_error_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_errorstate_raise_error_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_raise_error_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_errorstate_raise_error_t {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
        errorCode: crate::Mono::Unity::UnityTls_unitytls_error_code,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (errorState, errorCode))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_raise_error_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_errorstate_raise_error_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityTls_unitytls_interface_struct {
    __cordl_parent: crate::System::Object,
    pub UNITYTLS_INVALID_HANDLE: u64,
    pub UNITYTLS_TLSCTX_PROTOCOLRANGE_DEFAULT: crate::Mono::Unity::UnityTls_unitytls_tlsctx_protocolrange,
    pub unitytls_errorstate_create: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_errorstate_create_t,
    pub unitytls_errorstate_raise_error: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_errorstate_raise_error_t,
    pub unitytls_key_get_ref: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_key_get_ref_t,
    pub unitytls_key_parse_der: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_key_parse_der_t,
    pub unitytls_key_parse_pem: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_key_parse_pem_t,
    pub unitytls_key_free: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_key_free_t,
    pub unitytls_x509_export_der: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_x509_export_der_t,
    pub unitytls_x509list_get_ref: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_get_ref_t,
    pub unitytls_x509list_get_x509: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_get_x509_t,
    pub unitytls_x509list_create: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_create_t,
    pub unitytls_x509list_append: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_append_t,
    pub unitytls_x509list_append_der: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_append_der_t,
    pub unitytls_x509list_append_pem: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_append_der_t,
    pub unitytls_x509list_free: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_free_t,
    pub unitytls_x509verify_default_ca: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_x509verify_default_ca_t,
    pub unitytls_x509verify_explicit_ca: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_x509verify_explicit_ca_t,
    pub unitytls_tlsctx_create_server: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_create_server_t,
    pub unitytls_tlsctx_create_client: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_create_client_t,
    pub unitytls_tlsctx_server_require_client_authentication: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_server_require_client_authentication_t,
    pub unitytls_tlsctx_set_certificate_callback: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_certificate_callback_t,
    pub unitytls_tlsctx_set_trace_callback: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_trace_callback_t,
    pub unitytls_tlsctx_set_x509verify_callback: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_x509verify_callback_t,
    pub unitytls_tlsctx_set_supported_ciphersuites: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_supported_ciphersuites_t,
    pub unitytls_tlsctx_get_ciphersuite: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_get_ciphersuite_t,
    pub unitytls_tlsctx_get_protocol: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_get_protocol_t,
    pub unitytls_tlsctx_process_handshake: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_process_handshake_t,
    pub unitytls_tlsctx_read: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_read_t,
    pub unitytls_tlsctx_write: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_write_t,
    pub unitytls_tlsctx_notify_close: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_notify_close_t,
    pub unitytls_tlsctx_free: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_free_t,
    pub unitytls_random_generate_bytes: *mut crate::Mono::Unity::unitytls_interface_struct_unitytls_random_generate_bytes_t,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::UnityTls_unitytls_interface_struct
    => "Mono.Unity"."UnityTls/unitytls_interface_struct"
);
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct")]
impl std::ops::Deref for crate::Mono::Unity::UnityTls_unitytls_interface_struct {
    type Target = crate::System::Object;
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
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_read_t"
    )]
    pub type unitytls_tlsctx_read_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_read_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_get_ref_t"
    )]
    pub type unitytls_key_get_ref_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_key_get_ref_t;
    #[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_free_t")]
    pub type unitytls_key_free_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_key_free_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_pem_t"
    )]
    pub type unitytls_key_parse_pem_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_key_parse_pem_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_der_t"
    )]
    pub type unitytls_x509list_append_der_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_append_der_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_process_handshake_t"
    )]
    pub type unitytls_tlsctx_process_handshake_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_process_handshake_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_der_t"
    )]
    pub type unitytls_key_parse_der_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_key_parse_der_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509_export_der_t"
    )]
    pub type unitytls_x509_export_der_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_x509_export_der_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_client_t"
    )]
    pub type unitytls_tlsctx_create_client_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_create_client_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_supported_ciphersuites_t"
    )]
    pub type unitytls_tlsctx_set_supported_ciphersuites_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_supported_ciphersuites_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_certificate_callback_t"
    )]
    pub type unitytls_tlsctx_set_certificate_callback_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_certificate_callback_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_ref_t"
    )]
    pub type unitytls_x509list_get_ref_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_get_ref_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_server_require_client_authentication_t"
    )]
    pub type unitytls_tlsctx_server_require_client_authentication_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_server_require_client_authentication_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_explicit_ca_t"
    )]
    pub type unitytls_x509verify_explicit_ca_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_x509verify_explicit_ca_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_x509_t"
    )]
    pub type unitytls_x509list_get_x509_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_get_x509_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_x509verify_callback_t"
    )]
    pub type unitytls_tlsctx_set_x509verify_callback_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_x509verify_callback_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_server_t"
    )]
    pub type unitytls_tlsctx_create_server_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_create_server_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_protocol_t"
    )]
    pub type unitytls_tlsctx_get_protocol_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_get_protocol_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_free_t"
    )]
    pub type unitytls_x509list_free_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_free_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_write_t"
    )]
    pub type unitytls_tlsctx_write_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_write_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_notify_close_t"
    )]
    pub type unitytls_tlsctx_notify_close_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_notify_close_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_random_generate_bytes_t"
    )]
    pub type unitytls_random_generate_bytes_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_random_generate_bytes_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_default_ca_t"
    )]
    pub type unitytls_x509verify_default_ca_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_x509verify_default_ca_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_raise_error_t"
    )]
    pub type unitytls_errorstate_raise_error_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_errorstate_raise_error_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_t"
    )]
    pub type unitytls_x509list_append_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_append_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_trace_callback_t"
    )]
    pub type unitytls_tlsctx_set_trace_callback_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_trace_callback_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_ciphersuite_t"
    )]
    pub type unitytls_tlsctx_get_ciphersuite_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_get_ciphersuite_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_free_t"
    )]
    pub type unitytls_tlsctx_free_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_free_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_errorstate_create_t"
    )]
    pub type unitytls_errorstate_create_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_errorstate_create_t;
    #[cfg(
        feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_create_t"
    )]
    pub type unitytls_x509list_create_t = crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_create_t;
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[derive(Debug, Clone)]
pub struct UnityTls_unitytls_key {}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_key")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::UnityTls_unitytls_key =>
    "Mono.Unity"."UnityTls/unitytls_key"
);
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_free_t")]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_unitytls_key_free_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_free_t")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_key_free_t => "Mono.Unity"
    ."UnityTls/unitytls_interface_struct/unitytls_key_free_t"
);
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_free_t")]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_key_free_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_free_t")]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_key_free_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_free_t")]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_key_free_t {
    pub fn Invoke(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (key))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_free_t")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_key_free_t {
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
pub struct unitytls_interface_struct_unitytls_key_get_ref_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_get_ref_t")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_key_get_ref_t => "Mono.Unity"
    ."UnityTls/unitytls_interface_struct/unitytls_key_get_ref_t"
);
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_get_ref_t")]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_key_get_ref_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_get_ref_t")]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_key_get_ref_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_get_ref_t")]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_key_get_ref_t {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppObject,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<crate::Mono::Unity::UnityTls_unitytls_key_ref> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_key_ref = __cordl_object
            .invoke("Invoke", (key, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_get_ref_t")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_key_get_ref_t {
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
pub struct unitytls_interface_struct_unitytls_key_parse_der_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_der_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_key_parse_der_t => "Mono.Unity"
    ."UnityTls/unitytls_interface_struct/unitytls_key_parse_der_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_der_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_key_parse_der_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_der_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_key_parse_der_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_der_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_key_parse_der_t {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppObject,
        bufferLen: crate::System::IntPtr,
        password: *mut quest_hook::libil2cpp::Il2CppObject,
        passwordLen: crate::System::IntPtr,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("Invoke", (buffer, bufferLen, password, passwordLen, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_der_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_key_parse_der_t {
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
pub struct unitytls_interface_struct_unitytls_key_parse_pem_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_pem_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_key_parse_pem_t => "Mono.Unity"
    ."UnityTls/unitytls_interface_struct/unitytls_key_parse_pem_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_pem_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_key_parse_pem_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_pem_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_key_parse_pem_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_pem_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_key_parse_pem_t {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppObject,
        bufferLen: crate::System::IntPtr,
        password: *mut quest_hook::libil2cpp::Il2CppObject,
        passwordLen: crate::System::IntPtr,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("Invoke", (buffer, bufferLen, password, passwordLen, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_key_parse_pem_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_key_parse_pem_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_key_ref")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct UnityTls_unitytls_key_ref {
    pub handle: u64,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_key_ref")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::UnityTls_unitytls_key_ref =>
    "Mono.Unity"."UnityTls/unitytls_key_ref"
);
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnityTls_unitytls_protocol {
    UNITYTLS_PROTOCOL_INVALID = 20483u32,
    UNITYTLS_PROTOCOL_TLS_1_0 = 50462976u32,
    UNITYTLS_PROTOCOL_TLS_1_1 = 1342374401u32,
    UNITYTLS_PROTOCOL_TLS_1_2 = 5243650u32,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_protocol")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::UnityTls_unitytls_protocol =>
    "Mono.Unity"."UnityTls/unitytls_protocol"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_random_generate_bytes_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_unitytls_random_generate_bytes_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_random_generate_bytes_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_random_generate_bytes_t =>
    "Mono.Unity"."UnityTls/unitytls_interface_struct/unitytls_random_generate_bytes_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_random_generate_bytes_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_random_generate_bytes_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_random_generate_bytes_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_random_generate_bytes_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_random_generate_bytes_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_random_generate_bytes_t {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppObject,
        bufferLen: crate::System::IntPtr,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (buffer, bufferLen, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_random_generate_bytes_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_random_generate_bytes_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct UnityTls_unitytls_tlsctx {}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::UnityTls_unitytls_tlsctx =>
    "Mono.Unity"."UnityTls/unitytls_tlsctx"
);
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
#[derive(Debug, Clone)]
pub struct UnityTls_unitytls_tlsctx_callbacks {
    pub read: *mut crate::Mono::Unity::UnityTls_unitytls_tlsctx_read_callback,
    pub write: *mut crate::Mono::Unity::UnityTls_unitytls_tlsctx_write_callback,
    pub data: *mut quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_callbacks")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::UnityTls_unitytls_tlsctx_callbacks
    => "Mono.Unity"."UnityTls/unitytls_tlsctx_callbacks"
);
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::UnityTls_unitytls_tlsctx_certificate_callback => "Mono.Unity"
    ."UnityTls/unitytls_tlsctx_certificate_callback"
);
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
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        userData: *mut quest_hook::libil2cpp::Il2CppObject,
        ctx: *mut quest_hook::libil2cpp::Il2CppObject,
        cn: *mut quest_hook::libil2cpp::Il2CppObject,
        cnLen: crate::System::IntPtr,
        caList: *mut quest_hook::libil2cpp::Il2CppObject,
        caListLen: crate::System::IntPtr,
        chain: *mut quest_hook::libil2cpp::Il2CppObject,
        key: *mut quest_hook::libil2cpp::Il2CppObject,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Invoke",
                (userData, ctx, cn, cnLen, caList, caListLen, chain, key, errorState),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_client_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_unitytls_tlsctx_create_client_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_client_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_create_client_t =>
    "Mono.Unity"."UnityTls/unitytls_interface_struct/unitytls_tlsctx_create_client_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_client_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_create_client_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_client_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_create_client_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_client_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_create_client_t {
    pub fn Invoke(
        &mut self,
        supportedProtocols: crate::Mono::Unity::UnityTls_unitytls_tlsctx_protocolrange,
        callbacks: crate::Mono::Unity::UnityTls_unitytls_tlsctx_callbacks,
        cn: *mut quest_hook::libil2cpp::Il2CppObject,
        cnLen: crate::System::IntPtr,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("Invoke", (supportedProtocols, callbacks, cn, cnLen, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_client_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_create_client_t {
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
pub struct unitytls_interface_struct_unitytls_tlsctx_create_server_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_server_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_create_server_t =>
    "Mono.Unity"."UnityTls/unitytls_interface_struct/unitytls_tlsctx_create_server_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_server_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_create_server_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_server_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_create_server_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_server_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_create_server_t {
    pub fn Invoke(
        &mut self,
        supportedProtocols: crate::Mono::Unity::UnityTls_unitytls_tlsctx_protocolrange,
        callbacks: crate::Mono::Unity::UnityTls_unitytls_tlsctx_callbacks,
        certChain: u64,
        leafCertificateKey: u64,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke(
                "Invoke",
                (
                    supportedProtocols,
                    callbacks,
                    certChain,
                    leafCertificateKey,
                    errorState,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_create_server_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_create_server_t {
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
pub struct unitytls_interface_struct_unitytls_tlsctx_free_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_free_t")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_free_t => "Mono.Unity"
    ."UnityTls/unitytls_interface_struct/unitytls_tlsctx_free_t"
);
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_free_t")]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_free_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_free_t")]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_free_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_free_t")]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_free_t {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        ctx: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (ctx))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_free_t")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_free_t {
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
pub struct unitytls_interface_struct_unitytls_tlsctx_get_ciphersuite_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_ciphersuite_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_get_ciphersuite_t =>
    "Mono.Unity"."UnityTls/unitytls_interface_struct/unitytls_tlsctx_get_ciphersuite_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_ciphersuite_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_get_ciphersuite_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_ciphersuite_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_get_ciphersuite_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_ciphersuite_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_get_ciphersuite_t {
    pub fn Invoke(
        &mut self,
        ctx: *mut quest_hook::libil2cpp::Il2CppObject,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Unity::UnityTls_unitytls_ciphersuite,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_ciphersuite = __cordl_object
            .invoke("Invoke", (ctx, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_ciphersuite_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_get_ciphersuite_t {
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
pub struct unitytls_interface_struct_unitytls_tlsctx_get_protocol_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_protocol_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_get_protocol_t =>
    "Mono.Unity"."UnityTls/unitytls_interface_struct/unitytls_tlsctx_get_protocol_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_protocol_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_get_protocol_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_protocol_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_get_protocol_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_protocol_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_get_protocol_t {
    pub fn Invoke(
        &mut self,
        ctx: *mut quest_hook::libil2cpp::Il2CppObject,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<crate::Mono::Unity::UnityTls_unitytls_protocol> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_protocol = __cordl_object
            .invoke("Invoke", (ctx, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_get_protocol_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_get_protocol_t {
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
pub struct unitytls_interface_struct_unitytls_tlsctx_notify_close_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_notify_close_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_notify_close_t =>
    "Mono.Unity"."UnityTls/unitytls_interface_struct/unitytls_tlsctx_notify_close_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_notify_close_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_notify_close_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_notify_close_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_notify_close_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_notify_close_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_notify_close_t {
    pub fn Invoke(
        &mut self,
        ctx: *mut quest_hook::libil2cpp::Il2CppObject,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (ctx, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_notify_close_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_notify_close_t {
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
pub struct unitytls_interface_struct_unitytls_tlsctx_process_handshake_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_process_handshake_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_process_handshake_t =>
    "Mono.Unity"."UnityTls/unitytls_interface_struct/unitytls_tlsctx_process_handshake_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_process_handshake_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_process_handshake_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_process_handshake_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_process_handshake_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_process_handshake_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_process_handshake_t {
    pub fn Invoke(
        &mut self,
        ctx: *mut quest_hook::libil2cpp::Il2CppObject,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_x509verify_result = __cordl_object
            .invoke("Invoke", (ctx, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_process_handshake_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_process_handshake_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_protocolrange")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct UnityTls_unitytls_tlsctx_protocolrange {
    pub min: crate::Mono::Unity::UnityTls_unitytls_protocol,
    pub max: crate::Mono::Unity::UnityTls_unitytls_protocol,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_tlsctx_protocolrange")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::UnityTls_unitytls_tlsctx_protocolrange => "Mono.Unity"
    ."UnityTls/unitytls_tlsctx_protocolrange"
);
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::UnityTls_unitytls_tlsctx_read_callback => "Mono.Unity"
    ."UnityTls/unitytls_tlsctx_read_callback"
);
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
        userData: *mut quest_hook::libil2cpp::Il2CppObject,
        buffer: *mut quest_hook::libil2cpp::Il2CppObject,
        bufferLen: crate::System::IntPtr,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("Invoke", (userData, buffer, bufferLen, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_read_t")]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_unitytls_tlsctx_read_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_read_t")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_read_t => "Mono.Unity"
    ."UnityTls/unitytls_interface_struct/unitytls_tlsctx_read_t"
);
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_read_t")]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_read_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_read_t")]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_read_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_read_t")]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_read_t {
    pub fn Invoke(
        &mut self,
        ctx: *mut quest_hook::libil2cpp::Il2CppObject,
        buffer: *mut quest_hook::libil2cpp::Il2CppObject,
        bufferLen: crate::System::IntPtr,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("Invoke", (ctx, buffer, bufferLen, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_read_t")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_read_t {
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
pub struct unitytls_interface_struct_unitytls_tlsctx_server_require_client_authentication_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_server_require_client_authentication_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_server_require_client_authentication_t
    => "Mono.Unity"
    ."UnityTls/unitytls_interface_struct/unitytls_tlsctx_server_require_client_authentication_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_server_require_client_authentication_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_server_require_client_authentication_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_server_require_client_authentication_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_server_require_client_authentication_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_server_require_client_authentication_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_server_require_client_authentication_t {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        ctx: *mut quest_hook::libil2cpp::Il2CppObject,
        clientAuthCAList: crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (ctx, clientAuthCAList, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_server_require_client_authentication_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_server_require_client_authentication_t {
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
pub struct unitytls_interface_struct_unitytls_tlsctx_set_certificate_callback_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_certificate_callback_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_certificate_callback_t
    => "Mono.Unity"
    ."UnityTls/unitytls_interface_struct/unitytls_tlsctx_set_certificate_callback_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_certificate_callback_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_certificate_callback_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_certificate_callback_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_certificate_callback_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_certificate_callback_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_certificate_callback_t {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        ctx: *mut quest_hook::libil2cpp::Il2CppObject,
        cb: *mut crate::Mono::Unity::UnityTls_unitytls_tlsctx_certificate_callback,
        userData: *mut quest_hook::libil2cpp::Il2CppObject,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (ctx, cb, userData, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_certificate_callback_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_certificate_callback_t {
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
pub struct unitytls_interface_struct_unitytls_tlsctx_set_supported_ciphersuites_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_supported_ciphersuites_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_supported_ciphersuites_t
    => "Mono.Unity"
    ."UnityTls/unitytls_interface_struct/unitytls_tlsctx_set_supported_ciphersuites_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_supported_ciphersuites_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_supported_ciphersuites_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_supported_ciphersuites_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_supported_ciphersuites_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_supported_ciphersuites_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_supported_ciphersuites_t {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        ctx: *mut quest_hook::libil2cpp::Il2CppObject,
        supportedCiphersuites: *mut quest_hook::libil2cpp::Il2CppObject,
        supportedCiphersuitesLen: crate::System::IntPtr,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Invoke",
                (ctx, supportedCiphersuites, supportedCiphersuitesLen, errorState),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_supported_ciphersuites_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_supported_ciphersuites_t {
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
pub struct unitytls_interface_struct_unitytls_tlsctx_set_trace_callback_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_trace_callback_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_trace_callback_t =>
    "Mono.Unity"
    ."UnityTls/unitytls_interface_struct/unitytls_tlsctx_set_trace_callback_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_trace_callback_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_trace_callback_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_trace_callback_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_trace_callback_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_trace_callback_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_trace_callback_t {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        ctx: *mut quest_hook::libil2cpp::Il2CppObject,
        cb: *mut crate::Mono::Unity::UnityTls_unitytls_tlsctx_trace_callback,
        userData: *mut quest_hook::libil2cpp::Il2CppObject,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (ctx, cb, userData, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_trace_callback_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_trace_callback_t {
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
pub struct unitytls_interface_struct_unitytls_tlsctx_set_x509verify_callback_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_x509verify_callback_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_x509verify_callback_t =>
    "Mono.Unity"
    ."UnityTls/unitytls_interface_struct/unitytls_tlsctx_set_x509verify_callback_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_x509verify_callback_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_x509verify_callback_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_x509verify_callback_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_x509verify_callback_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_x509verify_callback_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_x509verify_callback_t {
    pub fn Invoke(
        &mut self,
        ctx: *mut quest_hook::libil2cpp::Il2CppObject,
        cb: *mut crate::Mono::Unity::UnityTls_unitytls_tlsctx_x509verify_callback,
        userData: *mut quest_hook::libil2cpp::Il2CppObject,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (ctx, cb, userData, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_set_x509verify_callback_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_set_x509verify_callback_t {
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::UnityTls_unitytls_tlsctx_trace_callback => "Mono.Unity"
    ."UnityTls/unitytls_tlsctx_trace_callback"
);
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
        userData: *mut quest_hook::libil2cpp::Il2CppObject,
        ctx: *mut quest_hook::libil2cpp::Il2CppObject,
        traceMessage: *mut quest_hook::libil2cpp::Il2CppObject,
        traceMessageLen: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (userData, ctx, traceMessage, traceMessageLen))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::UnityTls_unitytls_tlsctx_write_callback => "Mono.Unity"
    ."UnityTls/unitytls_tlsctx_write_callback"
);
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
        userData: *mut quest_hook::libil2cpp::Il2CppObject,
        data: *mut quest_hook::libil2cpp::Il2CppObject,
        bufferLen: crate::System::IntPtr,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("Invoke", (userData, data, bufferLen, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
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
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_write_t")]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_unitytls_tlsctx_write_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_write_t")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_write_t => "Mono.Unity"
    ."UnityTls/unitytls_interface_struct/unitytls_tlsctx_write_t"
);
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_write_t")]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_write_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_write_t")]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_write_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_write_t")]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_write_t {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        ctx: *mut quest_hook::libil2cpp::Il2CppObject,
        data: *mut quest_hook::libil2cpp::Il2CppObject,
        bufferLen: crate::System::IntPtr,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("Invoke", (ctx, data, bufferLen, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_tlsctx_write_t")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_tlsctx_write_t {
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::UnityTls_unitytls_tlsctx_x509verify_callback => "Mono.Unity"
    ."UnityTls/unitytls_tlsctx_x509verify_callback"
);
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
        userData: *mut quest_hook::libil2cpp::Il2CppObject,
        chain: crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_x509verify_result = __cordl_object
            .invoke("Invoke", (userData, chain, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509_export_der_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_unitytls_x509_export_der_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509_export_der_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_x509_export_der_t => "Mono.Unity"
    ."UnityTls/unitytls_interface_struct/unitytls_x509_export_der_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509_export_der_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509_export_der_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509_export_der_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509_export_der_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509_export_der_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_x509_export_der_t {
    pub fn Invoke(
        &mut self,
        cert: crate::Mono::Unity::UnityTls_unitytls_x509_ref,
        buffer: *mut quest_hook::libil2cpp::Il2CppObject,
        bufferLen: crate::System::IntPtr,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("Invoke", (cert, buffer, bufferLen, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509_export_der_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509_export_der_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509_ref")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct UnityTls_unitytls_x509_ref {
    pub handle: u64,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509_ref")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::UnityTls_unitytls_x509_ref =>
    "Mono.Unity"."UnityTls/unitytls_x509_ref"
);
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
#[derive(Debug, Clone)]
pub struct UnityTls_unitytls_x509list {}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::UnityTls_unitytls_x509list =>
    "Mono.Unity"."UnityTls/unitytls_x509list"
);
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_der_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_unitytls_x509list_append_der_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_der_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_x509list_append_der_t =>
    "Mono.Unity"."UnityTls/unitytls_interface_struct/unitytls_x509list_append_der_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_der_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_append_der_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_der_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_append_der_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_der_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_append_der_t {
    pub fn Invoke(
        &mut self,
        list: *mut quest_hook::libil2cpp::Il2CppObject,
        buffer: *mut quest_hook::libil2cpp::Il2CppObject,
        bufferLen: crate::System::IntPtr,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (list, buffer, bufferLen, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_der_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_append_der_t {
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
pub struct unitytls_interface_struct_unitytls_x509list_append_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_x509list_append_t => "Mono.Unity"
    ."UnityTls/unitytls_interface_struct/unitytls_x509list_append_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_append_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_append_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_append_t {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        list: *mut quest_hook::libil2cpp::Il2CppObject,
        cert: crate::Mono::Unity::UnityTls_unitytls_x509_ref,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (list, cert, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_append_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_append_t {
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
pub struct unitytls_interface_struct_unitytls_x509list_create_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_create_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_x509list_create_t => "Mono.Unity"
    ."UnityTls/unitytls_interface_struct/unitytls_x509list_create_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_create_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_create_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_create_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_create_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_create_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_create_t {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("Invoke", (errorState))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_create_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_create_t {
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
pub struct unitytls_interface_struct_unitytls_x509list_free_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_free_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_x509list_free_t => "Mono.Unity"
    ."UnityTls/unitytls_interface_struct/unitytls_x509list_free_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_free_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_free_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_free_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_free_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_free_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_free_t {
    pub fn Invoke(
        &mut self,
        list: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (list))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_free_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_free_t {
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
pub struct unitytls_interface_struct_unitytls_x509list_get_ref_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_ref_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_x509list_get_ref_t => "Mono.Unity"
    ."UnityTls/unitytls_interface_struct/unitytls_x509list_get_ref_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_ref_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_get_ref_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_ref_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_get_ref_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_ref_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_get_ref_t {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        list: *mut quest_hook::libil2cpp::Il2CppObject,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_x509list_ref = __cordl_object
            .invoke("Invoke", (list, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_ref_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_get_ref_t {
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
pub struct unitytls_interface_struct_unitytls_x509list_get_x509_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_x509_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_x509list_get_x509_t => "Mono.Unity"
    ."UnityTls/unitytls_interface_struct/unitytls_x509list_get_x509_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_x509_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_get_x509_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_x509_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_get_x509_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_x509_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_get_x509_t {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        list: crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
        index: crate::System::IntPtr,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<crate::Mono::Unity::UnityTls_unitytls_x509_ref> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_x509_ref = __cordl_object
            .invoke("Invoke", (list, index, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509list_get_x509_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509list_get_x509_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list_ref")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct UnityTls_unitytls_x509list_ref {
    pub handle: u64,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509list_ref")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::UnityTls_unitytls_x509list_ref =>
    "Mono.Unity"."UnityTls/unitytls_x509list_ref"
);
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
#[derive(Debug, Clone)]
pub struct UnityTls_unitytls_x509name {}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509name")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::UnityTls_unitytls_x509name =>
    "Mono.Unity"."UnityTls/unitytls_x509name"
);
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::UnityTls_unitytls_x509verify_callback => "Mono.Unity"
    ."UnityTls/unitytls_x509verify_callback"
);
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
        userData: *mut quest_hook::libil2cpp::Il2CppObject,
        cert: crate::Mono::Unity::UnityTls_unitytls_x509_ref,
        result: crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_x509verify_result = __cordl_object
            .invoke("Invoke", (userData, cert, result, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
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
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_default_ca_t"
)]
#[repr(C)]
#[derive(Debug)]
pub struct unitytls_interface_struct_unitytls_x509verify_default_ca_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_default_ca_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_x509verify_default_ca_t =>
    "Mono.Unity"."UnityTls/unitytls_interface_struct/unitytls_x509verify_default_ca_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_default_ca_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509verify_default_ca_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_default_ca_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509verify_default_ca_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_default_ca_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_x509verify_default_ca_t {
    pub fn Invoke(
        &mut self,
        chain: crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
        cn: *mut quest_hook::libil2cpp::Il2CppObject,
        cnLen: crate::System::IntPtr,
        cb: *mut crate::Mono::Unity::UnityTls_unitytls_x509verify_callback,
        userData: *mut quest_hook::libil2cpp::Il2CppObject,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_x509verify_result = __cordl_object
            .invoke("Invoke", (chain, cn, cnLen, cb, userData, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_default_ca_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509verify_default_ca_t {
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
pub struct unitytls_interface_struct_unitytls_x509verify_explicit_ca_t {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_explicit_ca_t"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Unity::unitytls_interface_struct_unitytls_x509verify_explicit_ca_t =>
    "Mono.Unity"."UnityTls/unitytls_interface_struct/unitytls_x509verify_explicit_ca_t"
);
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_explicit_ca_t"
)]
impl std::ops::Deref
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509verify_explicit_ca_t {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_explicit_ca_t"
)]
impl std::ops::DerefMut
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509verify_explicit_ca_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_explicit_ca_t"
)]
impl crate::Mono::Unity::unitytls_interface_struct_unitytls_x509verify_explicit_ca_t {
    pub fn Invoke(
        &mut self,
        chain: crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
        trustCA: crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
        cn: *mut quest_hook::libil2cpp::Il2CppObject,
        cnLen: crate::System::IntPtr,
        cb: *mut crate::Mono::Unity::UnityTls_unitytls_x509verify_callback,
        userData: *mut quest_hook::libil2cpp::Il2CppObject,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_x509verify_result = __cordl_object
            .invoke("Invoke", (chain, trustCA, cn, cnLen, cb, userData, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(
    feature = "Mono+Unity+UnityTls+unitytls_interface_struct+unitytls_x509verify_explicit_ca_t"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Unity::unitytls_interface_struct_unitytls_x509verify_explicit_ca_t {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509verify_result")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnityTls_unitytls_x509verify_result {
    UNITYTLS_X509VERIFY_FATAL_ERROR = 67240447u32,
    UNITYTLS_X509VERIFY_FLAG_CN_MISMATCH = 29362180u32,
    UNITYTLS_X509VERIFY_FLAG_EXPIRED = 134480385u32,
    UNITYTLS_X509VERIFY_FLAG_NOT_TRUSTED = 114696u32,
    UNITYTLS_X509VERIFY_FLAG_REVOKED = 3221750786u32,
    UNITYTLS_X509VERIFY_FLAG_UNKNOWN_ERROR = 200u32,
    UNITYTLS_X509VERIFY_FLAG_USER_ERROR1 = 448u32,
    UNITYTLS_X509VERIFY_FLAG_USER_ERROR2 = 704u32,
    UNITYTLS_X509VERIFY_FLAG_USER_ERROR3 = 1216u32,
    UNITYTLS_X509VERIFY_FLAG_USER_ERROR4 = 2240u32,
    UNITYTLS_X509VERIFY_FLAG_USER_ERROR5 = 4288u32,
    UNITYTLS_X509VERIFY_FLAG_USER_ERROR6 = 8384u32,
    UNITYTLS_X509VERIFY_FLAG_USER_ERROR7 = 16576u32,
    UNITYTLS_X509VERIFY_FLAG_USER_ERROR8 = 32960u32,
    UNITYTLS_X509VERIFY_NOT_DONE = 240u32,
    UNITYTLS_X509VERIFY_SUCCESS = 61440u32,
}
#[cfg(feature = "Mono+Unity+UnityTls+unitytls_x509verify_result")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::UnityTls_unitytls_x509verify_result
    => "Mono.Unity"."UnityTls/unitytls_x509verify_result"
);
