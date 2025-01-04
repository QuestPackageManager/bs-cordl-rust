#[cfg(feature = "System+Net+FtpOperation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FtpOperation {
    #[default]
    AppendFile = 5i32,
    DeleteFile = 6i32,
    DownloadFile = 0i32,
    GetDateTimestamp = 7i32,
    GetFileSize = 8i32,
    ListDirectory = 1i32,
    ListDirectoryDetails = 2i32,
    MakeDirectory = 10i32,
    Other = 13i32,
    PrintWorkingDirectory = 12i32,
    RemoveDirectory = 11i32,
    Rename = 9i32,
    UploadFile = 3i32,
    UploadFileUnique = 4i32,
}
#[cfg(feature = "System+Net+FtpOperation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::FtpOperation => "System.Net"
    ."FtpOperation"
);
