#[cfg(feature = "System+Text+NormalizationCheck")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizationCheck {
    Maybe = 2i32,
    No = 1i32,
    Yes = 0i32,
}
#[cfg(feature = "System+Text+NormalizationCheck")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Text::NormalizationCheck =>
    "System.Text"."NormalizationCheck"
);
