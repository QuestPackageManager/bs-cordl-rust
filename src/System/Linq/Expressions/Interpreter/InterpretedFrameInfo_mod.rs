#[cfg(feature = "System+Linq+Expressions+Interpreter+InterpretedFrameInfo")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InterpretedFrameInfo {
    pub _methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _debugInfo: quest_hook::libil2cpp::Gc<
        crate::System::Linq::Expressions::Interpreter::DebugInfo,
    >,
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InterpretedFrameInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Linq::Expressions::Interpreter::InterpretedFrameInfo =>
    "System.Linq.Expressions.Interpreter"."InterpretedFrameInfo"
);
#[cfg(feature = "System+Linq+Expressions+Interpreter+InterpretedFrameInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Linq::Expressions::Interpreter::InterpretedFrameInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Linq+Expressions+Interpreter+InterpretedFrameInfo")]
impl crate::System::Linq::Expressions::Interpreter::InterpretedFrameInfo {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Interpreter::DebugInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (methodName, info),
        )?;
        Ok(__cordl_ret.into())
    }
}
