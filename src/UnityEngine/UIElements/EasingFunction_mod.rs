#[cfg(feature = "UnityEngine+UIElements+EasingFunction")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EasingFunction {
    pub m_Mode: crate::UnityEngine::UIElements::EasingMode,
}
#[cfg(feature = "UnityEngine+UIElements+EasingFunction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::EasingFunction =>
    "UnityEngine.UIElements"."EasingFunction"
);
#[cfg(feature = "UnityEngine+UIElements+EasingFunction")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::EasingFunction {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+EasingFunction")]
impl crate::UnityEngine::UIElements::EasingFunction {
    pub fn Equals_EasingFunction0(
        &mut self,
        other: crate::UnityEngine::UIElements::EasingFunction,
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
    pub fn _ctor(
        &mut self,
        mode: crate::UnityEngine::UIElements::EasingMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (mode),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::EasingMode> {
        let __cordl_ret: crate::UnityEngine::UIElements::EasingMode = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_mode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::UIElements::EasingFunction,
        rhs: crate::UnityEngine::UIElements::EasingFunction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        easingMode: crate::UnityEngine::UIElements::EasingMode,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::EasingFunction> {
        let __cordl_ret: crate::UnityEngine::UIElements::EasingFunction = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (easingMode))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+EasingFunction")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::EasingFunction>>
for crate::UnityEngine::UIElements::EasingFunction {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::EasingFunction> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+EasingFunction")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::EasingFunction>>
for crate::UnityEngine::UIElements::EasingFunction {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::EasingFunction,
    > {
        todo!()
    }
}
