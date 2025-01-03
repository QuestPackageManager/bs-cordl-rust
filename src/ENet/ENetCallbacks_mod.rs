#[cfg(feature = "ENet+ENetCallbacks")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ENetCallbacks {
    pub malloc: quest_hook::libil2cpp::Gc<crate::ENet::AllocCallback>,
    pub free: quest_hook::libil2cpp::Gc<crate::ENet::FreeCallback>,
    pub noMemory: quest_hook::libil2cpp::Gc<crate::ENet::NoMemoryCallback>,
}
#[cfg(feature = "ENet+ENetCallbacks")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::ENet::ENetCallbacks => "ENet"."ENetCallbacks"
);
#[cfg(feature = "ENet+ENetCallbacks")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::ENet::ENetCallbacks {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "ENet+ENetCallbacks")]
impl crate::ENet::ENetCallbacks {}
