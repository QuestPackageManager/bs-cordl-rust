#[cfg(feature = "UnityEngine+UIElements+Length")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Length {
    pub m_Value: f32,
    pub m_Unit: crate::UnityEngine::UIElements::Length_Unit,
}
#[cfg(feature = "UnityEngine+UIElements+Length")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Length =>
    "UnityEngine.UIElements"."Length"
);
#[cfg(feature = "UnityEngine+UIElements+Length")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Length {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Length")]
impl crate::UnityEngine::UIElements::Length {
    pub const k_MaxValue: f32 = 8388608f32;
    #[cfg(feature = "UnityEngine+UIElements+Length+Unit")]
    pub type Unit = crate::UnityEngine::UIElements::Length_Unit;
    pub fn Auto() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Auto", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
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
    pub fn Equals_Length0(
        &mut self,
        other: crate::UnityEngine::UIElements::Length,
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
    pub fn IsAuto(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsAuto",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNone(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsNone",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn None() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Length,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("None", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Percent(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Percent", (value))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn _ctor_LengthUnit1(
        &mut self,
        value: f32,
        unit: crate::UnityEngine::UIElements::LengthUnit,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value, unit),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Length_Unit2(
        &mut self,
        value: f32,
        unit: crate::UnityEngine::UIElements::Length_Unit,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value, unit),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_0(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_unit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::LengthUnit> {
        let __cordl_ret: crate::UnityEngine::UIElements::LengthUnit = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_unit",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_value(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::UIElements::Length,
        rhs: crate::UnityEngine::UIElements::Length,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::UnityEngine::UIElements::Length,
        rhs: crate::UnityEngine::UIElements::Length,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_value(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_value",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Length")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::Length>>
for crate::UnityEngine::UIElements::Length {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::Length> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+Length")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::Length>>
for crate::UnityEngine::UIElements::Length {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::UIElements::Length> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+Length+Unit")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Length_Unit {
    Auto = 2i32,
    None = 3i32,
    Percent = 1i32,
    Pixel = 0i32,
}
#[cfg(feature = "UnityEngine+UIElements+Length+Unit")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Length_Unit =>
    "UnityEngine.UIElements"."Length/Unit"
);
