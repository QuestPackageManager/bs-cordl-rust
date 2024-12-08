#[cfg(feature = "System+Diagnostics+DebuggerBrowsableState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebuggerBrowsableState {
    Collapsed = 2i32,
    Never = 0i32,
    RootHidden = 3i32,
}
#[cfg(feature = "System+Diagnostics+DebuggerBrowsableState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Diagnostics::DebuggerBrowsableState =>
    "System.Diagnostics"."DebuggerBrowsableState"
);
