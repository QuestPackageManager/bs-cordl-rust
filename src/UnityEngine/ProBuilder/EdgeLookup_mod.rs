#[cfg(feature = "UnityEngine+ProBuilder+EdgeLookup")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct EdgeLookup {
    pub m_Local: crate::UnityEngine::ProBuilder::Edge,
    pub m_Common: crate::UnityEngine::ProBuilder::Edge,
}
#[cfg(feature = "UnityEngine+ProBuilder+EdgeLookup")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::EdgeLookup =>
    "UnityEngine.ProBuilder"."EdgeLookup"
);
#[cfg(feature = "UnityEngine+ProBuilder+EdgeLookup")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ProBuilder::EdgeLookup {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+EdgeLookup")]
impl crate::UnityEngine::ProBuilder::EdgeLookup {
    pub fn Equals_EdgeLookup0(
        &mut self,
        other: crate::UnityEngine::ProBuilder::EdgeLookup,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Gc1(
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
    pub fn GetEdgeLookup(
        edges: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Edge>,
        lookup: quest_hook::libil2cpp::Gc<i32, i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::EdgeLookup>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::EdgeLookup,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEdgeLookup", (edges, lookup))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEdgeLookupHashSet(
        edges: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Edge>,
        lookup: quest_hook::libil2cpp::Gc<i32, i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::EdgeLookup>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::EdgeLookup,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetEdgeLookupHashSet", (edges, lookup))?;
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
    pub fn _ctor_Edge_Edge0(
        &mut self,
        common: crate::UnityEngine::ProBuilder::Edge,
        local: crate::UnityEngine::ProBuilder::Edge,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (common, local),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_i32_i32_1(
        &mut self,
        cx: i32,
        cy: i32,
        x: i32,
        y: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (cx, cy, x, y),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_common(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::Edge> {
        let __cordl_ret: crate::UnityEngine::ProBuilder::Edge = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_common",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_local(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::Edge> {
        let __cordl_ret: crate::UnityEngine::ProBuilder::Edge = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_local",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        a: crate::UnityEngine::ProBuilder::EdgeLookup,
        b: crate::UnityEngine::ProBuilder::EdgeLookup,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        a: crate::UnityEngine::ProBuilder::EdgeLookup,
        b: crate::UnityEngine::ProBuilder::EdgeLookup,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_common(
        &mut self,
        value: crate::UnityEngine::ProBuilder::Edge,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_common",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_local(
        &mut self,
        value: crate::UnityEngine::ProBuilder::Edge,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_local",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+EdgeLookup")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::EdgeLookup>>
for crate::UnityEngine::ProBuilder::EdgeLookup {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::EdgeLookup> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+EdgeLookup")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::EdgeLookup>>
for crate::UnityEngine::ProBuilder::EdgeLookup {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::EdgeLookup> {
        todo!()
    }
}
