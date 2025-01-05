#[cfg(feature = "UnityEngine+UIElements+Rotate")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Rotate {
    pub m_Angle: crate::UnityEngine::UIElements::Angle,
    pub m_Axis: crate::UnityEngine::Vector3,
    pub m_IsNone: bool,
}
#[cfg(feature = "UnityEngine+UIElements+Rotate")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::Rotate =>
    "UnityEngine.UIElements"."Rotate"
);
#[cfg(feature = "UnityEngine+UIElements+Rotate")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::Rotate {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+Rotate")]
impl crate::UnityEngine::UIElements::Rotate {
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
    pub fn Equals_Rotate0(
        &mut self,
        other: crate::UnityEngine::UIElements::Rotate,
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
    pub fn Initial() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Rotate,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Rotate = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Initial", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn None() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::UIElements::Rotate,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::Rotate = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("None", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToQuaternion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToQuaternion",
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
        angle: crate::UnityEngine::UIElements::Angle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (angle),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_angle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Angle> {
        let __cordl_ret: crate::UnityEngine::UIElements::Angle = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_angle",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_axis(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_axis",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::UIElements::Rotate,
        rhs: crate::UnityEngine::UIElements::Rotate,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::UnityEngine::UIElements::Rotate,
        rhs: crate::UnityEngine::UIElements::Rotate,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_angle(
        &mut self,
        value: crate::UnityEngine::UIElements::Angle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_angle",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+Rotate")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::Rotate>>
for crate::UnityEngine::UIElements::Rotate {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::Rotate> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+Rotate")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::Rotate>>
for crate::UnityEngine::UIElements::Rotate {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::UIElements::Rotate> {
        todo!()
    }
}
