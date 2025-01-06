#[cfg(feature = "ENet+ENetAddress")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ENetAddress {
    padding: quest_hook::libil2cpp::ValueTypePadding<18usize>,
}
#[cfg(feature = "ENet+ENetAddress")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::ENet::ENetAddress => "ENet"."ENetAddress"
);
#[cfg(feature = "ENet+ENetAddress")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::ENet::ENetAddress {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ENet+ENetAddress")]
impl crate::ENet::ENetAddress {}
