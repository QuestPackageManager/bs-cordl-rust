#[cfg(feature = "Internal+Cryptography+Pal+GeneralNameType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GeneralNameType {
    #[default]
    DirectoryName = 4i32,
    DnsName = 2i32,
    EdiPartyName = 5i32,
    Email = 1i32,
    IPAddress = 7i32,
    OtherName = 0i32,
    RegisteredId = 8i32,
    UniformResourceIdentifier = 6i32,
    X400Address = 3i32,
}
#[cfg(feature = "Internal+Cryptography+Pal+GeneralNameType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Internal::Cryptography::Pal::GeneralNameType =>
    "Internal.Cryptography.Pal"."GeneralNameType"
);
