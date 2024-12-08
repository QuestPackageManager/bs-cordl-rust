#[cfg(feature = "System+Security+Principal+WindowsAccountType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowsAccountType {
    Anonymous = 3i32,
    Guest = 1i32,
    Normal = 0i32,
    System = 2i32,
}
#[cfg(feature = "System+Security+Principal+WindowsAccountType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Security::Principal::WindowsAccountType
    => "System.Security.Principal"."WindowsAccountType"
);
