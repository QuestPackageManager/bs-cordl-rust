#[cfg(feature = "BGLib+Polyglot+GoogleDriveDownloadFormat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoogleDriveDownloadFormat {
    CSV = 0i32,
    TSV = 1i32,
}
#[cfg(feature = "BGLib+Polyglot+GoogleDriveDownloadFormat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BGLib::Polyglot::GoogleDriveDownloadFormat =>
    "BGLib.Polyglot"."GoogleDriveDownloadFormat"
);
