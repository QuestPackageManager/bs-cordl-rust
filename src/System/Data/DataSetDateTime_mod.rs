#[cfg(feature = "System+Data+DataSetDateTime")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataSetDateTime {
    Local = 1i32,
    Unspecified = 2i32,
    UnspecifiedLocal = 3i32,
    Utc = 4i32,
}
#[cfg(feature = "System+Data+DataSetDateTime")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::DataSetDateTime => "System.Data"
    ."DataSetDateTime"
);
