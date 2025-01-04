#[cfg(feature = "System+Net+FtpMethodFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FtpMethodFlags {
    #[default]
    DoesNotTakeParameter = 16i32,
    HasHttpCommand = 128i32,
    IsDownload = 1i32,
    IsUpload = 2i32,
    MayTakeParameter = 8i32,
    MustChangeWorkingDirectoryToPath = 256i32,
    None = 0i32,
    ParameterIsDirectory = 32i32,
    ShouldParseForResponseUri = 64i32,
    TakesParameter = 4i32,
}
#[cfg(feature = "System+Net+FtpMethodFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::FtpMethodFlags => "System.Net"
    ."FtpMethodFlags"
);
