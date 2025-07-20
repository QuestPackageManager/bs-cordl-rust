#[cfg(feature = "GraphAPIClient")]
#[repr(C)]
#[derive(Debug)]
pub struct GraphAPIClient {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _networkConfig: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::INetworkConfig,
    >,
    pub _client: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpClient>,
}
#[cfg(feature = "GraphAPIClient")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::GraphAPIClient {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GraphAPIClient";
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
#[cfg(feature = "GraphAPIClient")]
impl std::ops::Deref for crate::GlobalNamespace::GraphAPIClient {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (i32, crate::GlobalNamespace::GraphAPIClient_PostOptions),
                        i32,
                        2usize,
                    >("CalculateDelayMsBeforeRetry")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CalculateDelayMsBeforeRetry", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (numAttempts, postOptions))?
        };
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
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResponse>>,
    >
    where
        TRequest: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResponse: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            TRequest,
                            crate::GlobalNamespace::GraphAPIClient_PostOptions,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TResponse>,
                        >,
                        4usize,
                    >("PostLoggedOut")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "PostLoggedOut", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResponse>,
        > = unsafe {
            method
                .invoke_unchecked(self, (path, request, postOptions, cancellationToken))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Post_Il2CppString_TRequest_GraphAPIClient_PostOptions_CancellationToken0<
        TRequest,
        TResponse,
    >(
        &mut self,
        path: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        accessToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        request: TRequest,
        postOptions: crate::GlobalNamespace::GraphAPIClient_PostOptions,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResponse>>,
    >
    where
        TRequest: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResponse: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            TRequest,
                            crate::GlobalNamespace::GraphAPIClient_PostOptions,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TResponse>,
                        >,
                        5usize,
                    >("Post")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Post", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResponse>,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (path, accessToken, request, postOptions, cancellationToken),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Post_Uri_HttpContent_CancellationToken1<TResponse>(
        &mut self,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        accessToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        httpContent: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpContent>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<TResponse>>,
    >
    where
        TResponse: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Uri>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Net::Http::HttpContent,
                            >,
                            crate::System::Threading::CancellationToken,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Threading::Tasks::Task_1<TResponse>,
                        >,
                        4usize,
                    >("Post")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Post", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<TResponse>,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (uri, accessToken, httpContent, cancellationToken),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        networkConfig: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::INetworkConfig>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::INetworkConfig,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (networkConfig))?
        };
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GraphAPIClient_PostOptions {
    pub MaxRetries: i32,
    pub MinWaitTimeForRetryMs: i32,
    pub WithExponentialBackoff: bool,
}
#[cfg(feature = "GraphAPIClient+PostOptions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GraphAPIClient_PostOptions {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GraphAPIClient/PostOptions";
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
#[cfg(feature = "GraphAPIClient+PostOptions")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::GraphAPIClient_PostOptions {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "GraphAPIClient+PostOptions")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::GraphAPIClient_PostOptions {
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
#[cfg(feature = "GraphAPIClient+PostOptions")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::GraphAPIClient_PostOptions {
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
#[cfg(feature = "GraphAPIClient+PostOptions")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::GraphAPIClient_PostOptions {
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
