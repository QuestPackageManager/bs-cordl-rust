#[cfg(feature = "System+Threading+LockRecursionPolicy")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LockRecursionPolicy {
    #[default]
    NoRecursion = 0i32,
    SupportsRecursion = 1i32,
}
#[cfg(feature = "System+Threading+LockRecursionPolicy")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::LockRecursionPolicy =>
    "System.Threading"."LockRecursionPolicy"
);
