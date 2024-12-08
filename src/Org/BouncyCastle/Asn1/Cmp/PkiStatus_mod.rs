#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiStatus")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PkiStatus {
    Granted = 0i32,
    GrantedWithMods = 1i32,
    KeyUpdateWarning = 6i32,
    Rejection = 2i32,
    RevocationNotification = 5i32,
    RevocationWarning = 4i32,
    Waiting = 3i32,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiStatus")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cmp::PkiStatus =>
    "Org.BouncyCastle.Asn1.Cmp"."PkiStatus"
);
