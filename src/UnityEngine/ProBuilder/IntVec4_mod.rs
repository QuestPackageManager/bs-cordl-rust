#[cfg(feature = "UnityEngine+ProBuilder+IntVec4")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct IntVec4 {
    pub value: crate::UnityEngine::Vector4,
}
#[cfg(feature = "UnityEngine+ProBuilder+IntVec4")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::IntVec4 =>
    "UnityEngine.ProBuilder"."IntVec4"
);
#[cfg(feature = "UnityEngine+ProBuilder+IntVec4")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ProBuilder::IntVec4 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+IntVec4")]
impl crate::UnityEngine::ProBuilder::IntVec4 {
    pub fn Equals_Il2CppObject2(
        &mut self,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (b),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_IntVec4_0(
        &mut self,
        p: crate::UnityEngine::ProBuilder::IntVec4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (p),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Vector4_1(
        &mut self,
        p: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (p),
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
        vector: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (vector),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_w(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_w",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_x(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_x",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_y(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
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
        a: crate::UnityEngine::ProBuilder::IntVec4,
        b: crate::UnityEngine::ProBuilder::IntVec4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_IntVec4_0(
        p: crate::UnityEngine::ProBuilder::IntVec4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (p))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_Vector4_1(
        p: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::IntVec4> {
        let __cordl_ret: crate::UnityEngine::ProBuilder::IntVec4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (p))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        a: crate::UnityEngine::ProBuilder::IntVec4,
        b: crate::UnityEngine::ProBuilder::IntVec4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn round(v: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("round", (v))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+IntVec4")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::IntVec4>>
for crate::UnityEngine::ProBuilder::IntVec4 {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::IntVec4> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+IntVec4")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::IntVec4>>
for crate::UnityEngine::ProBuilder::IntVec4 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::IntVec4> {
        todo!()
    }
}
