#[cfg(feature = "UnityEngine+ResourceManagement+Util+SerializedType")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SerializedType {
    pub m_AssemblyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_ClassName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_CachedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub _ValueChanged_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+SerializedType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::Util::SerializedType =>
    "UnityEngine.ResourceManagement.Util"."SerializedType"
);
#[cfg(feature = "UnityEngine+ResourceManagement+Util+SerializedType")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ResourceManagement::Util::SerializedType {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Util+SerializedType")]
impl crate::UnityEngine::ResourceManagement::Util::SerializedType {
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
    pub fn get_AssemblyName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_AssemblyName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ClassName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_ClassName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Value",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ValueChanged(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ValueChanged",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Value(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Value",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ValueChanged(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_ValueChanged",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
