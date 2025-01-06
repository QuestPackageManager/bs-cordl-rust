#[cfg(feature = "UnityEngine+UIElements+Angle")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Angle {
    pub m_Value: f32,
    pub m_Unit: crate::UnityEngine::UIElements::Angle_Unit,
}
#[cfg(feature = "UnityEngine+UIElements+Angle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Angle =>
    "UnityEngine.UIElements"."Angle"
);
#[cfg(feature = "UnityEngine+UIElements+Angle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Angle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Angle")]
impl crate::UnityEngine::UIElements::Angle {
    #[cfg(feature = "UnityEngine+UIElements+Angle+Unit")]
    pub type Unit = crate::UnityEngine::UIElements::Angle_Unit;
    pub fn Equals_Angle0(
        &mut self,
        other: crate::UnityEngine::UIElements::Angle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
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
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn None() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Angle,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Angle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("None", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToDegrees(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToDegrees",
            (),
        )?;
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
    pub fn _ctor_AngleUnit0(
        &mut self,
        value: f32,
        unit: crate::UnityEngine::UIElements::AngleUnit,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value, unit),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Angle_Unit1(
        &mut self,
        value: f32,
        unit: crate::UnityEngine::UIElements::Angle_Unit,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value, unit),
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
        lhs: crate::UnityEngine::UIElements::Angle,
        rhs: crate::UnityEngine::UIElements::Angle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Angle> {
        let __cordl_ret: crate::UnityEngine::UIElements::Angle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Angle")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::Angle>>
for crate::UnityEngine::UIElements::Angle {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::Angle> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+Angle")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::Angle>>
for crate::UnityEngine::UIElements::Angle {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::UIElements::Angle> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+Angle+Unit")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Angle_Unit {
    #[default]
    Degree = 0i32,
    Gradian = 1i32,
    None = 4i32,
    Radian = 2i32,
    Turn = 3i32,
}
#[cfg(feature = "UnityEngine+UIElements+Angle+Unit")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Angle_Unit =>
    "UnityEngine.UIElements"."Angle/Unit"
);
