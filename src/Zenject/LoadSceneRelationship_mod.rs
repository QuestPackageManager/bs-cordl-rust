#[cfg(feature = "Zenject+LoadSceneRelationship")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadSceneRelationship {
    Child = 1i32,
    None = 0i32,
    Sibling = 2i32,
}
#[cfg(feature = "Zenject+LoadSceneRelationship")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Zenject::LoadSceneRelationship => "Zenject"
    ."LoadSceneRelationship"
);
