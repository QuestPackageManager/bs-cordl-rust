#[cfg(feature = "System+IO+MonoIOError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MonoIOError {
    #[default]
    ERROR_ACCESS_DENIED = 5i32,
    ERROR_ALREADY_EXISTS = 183i32,
    ERROR_BROKEN_PIPE = 109i32,
    ERROR_CANNOT_MAKE = 82i32,
    ERROR_DIRECTORY = 267i32,
    ERROR_DIR_NOT_EMPTY = 145i32,
    ERROR_ENCRYPTION_FAILED = 6000i32,
    ERROR_FILENAME_EXCED_RANGE = 206i32,
    ERROR_FILE_EXISTS = 80i32,
    ERROR_FILE_NOT_FOUND = 2i32,
    ERROR_GEN_FAILURE = 31i32,
    ERROR_HANDLE_DISK_FULL = 39i32,
    ERROR_INVALID_DRIVE = 15i32,
    ERROR_INVALID_HANDLE = 6i32,
    ERROR_INVALID_NAME = 123i32,
    ERROR_INVALID_PARAMETER = 87i32,
    ERROR_LOCK_VIOLATION = 33i32,
    ERROR_NOT_READY = 21i32,
    ERROR_NOT_SAME_DEVICE = 17i32,
    ERROR_NOT_SUPPORTED = 50i32,
    ERROR_NO_MORE_FILES = 18i32,
    ERROR_PATH_NOT_FOUND = 3i32,
    ERROR_READ_FAULT = 30i32,
    ERROR_SHARING_VIOLATION = 32i32,
    ERROR_SUCCESS = 0i32,
    ERROR_TOO_MANY_OPEN_FILES = 4i32,
    ERROR_WRITE_FAULT = 29i32,
}
#[cfg(feature = "System+IO+MonoIOError")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::IO::MonoIOError {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.IO";
    const CLASS_NAME: &'static str = "MonoIOError";
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
unsafe impl quest_hook::libil2cpp::Argument for crate::System::IO::MonoIOError {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::IO::MonoIOError {
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
unsafe impl quest_hook::libil2cpp::Returned for crate::System::IO::MonoIOError {
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
unsafe impl quest_hook::libil2cpp::Return for crate::System::IO::MonoIOError {
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
