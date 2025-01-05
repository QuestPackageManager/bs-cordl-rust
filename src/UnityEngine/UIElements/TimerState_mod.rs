#[cfg(feature = "UnityEngine+UIElements+TimerState")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TimerState {
    pub _start_k__BackingField: i64,
    pub _now_k__BackingField: i64,
}
#[cfg(feature = "UnityEngine+UIElements+TimerState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TimerState =>
    "UnityEngine.UIElements"."TimerState"
);
#[cfg(feature = "UnityEngine+UIElements+TimerState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::TimerState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TimerState")]
impl crate::UnityEngine::UIElements::TimerState {
    pub fn Equals_Gc0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_TimerState1(
        &mut self,
        other: crate::UnityEngine::UIElements::TimerState,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
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
    pub fn get_deltaTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_deltaTime",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_now(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_now",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_start(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_start",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_now(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_now",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_start(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_start",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+TimerState")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TimerState>>
for crate::UnityEngine::UIElements::TimerState {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TimerState> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+TimerState")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TimerState>>
for crate::UnityEngine::UIElements::TimerState {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::TimerState> {
        todo!()
    }
}
