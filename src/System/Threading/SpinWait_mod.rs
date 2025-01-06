#[cfg(feature = "System+Threading+SpinWait")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpinWait {
    pub _count: i32,
}
#[cfg(feature = "System+Threading+SpinWait")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::SpinWait =>
    "System.Threading"."SpinWait"
);
#[cfg(feature = "System+Threading+SpinWait")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Threading::SpinWait {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+SpinWait")]
impl crate::System::Threading::SpinWait {
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Reset",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SpinOnceCore(
        &mut self,
        sleep1Threshold: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SpinOnceCore",
            (sleep1Threshold),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SpinOnce_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SpinOnce",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SpinOnce_i32_1(
        &mut self,
        sleep1Threshold: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SpinOnce",
            (sleep1Threshold),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Count",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NextSpinWillYield(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_NextSpinWillYield",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
