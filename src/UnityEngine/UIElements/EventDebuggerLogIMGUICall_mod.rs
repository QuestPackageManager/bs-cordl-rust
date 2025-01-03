#[cfg(feature = "UnityEngine+UIElements+EventDebuggerLogIMGUICall")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct EventDebuggerLogIMGUICall {}
#[cfg(feature = "UnityEngine+UIElements+EventDebuggerLogIMGUICall")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::EventDebuggerLogIMGUICall => "UnityEngine.UIElements"
    ."EventDebuggerLogIMGUICall"
);
#[cfg(feature = "UnityEngine+UIElements+EventDebuggerLogIMGUICall")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::EventDebuggerLogIMGUICall {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventDebuggerLogIMGUICall")]
impl crate::UnityEngine::UIElements::EventDebuggerLogIMGUICall {
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
#[cfg(feature = "UnityEngine+UIElements+EventDebuggerLogIMGUICall")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::UIElements::EventDebuggerLogIMGUICall {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+EventDebuggerLogIMGUICall")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::UIElements::EventDebuggerLogIMGUICall {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
