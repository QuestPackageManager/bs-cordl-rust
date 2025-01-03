#[cfg(feature = "UnityEngine+EventInterests")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct EventInterests {
    pub _wantsMouseMove_k__BackingField: bool,
    pub _wantsMouseEnterLeaveWindow_k__BackingField: bool,
    pub _wantsLessLayoutEvents_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+EventInterests")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::EventInterests => "UnityEngine"
    ."EventInterests"
);
#[cfg(feature = "UnityEngine+EventInterests")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::EventInterests {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+EventInterests")]
impl crate::UnityEngine::EventInterests {
    pub fn WantsEvent(
        &mut self,
        _cordl_type: crate::UnityEngine::EventType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WantsEvent",
            (_cordl_type),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WantsLayoutPass(
        &mut self,
        _cordl_type: crate::UnityEngine::EventType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WantsLayoutPass",
            (_cordl_type),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wantsLessLayoutEvents(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wantsLessLayoutEvents",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wantsMouseEnterLeaveWindow(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wantsMouseEnterLeaveWindow",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_wantsMouseMove(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_wantsMouseMove",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_wantsMouseEnterLeaveWindow(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_wantsMouseEnterLeaveWindow",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_wantsMouseMove(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_wantsMouseMove",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
