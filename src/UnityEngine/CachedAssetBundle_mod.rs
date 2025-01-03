#[cfg(feature = "UnityEngine+CachedAssetBundle")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct CachedAssetBundle {
    pub m_Name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Hash: crate::UnityEngine::Hash128,
}
#[cfg(feature = "UnityEngine+CachedAssetBundle")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::CachedAssetBundle => "UnityEngine"
    ."CachedAssetBundle"
);
#[cfg(feature = "UnityEngine+CachedAssetBundle")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::CachedAssetBundle {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+CachedAssetBundle")]
impl crate::UnityEngine::CachedAssetBundle {
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        hash: crate::UnityEngine::Hash128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (name, hash),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hash(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Hash128> {
        let __cordl_ret: crate::UnityEngine::Hash128 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_hash",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_name", ())?;
        Ok(__cordl_ret.into())
    }
}
