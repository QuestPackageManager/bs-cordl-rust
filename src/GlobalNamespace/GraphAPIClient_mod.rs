#[cfg(feature = "GraphAPIClient")]
#[repr(C)]
#[derive(Debug)]
pub struct GraphAPIClient {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _networkConfig: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::INetworkConfig,
    >,
    pub _client: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpClient>,
}
#[cfg(feature = "GraphAPIClient")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GraphAPIClient => ""
    ."GraphAPIClient"
);
#[cfg(feature = "GraphAPIClient")]
impl std::ops::Deref for crate::GlobalNamespace::GraphAPIClient {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GraphAPIClient")]
impl std::ops::DerefMut for crate::GlobalNamespace::GraphAPIClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GraphAPIClient")]
impl crate::GlobalNamespace::GraphAPIClient {
    #[cfg(feature = "GraphAPIClient+PostOptions")]
    pub type PostOptions = crate::GlobalNamespace::GraphAPIClient_PostOptions;
    pub fn CalculateDelayMsBeforeRetry(
        numAttempts: i32,
        postOptions: crate::GlobalNamespace::GraphAPIClient_PostOptions,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateDelayMsBeforeRetry", (numAttempts, postOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        networkConfig: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkConfig>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (networkConfig))?;
        Ok(__cordl_object.into())
    }
    pub fn PostLoggedOut<TRequest, TResponse>(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        request: TRequest,
        postOptions: crate::GlobalNamespace::GraphAPIClient_PostOptions,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResponse>>
    where
        TRequest: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResponse: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResponse> = __cordl_object
            .invoke("PostLoggedOut", (path, request, postOptions, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn Post_Gc_CancellationToken1<TResponse>(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        accessToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        httpContent: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpContent>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResponse>>
    where
        TResponse: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResponse> = __cordl_object
            .invoke("Post", (uri, accessToken, httpContent, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn Post_TRequest_GraphAPIClient_PostOptions_CancellationToken0<
        TRequest,
        TResponse,
    >(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        accessToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        request: TRequest,
        postOptions: crate::GlobalNamespace::GraphAPIClient_PostOptions,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<TResponse>>
    where
        TRequest: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResponse: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<TResponse> = __cordl_object
            .invoke(
                "Post",
                (path, accessToken, request, postOptions, cancellationToken),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        networkConfig: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkConfig>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (networkConfig))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GraphAPIClient")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::GraphAPIClient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GraphAPIClient+PostOptions")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
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
