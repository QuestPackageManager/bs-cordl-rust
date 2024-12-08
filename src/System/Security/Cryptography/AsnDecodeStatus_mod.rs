#[cfg(feature = "System+Security+Cryptography+AsnDecodeStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsnDecodeStatus {
    BadAsn = 1i32,
    BadLength = 3i32,
    BadTag = 2i32,
    InformationNotAvailable = 4i32,
    NotDecoded = -1i32,
    _cordl_Ok = 0i32,
}
#[cfg(feature = "System+Security+Cryptography+AsnDecodeStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Cryptography::AsnDecodeStatus
    => "System.Security.Cryptography"."AsnDecodeStatus"
);
