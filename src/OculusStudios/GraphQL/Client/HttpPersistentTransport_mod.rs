#[cfg(feature = "cordl_class_OculusStudios+GraphQL+Client+HttpPersistentTransport")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpPersistentTransport {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Endpoint_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _AccessToken_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub CustomAppHeaders: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IDictionary_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub logger: quest_hook::libil2cpp::Gc<
        crate::OculusStudios::GraphQL::ClientInterface::IGraphQLClientEvents,
    >,
    pub http_: quest_hook::libil2cpp::Gc<
        crate::OculusStudios::GraphQL::Client::HttpRequestManager,
    >,
}
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+Client+HttpPersistentTransport")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OculusStudios::GraphQL::Client::HttpPersistentTransport {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "OculusStudios.GraphQL.Client";
    const CLASS_NAME: &'static str = "HttpPersistentTransport";
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
#[cfg(feature = "OculusStudios+GraphQL+Client+HttpPersistentTransport")]
impl std::ops::Deref for crate::OculusStudios::GraphQL::Client::HttpPersistentTransport {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+HttpPersistentTransport")]
impl std::ops::DerefMut
for crate::OculusStudios::GraphQL::Client::HttpPersistentTransport {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+HttpPersistentTransport")]
impl crate::OculusStudios::GraphQL::Client::HttpPersistentTransport {
    pub const BUFF_SIZE: i32 = 1024i32;
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExecuteAsync(
        &mut self,
        graphQLRequest: quest_hook::libil2cpp::Gc<
            crate::OculusStudios::GraphQL::Client::GraphQLRequest,
        >,
        MinimalMainThreadExecutor: quest_hook::libil2cpp::Gc<
            crate::OculusStudios::GraphQL::Client::MinimalMainThreadExecutor,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::OculusStudios::GraphQL::Client::GraphQLResponseStream,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::OculusStudios::GraphQL::Client::GraphQLRequest,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::OculusStudios::GraphQL::Client::MinimalMainThreadExecutor,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::OculusStudios::GraphQL::Client::GraphQLResponseStream,
                                >,
                            >,
                        >,
                        2usize,
                    >("ExecuteAsync")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExecuteAsync", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::OculusStudios::GraphQL::Client::GraphQLResponseStream,
                >,
            >,
        > = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (graphQLRequest, MinimalMainThreadExecutor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetHttpRequest(
        &mut self,
        graphQLRequest: quest_hook::libil2cpp::Gc<
            crate::OculusStudios::GraphQL::Client::GraphQLRequest,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpRequestMessage>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::OculusStudios::GraphQL::Client::GraphQLRequest,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Net::Http::HttpRequestMessage,
                        >,
                        1usize,
                    >("GetHttpRequest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetHttpRequest", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Http::HttpRequestMessage,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (graphQLRequest))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        logger: quest_hook::libil2cpp::Gc<
            crate::OculusStudios::GraphQL::ClientInterface::IGraphQLClientEvents,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (logger))?;
        Ok(__cordl_object.into())
    }
    pub fn ReadFromStream(
        &mut self,
        liveQuery: quest_hook::libil2cpp::Gc<
            crate::OculusStudios::GraphQL::Client::GraphQLLiveQuery,
        >,
        responseStream: quest_hook::libil2cpp::Gc<
            crate::OculusStudios::GraphQL::Client::GraphQLResponseStream,
        >,
        MinimalMainThreadExecutor: quest_hook::libil2cpp::Gc<
            crate::OculusStudios::GraphQL::Client::MinimalMainThreadExecutor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::OculusStudios::GraphQL::Client::GraphQLLiveQuery,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::OculusStudios::GraphQL::Client::GraphQLResponseStream,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::OculusStudios::GraphQL::Client::MinimalMainThreadExecutor,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ReadFromStream")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadFromStream", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (liveQuery, responseStream, MinimalMainThreadExecutor),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        logger: quest_hook::libil2cpp::Gc<
            crate::OculusStudios::GraphQL::ClientInterface::IGraphQLClientEvents,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::OculusStudios::GraphQL::ClientInterface::IGraphQLClientEvents,
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
            cordl_method_info.invoke_unchecked(self, (logger))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_AccessToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_AccessToken")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_AccessToken", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Endpoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_Endpoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Endpoint", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_AccessToken(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_AccessToken")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_AccessToken", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Endpoint(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_Endpoint")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_Endpoint", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+Client+HttpPersistentTransport")]
impl quest_hook::libil2cpp::ObjectType
for crate::OculusStudios::GraphQL::Client::HttpPersistentTransport {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+HttpPersistentTransport")]
impl AsRef<crate::OculusStudios::GraphQL::Client::IGraphQLClientTransport>
for crate::OculusStudios::GraphQL::Client::HttpPersistentTransport {
    fn as_ref(&self) -> &crate::OculusStudios::GraphQL::Client::IGraphQLClientTransport {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+HttpPersistentTransport")]
impl AsMut<crate::OculusStudios::GraphQL::Client::IGraphQLClientTransport>
for crate::OculusStudios::GraphQL::Client::HttpPersistentTransport {
    fn as_mut(
        &mut self,
    ) -> &mut crate::OculusStudios::GraphQL::Client::IGraphQLClientTransport {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+HttpPersistentTransport")]
impl AsRef<crate::System::IDisposable>
for crate::OculusStudios::GraphQL::Client::HttpPersistentTransport {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusStudios+GraphQL+Client+HttpPersistentTransport")]
impl AsMut<crate::System::IDisposable>
for crate::OculusStudios::GraphQL::Client::HttpPersistentTransport {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
