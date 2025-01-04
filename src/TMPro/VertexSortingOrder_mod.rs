#[cfg(feature = "TMPro+VertexSortingOrder")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VertexSortingOrder {
    #[default]
    Normal = 0i32,
    Reverse = 1i32,
}
#[cfg(feature = "TMPro+VertexSortingOrder")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::VertexSortingOrder => "TMPro"
    ."VertexSortingOrder"
);
