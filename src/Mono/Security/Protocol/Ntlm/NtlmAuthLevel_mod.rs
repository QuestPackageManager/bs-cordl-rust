#[cfg(feature = "Mono+Security+Protocol+Ntlm+NtlmAuthLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NtlmAuthLevel {
    #[default]
    LM_and_NTLM = 0i32,
    LM_and_NTLM_and_try_NTLMv2_Session = 1i32,
    NTLM_only = 2i32,
    NTLMv2_only = 3i32,
}
#[cfg(feature = "Mono+Security+Protocol+Ntlm+NtlmAuthLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Protocol::Ntlm::NtlmAuthLevel =>
    "Mono.Security.Protocol.Ntlm"."NtlmAuthLevel"
);
