#[cfg(feature = "GraphAPIClient")]
#[repr(C)]
#[derive(Debug)]
pub struct GraphAPIClient {
    __cordl_parent: crate::System::Object,
    pub _networkConfig: *mut INetworkConfig,
    pub _client: *mut crate::System::Net::Http::HttpClient,
}
#[cfg(feature = "GraphAPIClient")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for GraphAPIClient => ""."GraphAPIClient"
);
#[cfg(feature = "GraphAPIClient")]
impl std::ops::Deref for GraphAPIClient {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GraphAPIClient")]
impl std::ops::DerefMut for GraphAPIClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GraphAPIClient")]
impl GraphAPIClient {
    #[cfg(feature = "GraphAPIClient+PostOptions")]
    pub type PostOptions = crate::GlobalNamespace::GraphAPIClient_PostOptions;
    #[cfg(feature = "GraphAPIClient+_Post_d__4_2")]
    pub type _Post_d__4_2<
        TRequest: quest_hook::libil2cpp::Type,
        TResponse: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::GraphAPIClient__Post_d__4_2<TRequest, TResponse>;
    #[cfg(feature = "GraphAPIClient+_Post_d__5_1")]
    pub type _Post_d__5_1<TResponse: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::GraphAPIClient__Post_d__5_1<
        TResponse,
    >;
    pub fn PostLoggedOut<TRequest, TResponse>(
        &mut self,
        path: *mut crate::System::String,
        request: TRequest,
        postOptions: crate::GlobalNamespace::GraphAPIClient_PostOptions,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<TResponse>,
    >
    where
        TRequest: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResponse: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<TResponse> = __cordl_object
            .invoke("PostLoggedOut", (path, request, postOptions, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn Post_String_TRequest_GraphAPIClient_PostOptions_CancellationToken0<
        TRequest,
        TResponse,
    >(
        &mut self,
        path: *mut crate::System::String,
        accessToken: *mut crate::System::String,
        request: TRequest,
        postOptions: crate::GlobalNamespace::GraphAPIClient_PostOptions,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<TResponse>,
    >
    where
        TRequest: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResponse: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<TResponse> = __cordl_object
            .invoke(
                "Post",
                (path, accessToken, request, postOptions, cancellationToken),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Post_Uri_HttpContent_CancellationToken1<TResponse>(
        &mut self,
        uri: *mut crate::System::Uri,
        accessToken: *mut crate::System::String,
        httpContent: *mut crate::System::Net::Http::HttpContent,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<TResponse>,
    >
    where
        TResponse: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<TResponse> = __cordl_object
            .invoke("Post", (uri, accessToken, httpContent, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        networkConfig: *mut INetworkConfig,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (networkConfig))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        networkConfig: *mut INetworkConfig,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (networkConfig))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "GraphAPIClient")]
impl quest_hook::libil2cpp::ObjectType for GraphAPIClient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GraphAPIClient+PostOptions")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GraphAPIClient_PostOptions {
    pub MaxRetries: i32,
    pub MinWaitTimeForRetryMs: i32,
    pub WithExponentialBackoff: bool,
}
#[cfg(feature = "GraphAPIClient+PostOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GraphAPIClient_PostOptions =>
    ""."GraphAPIClient/PostOptions"
);
#[cfg(feature = "GraphAPIClient+PostOptions")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::GraphAPIClient_PostOptions {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "GraphAPIClient+PostOptions")]
impl crate::GlobalNamespace::GraphAPIClient_PostOptions {}
