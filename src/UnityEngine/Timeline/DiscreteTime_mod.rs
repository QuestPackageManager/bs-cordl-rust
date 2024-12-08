#[cfg(feature = "UnityEngine+Timeline+DiscreteTime")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DiscreteTime {
    pub m_DiscreteTime: i64,
}
#[cfg(feature = "UnityEngine+Timeline+DiscreteTime")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::DiscreteTime =>
    "UnityEngine.Timeline"."DiscreteTime"
);
#[cfg(feature = "UnityEngine+Timeline+DiscreteTime")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Timeline::DiscreteTime {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Timeline+DiscreteTime")]
impl crate::UnityEngine::Timeline::DiscreteTime {
    pub const k_Tick: f64 = 0.000000000001f64;
    pub fn _ctor_DiscreteTime0(
        &mut self,
        _cordl_time: crate::UnityEngine::Timeline::DiscreteTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_time),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i64_1(
        &mut self,
        _cordl_time: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_time),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_f64_2(
        &mut self,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_time),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_f32_3(
        &mut self,
        _cordl_time: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_time),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_4(
        &mut self,
        _cordl_time: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_time),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_f64_5(
        &mut self,
        frame: i32,
        fps: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (frame, fps),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompareTo(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
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
    pub fn GetTick(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetTick",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn OneTickBefore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::DiscreteTime> {
        let __cordl_ret: crate::UnityEngine::Timeline::DiscreteTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OneTickBefore",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn OneTickAfter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Timeline::DiscreteTime> {
        let __cordl_ret: crate::UnityEngine::Timeline::DiscreteTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "OneTickAfter",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_DiscreteTime0(
        &mut self,
        other: crate::UnityEngine::Timeline::DiscreteTime,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object1(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
}
