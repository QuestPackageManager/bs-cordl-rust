#[cfg(feature = "System+Data+AcceptRejectRule")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AcceptRejectRule {
    #[default]
    Cascade = 1i32,
    None = 0i32,
}
#[cfg(feature = "System+Data+AcceptRejectRule")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::AcceptRejectRule => "System.Data"
    ."AcceptRejectRule"
);
