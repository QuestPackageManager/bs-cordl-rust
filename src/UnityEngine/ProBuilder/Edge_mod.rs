#[cfg(feature = "UnityEngine+ProBuilder+Edge")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Edge {
    pub a: i32,
    pub b: i32,
}
#[cfg(feature = "UnityEngine+ProBuilder+Edge")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Edge =>
    "UnityEngine.ProBuilder"."Edge"
);
#[cfg(feature = "UnityEngine+ProBuilder+Edge")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ProBuilder::Edge {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Edge")]
impl crate::UnityEngine::ProBuilder::Edge {
    pub fn Contains_Edge1(
        &mut self,
        other: crate::UnityEngine::ProBuilder::Edge,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Contains",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains_i32_0(&mut self, index: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Contains",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains_i32_Dictionary_2_2(
        &mut self,
        index: i32,
        lookup: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<i32, i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Contains",
            (index, lookup),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Edge0(
        &mut self,
        other: crate::UnityEngine::ProBuilder::Edge,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Edge_Dictionary_2_2(
        &mut self,
        other: crate::UnityEngine::ProBuilder::Edge,
        lookup: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<i32, i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other, lookup),
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
    pub fn IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsValid",
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
        a: i32,
        b: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Edge")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::Edge>>
for crate::UnityEngine::ProBuilder::Edge {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::Edge> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Edge")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::Edge>>
for crate::UnityEngine::ProBuilder::Edge {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::ProBuilder::Edge> {
        todo!()
    }
}
