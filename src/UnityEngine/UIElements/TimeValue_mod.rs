#[cfg(feature = "UnityEngine+UIElements+TimeValue")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TimeValue {
    pub m_Value: f32,
    pub m_Unit: crate::UnityEngine::UIElements::TimeUnit,
}
#[cfg(feature = "UnityEngine+UIElements+TimeValue")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TimeValue =>
    "UnityEngine.UIElements"."TimeValue"
);
#[cfg(feature = "UnityEngine+UIElements+TimeValue")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::TimeValue {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TimeValue")]
impl crate::UnityEngine::UIElements::TimeValue {
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
    pub fn Equals_TimeValue0(
        &mut self,
        other: crate::UnityEngine::UIElements::TimeValue,
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
    pub fn _ctor_TimeUnit1(
        &mut self,
        value: f32,
        unit: crate::UnityEngine::UIElements::TimeUnit,
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
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TimeUnit> {
        let __cordl_ret: crate::UnityEngine::UIElements::TimeUnit = quest_hook::libil2cpp::ValueTypeExt::invoke(
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
        lhs: crate::UnityEngine::UIElements::TimeValue,
        rhs: crate::UnityEngine::UIElements::TimeValue,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        value: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::TimeValue> {
        let __cordl_ret: crate::UnityEngine::UIElements::TimeValue = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::UnityEngine::UIElements::TimeValue,
        rhs: crate::UnityEngine::UIElements::TimeValue,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+TimeValue")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::TimeValue>>
for crate::UnityEngine::UIElements::TimeValue {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::TimeValue> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+TimeValue")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::TimeValue>>
for crate::UnityEngine::UIElements::TimeValue {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::UIElements::TimeValue> {
        todo!()
    }
}
