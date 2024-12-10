#[cfg(feature = "UnityEngine+ProBuilder+Triangle")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Triangle {
    pub m_A: i32,
    pub m_B: i32,
    pub m_C: i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+Triangle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Triangle =>
    "UnityEngine.ProBuilder"."Triangle"
);
#[cfg(feature = "UnityEngine+ProBuilder+Triangle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ProBuilder::Triangle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Triangle")]
impl crate::UnityEngine::ProBuilder::Triangle {
    pub fn ContainsEdge(
        &mut self,
        edge: crate::UnityEngine::ProBuilder::Edge,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ContainsEdge",
            (edge),
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
    pub fn Equals_Triangle0(
        &mut self,
        other: crate::UnityEngine::ProBuilder::Triangle,
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
    pub fn IsAdjacent(
        &mut self,
        other: crate::UnityEngine::ProBuilder::Triangle,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsAdjacent",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        a: i32,
        b: i32,
        c: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_a(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_a",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_b(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_b",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_c(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_c",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_indices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_indices", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Triangle")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::Triangle>>
for crate::UnityEngine::ProBuilder::Triangle {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::Triangle> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Triangle")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::Triangle>>
for crate::UnityEngine::ProBuilder::Triangle {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::Triangle> {
        todo!()
    }
}
