#[cfg(feature = "System+ExceptionResource")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExceptionResource {
    #[default]
    Arg_ArrayPlusOffTooSmall = 5i32,
    Arg_NonZeroLowerBound = 6i32,
    Arg_RankMultiDimNotSupported = 7i32,
    Arg_RegKeyDelHive = 8i32,
    Arg_RegKeyStrLenBug = 9i32,
    Arg_RegSetMismatchedKind = 11i32,
    Arg_RegSetStrArrNull = 10i32,
    Arg_RegSubKeyAbsent = 12i32,
    Arg_RegSubKeyValueAbsent = 13i32,
    ArgumentOutOfRange_BiggerThanCollection = 34i32,
    ArgumentOutOfRange_Count = 25i32,
    ArgumentOutOfRange_Index = 22i32,
    ArgumentOutOfRange_InvalidThreshold = 26i32,
    ArgumentOutOfRange_ListInsert = 27i32,
    ArgumentOutOfRange_NeedNonNegNum = 4i32,
    ArgumentOutOfRange_SmallCapacity = 21i32,
    Argument_AddingDuplicate = 14i32,
    Argument_ImplementIComparable = 0i32,
    Argument_InvalidArgumentForComparison = 2i32,
    Argument_InvalidArrayType = 18i32,
    Argument_InvalidOffLen = 23i32,
    Argument_InvalidRegistryKeyPermissionCheck = 3i32,
    Argument_InvalidRegistryOptionsCheck = 44i32,
    Argument_InvalidRegistryViewCheck = 45i32,
    Argument_InvalidType = 1i32,
    Argument_ItemNotExist = 24i32,
    InvalidOperation_CannotRemoveFromStackOrQueue = 29i32,
    InvalidOperation_EmptyQueue = 30i32,
    InvalidOperation_EmptyStack = 33i32,
    InvalidOperation_EnumEnded = 36i32,
    InvalidOperation_EnumFailedVersion = 32i32,
    InvalidOperation_EnumNotStarted = 35i32,
    InvalidOperation_EnumOpCantHappen = 31i32,
    InvalidOperation_NoValue = 38i32,
    InvalidOperation_NullArray = 50i32,
    InvalidOperation_RegRemoveSubKey = 39i32,
    NotSupported_InComparableType = 43i32,
    NotSupported_KeyCollectionSet = 19i32,
    NotSupported_ReadOnlyCollection = 28i32,
    NotSupported_SortedListNestedWrite = 37i32,
    NotSupported_StringComparison = 49i32,
    NotSupported_ValueCollectionSet = 20i32,
    ObjectDisposed_RegKeyClosed = 42i32,
    Security_RegistryPermission = 40i32,
    Serialization_InvalidOnDeser = 15i32,
    Serialization_MissingKeys = 16i32,
    Serialization_NullKey = 17i32,
    TaskCompletionSourceT_TrySetException_NoExceptions = 48i32,
    TaskCompletionSourceT_TrySetException_NullException = 47i32,
    TaskT_TransitionToFinal_AlreadyCompleted = 46i32,
    UnauthorizedAccess_RegistryNoWrite = 41i32,
}
#[cfg(feature = "System+ExceptionResource")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::ExceptionResource {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "ExceptionResource";
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
#[cfg(feature = "System+ExceptionResource")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::ExceptionResource {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+ExceptionResource")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::ExceptionResource {
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
#[cfg(feature = "System+ExceptionResource")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::ExceptionResource {
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
#[cfg(feature = "System+ExceptionResource")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::ExceptionResource {
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
