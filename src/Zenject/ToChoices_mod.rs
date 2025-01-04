#[cfg(feature = "Zenject+ToChoices")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToChoices {
    #[default]
    Concrete = 1i32,
    _cordl_Self = 0i32,
}
#[cfg(feature = "Zenject+ToChoices")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ToChoices => "Zenject"."ToChoices"
);
