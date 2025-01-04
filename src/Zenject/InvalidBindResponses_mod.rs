#[cfg(feature = "Zenject+InvalidBindResponses")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InvalidBindResponses {
    #[default]
    _cordl_Assert = 0i32,
    Skip = 1i32,
}
#[cfg(feature = "Zenject+InvalidBindResponses")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Zenject::InvalidBindResponses => "Zenject"
    ."InvalidBindResponses"
);
