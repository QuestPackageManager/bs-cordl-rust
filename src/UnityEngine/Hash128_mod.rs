#[cfg(feature = "UnityEngine+Hash128")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Hash128 {
    pub u64_0: u64,
    pub u64_1: u64,
}
#[cfg(feature = "UnityEngine+Hash128")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Hash128 => "UnityEngine"."Hash128"
);
#[cfg(feature = "UnityEngine+Hash128")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Hash128 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Hash128")]
impl crate::UnityEngine::Hash128 {
    pub fn CompareTo_Hash128_0(
        &mut self,
        rhs: crate::UnityEngine::Hash128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (rhs),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Hash128_1(
        &mut self,
        obj: crate::UnityEngine::Hash128,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
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
        u64_0: u64,
        u64_1: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (u64_0, u64_1),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_isValid",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Hash128")]
impl AsRef<crate::System::IComparable> for crate::UnityEngine::Hash128 {
    fn as_ref(&self) -> &crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Hash128")]
impl AsMut<crate::System::IComparable> for crate::UnityEngine::Hash128 {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Hash128")]
impl AsRef<crate::System::IComparable_1<crate::UnityEngine::Hash128>>
for crate::UnityEngine::Hash128 {
    fn as_ref(&self) -> &crate::System::IComparable_1<crate::UnityEngine::Hash128> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Hash128")]
impl AsMut<crate::System::IComparable_1<crate::UnityEngine::Hash128>>
for crate::UnityEngine::Hash128 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<crate::UnityEngine::Hash128> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Hash128")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::Hash128>>
for crate::UnityEngine::Hash128 {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::UnityEngine::Hash128> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Hash128")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::Hash128>>
for crate::UnityEngine::Hash128 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::Hash128> {
        todo!()
    }
}
