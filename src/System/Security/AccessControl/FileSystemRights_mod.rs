#[cfg(feature = "System+Security+AccessControl+FileSystemRights")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileSystemRights {
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
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Security::AccessControl::FileSystemRights =>
    "System.Security.AccessControl"."FileSystemRights"
);
