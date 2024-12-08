#[cfg(feature = "LiteNetLib+Utils+NtpLeapIndicator")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NtpLeapIndicator {
    AlarmCondition = 3i32,
    LastMinuteHas59Seconds = 2i32,
    LastMinuteHas61Seconds = 1i32,
    NoWarning = 0i32,
}
#[cfg(feature = "LiteNetLib+Utils+NtpLeapIndicator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::Utils::NtpLeapIndicator =>
    "LiteNetLib.Utils"."NtpLeapIndicator"
);
