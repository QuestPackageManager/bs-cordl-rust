#[cfg(feature = "System+ComponentModel+ListChangedType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListChangedType {
    ItemAdded = 1i32,
    ItemChanged = 4i32,
    ItemDeleted = 2i32,
    ItemMoved = 3i32,
    PropertyDescriptorAdded = 5i32,
    PropertyDescriptorChanged = 7i32,
    PropertyDescriptorDeleted = 6i32,
    Reset = 0i32,
}
#[cfg(feature = "System+ComponentModel+ListChangedType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::ListChangedType =>
    "System.ComponentModel"."ListChangedType"
);
