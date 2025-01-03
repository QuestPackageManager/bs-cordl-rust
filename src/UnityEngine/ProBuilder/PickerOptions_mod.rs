#[cfg(feature = "UnityEngine+ProBuilder+PickerOptions")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PickerOptions {
    pub _depthTest_k__BackingField: bool,
    pub _rectSelectMode_k__BackingField: crate::UnityEngine::ProBuilder::RectSelectMode,
}
#[cfg(feature = "UnityEngine+ProBuilder+PickerOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::PickerOptions =>
    "UnityEngine.ProBuilder"."PickerOptions"
);
#[cfg(feature = "UnityEngine+ProBuilder+PickerOptions")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ProBuilder::PickerOptions {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+PickerOptions")]
impl crate::UnityEngine::ProBuilder::PickerOptions {
    pub fn Equals_Il2CppObject0(
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
    pub fn Equals_PickerOptions1(
        &mut self,
        other: crate::UnityEngine::ProBuilder::PickerOptions,
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
    pub fn get_Default() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ProBuilder::PickerOptions,
    > {
        let __cordl_ret: crate::UnityEngine::ProBuilder::PickerOptions = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Default", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_depthTest(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_depthTest",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rectSelectMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::RectSelectMode> {
        let __cordl_ret: crate::UnityEngine::ProBuilder::RectSelectMode = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_rectSelectMode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        a: crate::UnityEngine::ProBuilder::PickerOptions,
        b: crate::UnityEngine::ProBuilder::PickerOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        a: crate::UnityEngine::ProBuilder::PickerOptions,
        b: crate::UnityEngine::ProBuilder::PickerOptions,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_depthTest(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_depthTest",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rectSelectMode(
        &mut self,
        value: crate::UnityEngine::ProBuilder::RectSelectMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_rectSelectMode",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
