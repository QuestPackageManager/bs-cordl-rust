#[cfg(feature = "System+Text+NormalizationForm")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizationForm {
    FormC = 1i32,
    FormD = 2i32,
    FormKC = 5i32,
    FormKD = 6i32,
}
#[cfg(feature = "System+Text+NormalizationForm")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Text::NormalizationForm => "System.Text"
    ."NormalizationForm"
);
