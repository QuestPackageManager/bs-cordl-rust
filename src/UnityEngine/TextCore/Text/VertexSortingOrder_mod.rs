#[cfg(feature = "UnityEngine+TextCore+Text+VertexSortingOrder")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexSortingOrder {
    Normal = 0i32,
    Reverse = 1i32,
}
#[cfg(feature = "UnityEngine+TextCore+Text+VertexSortingOrder")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TextCore::Text::VertexSortingOrder
    => "UnityEngine.TextCore.Text"."VertexSortingOrder"
);
