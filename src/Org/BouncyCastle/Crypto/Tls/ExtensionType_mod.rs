#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Tls+ExtensionType")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct ExtensionType {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Tls+ExtensionType")]
unsafe impl quest_hook::libil2cpp::Type for crate::Org::BouncyCastle::Crypto::Tls::ExtensionType {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Tls";
    const CLASS_NAME: &'static str = "ExtensionType";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ExtensionType")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::ExtensionType {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ExtensionType")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Tls::ExtensionType {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+ExtensionType")]
impl crate::Org::BouncyCastle::Crypto::Tls::ExtensionType {
    pub const application_layer_protocol_negotiation: i32 = 16i32;
    pub const cached_info: i32 = 25i32;
    pub const cert_type: i32 = 9i32;
    pub const client_authz: i32 = 7i32;
    pub const client_certificate_type: i32 = 19i32;
    pub const client_certificate_url: i32 = 2i32;
    pub const ec_point_formats: i32 = 11i32;
    pub const elliptic_curves: i32 = 10i32;
    pub const encrypt_then_mac: i32 = 22i32;
    pub const extended_master_secret: i32 = 23i32;
    pub const heartbeat: i32 = 15i32;
    pub const max_fragment_length: i32 = 1i32;
    pub const padding: i32 = 21i32;
    pub const renegotiation_info: i32 = 65281i32;
    pub const server_authz: i32 = 8i32;
    pub const server_certificate_type: i32 = 20i32;
    pub const server_name: i32 = 0i32;
    pub const session_ticket: i32 = 35i32;
    pub const signature_algorithms: i32 = 13i32;
    pub const signed_certificate_timestamp: i32 = 18i32;
    pub const srp: i32 = 12i32;
    pub const status_request: i32 = 5i32;
    pub const status_request_v2: i32 = 17i32;
    pub const supported_groups: i32 = 10i32;
    pub const truncated_hmac: i32 = 4i32;
    pub const trusted_ca_keys: i32 = 3i32;
    pub const use_srtp: i32 = 14i32;
    pub const user_mapping: i32 = 6i32;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Org+BouncyCastle+Crypto+Tls+ExtensionType")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Crypto::Tls::ExtensionType {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
