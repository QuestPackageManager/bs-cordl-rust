#[cfg(feature = "Mono+Security+Protocol+Ntlm+NtlmFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NtlmFlags {
    Negotiate128 = 536870912i32,
    Negotiate56 = -2147483648i32,
    NegotiateAlwaysSign = 32768i32,
    NegotiateDomainSupplied = 4096i32,
    NegotiateNtlm = 512i32,
    NegotiateNtlm2Key = 524288i32,
    NegotiateOem = 2i32,
    NegotiateUnicode = 1i32,
    NegotiateWorkstationSupplied = 8192i32,
    RequestTarget = 4i32,
}
#[cfg(feature = "Mono+Security+Protocol+Ntlm+NtlmFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Protocol::Ntlm::NtlmFlags =>
    "Mono.Security.Protocol.Ntlm"."NtlmFlags"
);
