#[cfg(feature = "System+Linq+Expressions+Interpreter+RuntimeLabel")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RuntimeLabel {
    pub Index: i32,
    pub StackDepth: i32,
    pub ContinuationStackDepth: i32,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+RuntimeLabel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::RuntimeLabel =>
    "System.Linq.Expressions.Interpreter"."RuntimeLabel"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+RuntimeLabel")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Linq::Expressions::Interpreter::RuntimeLabel {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+RuntimeLabel")]
impl crate::System::Linq::Expressions::Interpreter::RuntimeLabel {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        index: i32,
        continuationStackDepth: i32,
        stackDepth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (index, continuationStackDepth, stackDepth),
        )?;
        Ok(__cordl_ret)
    }
}