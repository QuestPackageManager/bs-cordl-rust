#[cfg(feature = "UnityEngine+ProBuilder+IntVec2")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct IntVec2 {
    pub value: crate::UnityEngine::Vector2,
}
#[cfg(feature = "UnityEngine+ProBuilder+IntVec2")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::IntVec2 =>
    "UnityEngine.ProBuilder"."IntVec2"
);
#[cfg(feature = "UnityEngine+ProBuilder+IntVec2")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ProBuilder::IntVec2 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+IntVec2")]
impl crate::UnityEngine::ProBuilder::IntVec2 {
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
    pub fn Equals_IntVec2_0(
        &mut self,
        p: crate::UnityEngine::ProBuilder::IntVec2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (p),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Vector2_1(
        &mut self,
        p: crate::UnityEngine::Vector2,
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
        vector: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (vector),
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
}
#[cfg(feature = "UnityEngine+ProBuilder+IntVec2")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::IntVec2>>
for crate::UnityEngine::ProBuilder::IntVec2 {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::IntVec2> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+IntVec2")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::IntVec2>>
for crate::UnityEngine::ProBuilder::IntVec2 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::IntVec2> {
        todo!()
    }
}
