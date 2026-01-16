#[cfg(feature = "cordl_class_OculusStudios+GraphQL+ClientInterface+GraphErrorCode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GraphErrorCode {
    #[default]
    GRAPH_QL__ADFINDER_EXCEPTION = 1675018i32,
    GRAPH_QL__BAD_ARGUMENTS = 1675012i32,
    GRAPH_QL__BAD_QUERY = 1675002i32,
    GRAPH_QL__BAD_QUERY_SYNTAX = 1675001i32,
    GRAPH_QL__BATCH_CIRCULAR_DEPENDENCY = 1675019i32,
    GRAPH_QL__BATCH_UNSATISFIED_REF_PARAM = 1675024i32,
    GRAPH_QL__BLOCKED_FOR_PROXY_USER_ACCESS = 1675043i32,
    GRAPH_QL__CANCELED_SUBSCRIPTION_PAYLOAD = 1675032i32,
    GRAPH_QL__DATA_LIMIT_EXCEEDED = 1675044i32,
    GRAPH_QL__ELASTIC_UNAVAILABILITY = 1675045i32,
    GRAPH_QL__FLATBUFFER_INVALID_JSON_STRING = 1675026i32,
    GRAPH_QL__INVALID_ACTOR_ID = 1675023i32,
    GRAPH_QL__INVALID_CURSOR = 1675011i32,
    GRAPH_QL__INVALID_GRAMMAR_ENTITY_AT_CALL = 1675016i32,
    GRAPH_QL__INVALID_GRAPH_SEARCH_QUERY = 1675017i32,
    GRAPH_QL__INVALID_ID = 1675015i32,
    GRAPH_QL__INVALID_LIVE_QUERY_CONFIG_ID = 1675035i32,
    GRAPH_QL__INVALID_MUATION_RESULT = 1675021i32,
    GRAPH_QL__INVALID_PHASE = 1675020i32,
    GRAPH_QL__INVALID_SHARED_PARAMS = 1675031i32,
    GRAPH_QL__INVALID_SLICE = 1675014i32,
    GRAPH_QL__LEGACY_QUERY_NOT_WHITELISTED = 1675036i32,
    GRAPH_QL__LEGACY_STRING_QUERY_NOT_ALLOWLISTED = 1675039i32,
    GRAPH_QL__MISSING_QUERY_PARAMETER = 1675003i32,
    GRAPH_QL__MODERN_STRING_QUERY_NOT_ALLOWLISTED = 1675040i32,
    GRAPH_QL__MUTATION_ERROR = 1675022i32,
    GRAPH_QL__NO_LOGGED_IN_USER = 1675006i32,
    GRAPH_QL__NULL_GRAPH_SEARCH_QUERY = 1675027i32,
    GRAPH_QL__OCULUS_STORE_MX_INVOICE_TEST = 1675037i32,
    GRAPH_QL__OVER_BATCH_MAX_LIMIT = 1675041i32,
    GRAPH_QL__PERMISSION_TOKEN_CHECK_DENIED = 1675042i32,
    GRAPH_QL__PERSISTED_QUERY_NOT_AVAILABLE = 1675007i32,
    GRAPH_QL__QUERY_ERROR = 1675030i32,
    GRAPH_QL__QUERY_ERROR_WITH_CAUSE = 1675038i32,
    GRAPH_QL__RATE_LIMIT_EXCEEDED = 1675004i32,
    GRAPH_QL__REJECTED_DUE_TO_PEAK_OPTIMIZATION = 1675033i32,
    GRAPH_QL__TOO_MANY_INPUTS = 1675028i32,
    GRAPH_QL__UNAUTHORIZED_QUERY = 1675034i32,
    GRAPH_QL__UNEXPECTED_TYPE = 1675029i32,
    GRAPH_QL__UNPERSISTABLE_QUERY = 1675013i32,
    GRAPH_QL__UNPERSISTABLE_QUERY_ROOT_CALL = 1675025i32,
    UNKNOWN = 0i32,
}
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+ClientInterface+GraphErrorCode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::OculusStudios::GraphQL::ClientInterface::GraphErrorCode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "OculusStudios.GraphQL.ClientInterface";
    const CLASS_NAME: &'static str = "GraphErrorCode";
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
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+ClientInterface+GraphErrorCode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::OculusStudios::GraphQL::ClientInterface::GraphErrorCode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+ClientInterface+GraphErrorCode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::OculusStudios::GraphQL::ClientInterface::GraphErrorCode {
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
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+ClientInterface+GraphErrorCode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::OculusStudios::GraphQL::ClientInterface::GraphErrorCode {
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
#[cfg(feature = "cordl_class_OculusStudios+GraphQL+ClientInterface+GraphErrorCode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::OculusStudios::GraphQL::ClientInterface::GraphErrorCode {
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
