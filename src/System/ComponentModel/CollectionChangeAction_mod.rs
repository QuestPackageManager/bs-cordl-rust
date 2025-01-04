#[cfg(feature = "System+ComponentModel+CollectionChangeAction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CollectionChangeAction {
    #[default]
    Add = 1i32,
    Refresh = 3i32,
    Remove = 2i32,
}
#[cfg(feature = "System+ComponentModel+CollectionChangeAction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::CollectionChangeAction
    => "System.ComponentModel"."CollectionChangeAction"
);
