#[cfg(feature = "System+Threading+ExecutionContextSwitcher")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ExecutionContextSwitcher {
    pub outerEC: crate::System::Threading::ExecutionContext_Reader,
    pub outerECBelongsToScope: bool,
    pub hecsw: *mut quest_hook::libil2cpp::Il2CppObject,
    pub thread: *mut crate::System::Threading::Thread,
}
#[cfg(feature = "System+Threading+ExecutionContextSwitcher")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::ExecutionContextSwitcher =>
    "System.Threading"."ExecutionContextSwitcher"
);
#[cfg(feature = "System+Threading+ExecutionContextSwitcher")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Threading::ExecutionContextSwitcher {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+ExecutionContextSwitcher")]
impl crate::System::Threading::ExecutionContextSwitcher {
    pub fn Undo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Undo",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn UndoNoThrow(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "UndoNoThrow",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
