#[cfg(feature = "System+ArgIterator")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ArgIterator {
    pub sig: crate::System::IntPtr,
    pub args: crate::System::IntPtr,
    pub next_arg: i32,
    pub num_args: i32,
}
#[cfg(feature = "System+ArgIterator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::ArgIterator => "System"."ArgIterator"
);
#[cfg(feature = "System+ArgIterator")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::ArgIterator {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+ArgIterator")]
impl crate::System::ArgIterator {
    pub fn Equals(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (o),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
