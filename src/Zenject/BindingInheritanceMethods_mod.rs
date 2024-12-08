#[cfg(feature = "Zenject+BindingInheritanceMethods")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingInheritanceMethods {
    CopyDirectOnly = 2i32,
    CopyIntoAll = 1i32,
    MoveDirectOnly = 4i32,
    MoveIntoAll = 3i32,
    None = 0i32,
}
#[cfg(feature = "Zenject+BindingInheritanceMethods")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Zenject::BindingInheritanceMethods => "Zenject"
    ."BindingInheritanceMethods"
);
