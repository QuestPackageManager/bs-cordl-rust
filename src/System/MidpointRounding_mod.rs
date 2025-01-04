#[cfg(feature = "System+MidpointRounding")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MidpointRounding {
    #[default]
    AwayFromZero = 1i32,
    ToEven = 0i32,
}
#[cfg(feature = "System+MidpointRounding")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::MidpointRounding => "System"
    ."MidpointRounding"
);
