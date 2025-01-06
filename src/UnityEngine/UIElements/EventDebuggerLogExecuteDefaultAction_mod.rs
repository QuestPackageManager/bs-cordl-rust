#[cfg(feature = "UnityEngine+UIElements+EventDebuggerLogExecuteDefaultAction")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EventDebuggerLogExecuteDefaultAction {}
#[cfg(feature = "UnityEngine+UIElements+EventDebuggerLogExecuteDefaultAction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::EventDebuggerLogExecuteDefaultAction =>
    "UnityEngine.UIElements"."EventDebuggerLogExecuteDefaultAction"
);
#[cfg(feature = "UnityEngine+UIElements+EventDebuggerLogExecuteDefaultAction")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::EventDebuggerLogExecuteDefaultAction {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventDebuggerLogExecuteDefaultAction")]
impl crate::UnityEngine::UIElements::EventDebuggerLogExecuteDefaultAction {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        evt: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::EventBase>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (evt),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventDebuggerLogExecuteDefaultAction")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::UIElements::EventDebuggerLogExecuteDefaultAction {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventDebuggerLogExecuteDefaultAction")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::UIElements::EventDebuggerLogExecuteDefaultAction {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
