#[cfg(feature = "ENet+ENetSslConfiguration")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ENetSslConfiguration {
    pub mode: crate::ENet::SslMode,
    pub certificatePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub certificate: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub privateKeyPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub privateKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub validateCertificate: i32,
    pub rootCertificatePath: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub rootCertificate: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub hostName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "ENet+ENetSslConfiguration")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::ENet::ENetSslConfiguration => "ENet"
    ."ENetSslConfiguration"
);
#[cfg(feature = "ENet+ENetSslConfiguration")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::ENet::ENetSslConfiguration {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ENet+ENetSslConfiguration")]
impl crate::ENet::ENetSslConfiguration {}
