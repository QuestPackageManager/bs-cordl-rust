#[cfg(feature = "System+Variant")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Variant {
    padding: [u8; 24usize],
}
#[cfg(feature = "System+Variant")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Variant => "System"."Variant"
);
#[cfg(feature = "System+Variant")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Variant {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Variant")]
impl crate::System::Variant {
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
