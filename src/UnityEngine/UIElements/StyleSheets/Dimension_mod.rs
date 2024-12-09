#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Dimension")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Dimension {
    pub unit: crate::UnityEngine::UIElements::StyleSheets::Dimension_Unit,
    pub value: f32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Dimension")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::StyleSheets::Dimension
    => "UnityEngine.UIElements.StyleSheets"."Dimension"
);
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Dimension")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::StyleSheets::Dimension {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Dimension")]
impl crate::UnityEngine::UIElements::StyleSheets::Dimension {
    #[cfg(feature = "UnityEngine+UIElements+StyleSheets+Dimension+Unit")]
    pub type Unit = crate::UnityEngine::UIElements::StyleSheets::Dimension_Unit;
    pub fn Equals_Dimension0(
        &mut self,
        other: crate::UnityEngine::UIElements::StyleSheets::Dimension,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
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
    pub fn ToAngle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Angle> {
        let __cordl_ret: crate::UnityEngine::UIElements::Angle = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToAngle",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToLength(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToLength",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToTime(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TimeValue> {
        let __cordl_ret: crate::UnityEngine::UIElements::TimeValue = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToTime",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        value: f32,
        unit: crate::UnityEngine::UIElements::StyleSheets::Dimension_Unit,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value, unit),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Dimension+Unit")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dimension_Unit {
    Degree = 5i32,
    Gradian = 6i32,
    Millisecond = 4i32,
    Percent = 2i32,
    Pixel = 1i32,
    Radian = 7i32,
    Second = 3i32,
    Turn = 8i32,
    Unitless = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+StyleSheets+Dimension+Unit")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::StyleSheets::Dimension_Unit =>
    "UnityEngine.UIElements.StyleSheets"."Dimension/Unit"
);
