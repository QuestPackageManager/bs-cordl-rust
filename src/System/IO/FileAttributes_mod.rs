#[cfg(feature = "System+IO+FileAttributes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FileAttributes {
    #[default]
    Archive = 32i32,
    Compressed = 2048i32,
    Device = 64i32,
    Directory = 16i32,
    Encrypted = 16384i32,
    Hidden = 2i32,
    IntegrityStream = 32768i32,
    NoScrubData = 131072i32,
    Normal = 128i32,
    NotContentIndexed = 8192i32,
    Offline = 4096i32,
    ReadOnly = 1i32,
    ReparsePoint = 1024i32,
    SparseFile = 512i32,
    System = 4i32,
    Temporary = 256i32,
}
#[cfg(feature = "System+IO+FileAttributes")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::IO::FileAttributes {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.IO";
    const CLASS_NAME: &'static str = "FileAttributes";
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
#[cfg(feature = "System+IO+FileAttributes")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::IO::FileAttributes {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+IO+FileAttributes")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::IO::FileAttributes {
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
#[cfg(feature = "System+IO+FileAttributes")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::IO::FileAttributes {
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
#[cfg(feature = "System+IO+FileAttributes")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::IO::FileAttributes {
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
