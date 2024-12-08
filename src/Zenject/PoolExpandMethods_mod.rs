#[cfg(feature = "Zenject+PoolExpandMethods")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoolExpandMethods {
    Disabled = 2i32,
    Double = 1i32,
    OneAtATime = 0i32,
}
#[cfg(feature = "Zenject+PoolExpandMethods")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Zenject::PoolExpandMethods => "Zenject"
    ."PoolExpandMethods"
);
