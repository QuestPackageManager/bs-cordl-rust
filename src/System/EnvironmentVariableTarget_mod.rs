#[cfg(feature = "System+EnvironmentVariableTarget")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentVariableTarget {
    Machine = 2i32,
    Process = 0i32,
    User = 1i32,
}
#[cfg(feature = "System+EnvironmentVariableTarget")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::EnvironmentVariableTarget => "System"
    ."EnvironmentVariableTarget"
);
