#[cfg(feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpRequestManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub sessionConfig_: crate::OculusStudios::GraphQL::Client::HttpRequestManager_SessionConfig,
}
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OculusStudios::GraphQL::Client::HttpRequestManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OculusStudios.GraphQL.Client";
    const CLASS_NAME: &'static str = "HttpRequestManager";
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
#[cfg(feature = "OculusStudios+GraphQL+Client+HttpRequestManager")]
impl std::ops::Deref for crate::OculusStudios::GraphQL::Client::HttpRequestManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+HttpRequestManager")]
impl std::ops::DerefMut for crate::OculusStudios::GraphQL::Client::HttpRequestManager {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+HttpRequestManager")]
impl crate::OculusStudios::GraphQL::Client::HttpRequestManager {
    pub const FAST_FALLBACK_TEST_HOST: &'static str = "graph.facebook.com";
    pub const FAST_FALLBACK_TEST_PORT: i32 = 443i32;
    pub const FAST_FALLBACK_TEST_THRESHOLD_MS: i32 = 200i32;
    pub const FAST_FALLBACK_TEST_WAIT_INTERVAL_MS: i32 = 200i32;
    pub const QUERY_TIMEOUT: f32 = 30f32;
    #[cfg(
        feature = "OculusStudios+GraphQL+Client+HttpRequestManager+ByteArrayResultCallback"
    )]
    pub type ByteArrayResultCallback = crate::OculusStudios::GraphQL::Client::HttpRequestManager_ByteArrayResultCallback;
    #[cfg(feature = "OculusStudios+GraphQL+Client+HttpRequestManager+GlobalConfig")]
    pub type GlobalConfig = crate::OculusStudios::GraphQL::Client::HttpRequestManager_GlobalConfig;
    #[cfg(feature = "OculusStudios+GraphQL+Client+HttpRequestManager+SessionConfig")]
    pub type SessionConfig = crate::OculusStudios::GraphQL::Client::HttpRequestManager_SessionConfig;
    #[cfg(feature = "OculusStudios+GraphQL+Client+HttpRequestManager+fastFallbackState")]
    pub type fastFallbackState = crate::OculusStudios::GraphQL::Client::HttpRequestManager_fastFallbackState;
    pub fn BindTo(
        servicePoint: quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint>,
        remoteEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
        retryCount: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Net::ServicePoint>,
                            quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
                        3usize,
                    >("BindTo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "BindTo",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint> = unsafe {
            cordl_method_info
                .invoke_unchecked((), (servicePoint, remoteEndPoint, retryCount))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckAndConsiderFastFallbackAsync(
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Uri>),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        1usize,
                    >("CheckAndConsiderFastFallbackAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckAndConsiderFastFallbackAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe { cordl_method_info.invoke_unchecked((), (uri))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckFastFallbackAsync() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                        0usize,
                    >("CheckFastFallbackAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckFastFallbackAsync", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckFastFallbackNoWait() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("CheckFastFallbackNoWait")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckFastFallbackNoWait", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConsiderFastFallback(
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Uri>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ConsiderFastFallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConsiderFastFallback", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (uri))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateByteArrayRequest(
        &mut self,
        method: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpMethod>,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        accept: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        contentType: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        content: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpRequestMessage>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Net::Http::HttpMethod,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Uri>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Net::Http::HttpRequestMessage,
                        >,
                        5usize,
                    >("CreateByteArrayRequest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateByteArrayRequest", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Http::HttpRequestMessage,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (method, uri, accept, contentType, content))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateHttpClient() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("CreateHttpClient")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateHttpClient", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRequest(
        &mut self,
        method: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpMethod>,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpRequestMessage>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Net::Http::HttpMethod,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Uri>,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Net::Http::HttpRequestMessage,
                        >,
                        2usize,
                    >("CreateRequest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateRequest", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Http::HttpRequestMessage,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (method, uri))? };
        Ok(__cordl_ret.into())
    }
    pub fn MyCertHandler(
        sender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        chain: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Chain,
        >,
        error: crate::System::Net::Security::SslPolicyErrors,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Cryptography::X509Certificates::X509Certificate,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Security::Cryptography::X509Certificates::X509Chain,
                            >,
                            crate::System::Net::Security::SslPolicyErrors,
                        ),
                        bool,
                        4usize,
                    >("MyCertHandler")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MyCertHandler", 4usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (sender, certificate, chain, error))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_HttpMessageHandler2(
        handler: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpMessageHandler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (handler))?;
        Ok(__cordl_object.into())
    }
    pub fn New_HttpRequestManager_SessionConfig1(
        config: crate::OculusStudios::GraphQL::Client::HttpRequestManager_SessionConfig,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (config))?;
        Ok(__cordl_object.into())
    }
    pub fn OpenPersistentStream(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpRequestMessage>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpResponseMessage>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Net::Http::HttpRequestMessage,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Net::Http::HttpResponseMessage,
                                >,
                            >,
                        >,
                        1usize,
                    >("OpenPersistentStream")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OpenPersistentStream", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpResponseMessage>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (request))? };
        Ok(__cordl_ret.into())
    }
    pub fn ResetFastFallback() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("ResetFastFallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResetFastFallback", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SendAsync(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpRequestMessage>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpResponseMessage>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Net::Http::HttpRequestMessage,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Net::Http::HttpResponseMessage,
                                >,
                            >,
                        >,
                        1usize,
                    >("SendAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SendAsync", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpResponseMessage>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (request))? };
        Ok(__cordl_ret.into())
    }
    pub fn SendAsyncInternal(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpRequestMessage>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpResponseMessage>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Net::Http::HttpRequestMessage,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Net::Http::HttpResponseMessage,
                                >,
                            >,
                        >,
                        1usize,
                    >("SendAsyncInternal")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SendAsyncInternal", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpResponseMessage>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (request))? };
        Ok(__cordl_ret.into())
    }
    pub fn SendAsyncReceiveByteArray(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpRequestMessage>,
        callback: quest_hook::libil2cpp::Gc<
            crate::OculusStudios::GraphQL::Client::HttpRequestManager_ByteArrayResultCallback,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Net::Http::HttpRequestMessage,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::OculusStudios::GraphQL::Client::HttpRequestManager_ByteArrayResultCallback,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SendAsyncReceiveByteArray")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SendAsyncReceiveByteArray", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (request, callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateGlobalConfig(
        config: crate::OculusStudios::GraphQL::Client::HttpRequestManager_GlobalConfig,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::OculusStudios::GraphQL::Client::HttpRequestManager_GlobalConfig),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UpdateGlobalConfig")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateGlobalConfig", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (config))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateSessionConfig(
        &mut self,
        config: crate::OculusStudios::GraphQL::Client::HttpRequestManager_SessionConfig,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::OculusStudios::GraphQL::Client::HttpRequestManager_SessionConfig),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UpdateSessionConfig")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateSessionConfig", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (config))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_HttpMessageHandler2(
        &mut self,
        handler: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpMessageHandler>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Net::Http::HttpMessageHandler,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (handler))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_HttpRequestManager_SessionConfig1(
        &mut self,
        config: crate::OculusStudios::GraphQL::Client::HttpRequestManager_SessionConfig,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::OculusStudios::GraphQL::Client::HttpRequestManager_SessionConfig),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (config))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_currentFastFallbackState() -> quest_hook::libil2cpp::Result<
        crate::OculusStudios::GraphQL::Client::HttpRequestManager_fastFallbackState,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::OculusStudios::GraphQL::Client::HttpRequestManager_fastFallbackState,
                        0usize,
                    >("get_currentFastFallbackState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_currentFastFallbackState", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::OculusStudios::GraphQL::Client::HttpRequestManager_fastFallbackState = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_fastFallbackChecked() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("get_fastFallbackChecked")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_fastFallbackChecked", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_globalConfig() -> quest_hook::libil2cpp::Result<
        crate::OculusStudios::GraphQL::Client::HttpRequestManager_GlobalConfig,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::OculusStudios::GraphQL::Client::HttpRequestManager_GlobalConfig,
                        0usize,
                    >("get_globalConfig")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_globalConfig", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::OculusStudios::GraphQL::Client::HttpRequestManager_GlobalConfig = unsafe {
            cordl_method_info.invoke_unchecked((), ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_sessionConfig(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::OculusStudios::GraphQL::Client::HttpRequestManager_SessionConfig,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::OculusStudios::GraphQL::Client::HttpRequestManager_SessionConfig,
                        0usize,
                    >("get_sessionConfig")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_sessionConfig", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::OculusStudios::GraphQL::Client::HttpRequestManager_SessionConfig = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::OculusStudios::GraphQL::Client::HttpRequestManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+ByteArrayResultCallback"
)]
#[repr(C)]
#[derive(Debug)]
pub struct HttpRequestManager_ByteArrayResultCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+ByteArrayResultCallback"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_ByteArrayResultCallback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OculusStudios.GraphQL.Client";
    const CLASS_NAME: &'static str = "HttpRequestManager/ByteArrayResultCallback";
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
    feature = "OculusStudios+GraphQL+Client+HttpRequestManager+ByteArrayResultCallback"
)]
impl std::ops::Deref
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_ByteArrayResultCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "OculusStudios+GraphQL+Client+HttpRequestManager+ByteArrayResultCallback"
)]
impl std::ops::DerefMut
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_ByteArrayResultCallback {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "OculusStudios+GraphQL+Client+HttpRequestManager+ByteArrayResultCallback"
)]
impl crate::OculusStudios::GraphQL::Client::HttpRequestManager_ByteArrayResultCallback {
    pub fn BeginInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        statusCode: crate::System::Net::HttpStatusCode,
        reasonPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            crate::System::Net::HttpStatusCode,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Exception>,
                            quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
                        6usize,
                    >("BeginInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BeginInvoke", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (result, statusCode, reasonPhrase, exception, callback, object),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("EndInvoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "EndInvoke", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (result))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        statusCode: crate::System::Net::HttpStatusCode,
        reasonPhrase: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        exception: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            crate::System::Net::HttpStatusCode,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Invoke",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (result, statusCode, reasonPhrase, exception))?
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
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
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
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (object, method))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+ByteArrayResultCallback"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_ByteArrayResultCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+GlobalConfig"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HttpRequestManager_GlobalConfig {
    pub connectionLimit: i32,
    pub defaultRequestTimeout: crate::System::TimeSpan,
    pub forceIPV6: bool,
}
#[cfg(
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+GlobalConfig"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_GlobalConfig {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OculusStudios.GraphQL.Client";
    const CLASS_NAME: &'static str = "HttpRequestManager/GlobalConfig";
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
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+GlobalConfig"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_GlobalConfig {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+GlobalConfig"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_GlobalConfig {
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
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+GlobalConfig"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_GlobalConfig {
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
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+GlobalConfig"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_GlobalConfig {
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
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+GlobalConfig"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_GlobalConfig {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+HttpRequestManager+GlobalConfig")]
impl crate::OculusStudios::GraphQL::Client::HttpRequestManager_GlobalConfig {}
#[cfg(
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+SessionConfig"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HttpRequestManager_SessionConfig {
    pub baseAddress: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    pub requestTimeout: crate::System::TimeSpan,
}
#[cfg(
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+SessionConfig"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_SessionConfig {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OculusStudios.GraphQL.Client";
    const CLASS_NAME: &'static str = "HttpRequestManager/SessionConfig";
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
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+SessionConfig"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_SessionConfig {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+SessionConfig"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_SessionConfig {
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
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+SessionConfig"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_SessionConfig {
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
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+SessionConfig"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_SessionConfig {
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
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+SessionConfig"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_SessionConfig {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+HttpRequestManager+SessionConfig")]
impl crate::OculusStudios::GraphQL::Client::HttpRequestManager_SessionConfig {}
#[cfg(
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+fastFallbackState"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HttpRequestManager_fastFallbackState {
    #[default]
    CHECKING = 1i32,
    ERROR_DNS = 7i32,
    IPV4 = 6i32,
    NEEDS_CHECK = 0i32,
    NOT_REQUIRED_DNS_ONLY_IPV4 = 3i32,
    NOT_REQUIRED_DNS_ONLY_IPV6 = 4i32,
    NOT_REQUIRED_IPV6_OK = 2i32,
    NOT_REQUIRED_NO_IPV6 = 5i32,
}
#[cfg(
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+fastFallbackState"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_fastFallbackState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OculusStudios.GraphQL.Client";
    const CLASS_NAME: &'static str = "HttpRequestManager/fastFallbackState";
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
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+fastFallbackState"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_fastFallbackState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+fastFallbackState"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_fastFallbackState {
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
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+fastFallbackState"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_fastFallbackState {
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
    feature = "cordl_class_OculusStudios+GraphQL+Client+HttpRequestManager+fastFallbackState"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::OculusStudios::GraphQL::Client::HttpRequestManager_fastFallbackState {
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
