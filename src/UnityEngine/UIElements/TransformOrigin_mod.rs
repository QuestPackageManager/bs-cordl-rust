#[cfg(feature = "UnityEngine+UIElements+TransformOrigin")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TransformOrigin {
    pub m_X: crate::UnityEngine::UIElements::Length,
    pub m_Y: crate::UnityEngine::UIElements::Length,
    pub m_Z: f32,
}
#[cfg(feature = "UnityEngine+UIElements+TransformOrigin")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TransformOrigin =>
    "UnityEngine.UIElements"."TransformOrigin"
);
#[cfg(feature = "UnityEngine+UIElements+TransformOrigin")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::TransformOrigin {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TransformOrigin")]
impl crate::UnityEngine::UIElements::TransformOrigin {
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
    pub fn Equals_TransformOrigin0(
        &mut self,
        other: crate::UnityEngine::UIElements::TransformOrigin,
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
        crate::UnityEngine::UIElements::TransformOrigin,
    > {
        let __cordl_ret: crate::UnityEngine::UIElements::TransformOrigin = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Initial", ())?;
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
        x: crate::UnityEngine::UIElements::Length,
        y: crate::UnityEngine::UIElements::Length,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (x, y, z),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_x(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_x",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_y(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Length> {
        let __cordl_ret: crate::UnityEngine::UIElements::Length = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_y",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_z(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_z",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::UIElements::TransformOrigin,
        rhs: crate::UnityEngine::UIElements::TransformOrigin,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::UnityEngine::UIElements::TransformOrigin,
        rhs: crate::UnityEngine::UIElements::TransformOrigin,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_x(
        &mut self,
        value: crate::UnityEngine::UIElements::Length,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_x",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_y(
        &mut self,
        value: crate::UnityEngine::UIElements::Length,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_y",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+TransformOrigin")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::UIElements::TransformOrigin>>
for crate::UnityEngine::UIElements::TransformOrigin {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::UIElements::TransformOrigin> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+UIElements+TransformOrigin")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::UIElements::TransformOrigin>>
for crate::UnityEngine::UIElements::TransformOrigin {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        crate::UnityEngine::UIElements::TransformOrigin,
    > {
        todo!()
    }
}
