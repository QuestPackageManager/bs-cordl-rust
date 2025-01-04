#[cfg(feature = "System+Handles")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Handles {
    #[default]
    STD_ERROR = -12i32,
    STD_INPUT = -10i32,
    STD_OUTPUT = -11i32,
}
#[cfg(feature = "System+Handles")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Handles => "System"."Handles"
);
