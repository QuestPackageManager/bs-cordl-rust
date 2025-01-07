#[cfg(feature = "System+Security+AccessControl+FileSystemRights")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FileSystemRights {
    #[default]
    AppendData = 4i32,
    ChangePermissions = 262144i32,
    CreateFiles = 2i32,
    Delete = 65536i32,
    DeleteSubdirectoriesAndFiles = 64i32,
    ExecuteFile = 32i32,
    FullControl = 2032127i32,
    ListDirectory = 1i32,
    Modify = 197055i32,
    Read = 131209i32,
    ReadAndExecute = 131241i32,
    ReadAttributes = 128i32,
    ReadExtendedAttributes = 8i32,
    ReadPermissions = 131072i32,
    Synchronize = 1048576i32,
    TakeOwnership = 524288i32,
    Write = 278i32,
    WriteAttributes = 256i32,
    WriteExtendedAttributes = 16i32,
}
#[cfg(feature = "System+Security+AccessControl+FileSystemRights")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Security::AccessControl::FileSystemRights {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Security.AccessControl";
    const CLASS_NAME: &'static str = "FileSystemRights";
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
#[cfg(feature = "System+Security+AccessControl+FileSystemRights")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Security::AccessControl::FileSystemRights {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Security+AccessControl+FileSystemRights")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Security::AccessControl::FileSystemRights {
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
#[cfg(feature = "System+Security+AccessControl+FileSystemRights")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Security::AccessControl::FileSystemRights {
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
#[cfg(feature = "System+Security+AccessControl+FileSystemRights")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Security::AccessControl::FileSystemRights {
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
