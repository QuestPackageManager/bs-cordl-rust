#[cfg(feature = "System+ExceptionArgument")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExceptionArgument {
    #[default]
    action = 33i32,
    array = 3i32,
    arrayIndex = 17i32,
    byteOffset = 43i32,
    capacity = 12i32,
    collection = 6i32,
    comparable = 30i32,
    comparer = 29i32,
    comparison = 34i32,
    comparisonType = 47i32,
    converter = 9i32,
    count = 16i32,
    culture = 41i32,
    destination = 42i32,
    dictionary = 1i32,
    dictionaryCreationThreshold = 2i32,
    endIndex = 37i32,
    endSegment = 36i32,
    exception = 32i32,
    exceptions = 31i32,
    format = 50i32,
    index = 13i32,
    info = 4i32,
    input = 49i32,
    item = 20i32,
    key = 5i32,
    length = 28i32,
    list = 7i32,
    _cordl_match = 8i32,
    minimumBufferSize = 44i32,
    mode = 19i32,
    name = 18i32,
    obj = 0i32,
    offset = 45i32,
    options = 21i32,
    ownedMemory = 26i32,
    pointer = 25i32,
    queue = 10i32,
    s = 48i32,
    source = 39i32,
    sourceBytesToCopy = 23i32,
    stack = 11i32,
    start = 24i32,
    startIndex = 14i32,
    startSegment = 35i32,
    state = 40i32,
    task = 38i32,
    text = 27i32,
    value = 15i32,
    values = 46i32,
    view = 22i32,
}
#[cfg(feature = "System+ExceptionArgument")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::ExceptionArgument {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "ExceptionArgument";
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
#[cfg(feature = "System+ExceptionArgument")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::ExceptionArgument {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+ExceptionArgument")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::ExceptionArgument {
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
#[cfg(feature = "System+ExceptionArgument")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::ExceptionArgument {
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
#[cfg(feature = "System+ExceptionArgument")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::ExceptionArgument {
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
