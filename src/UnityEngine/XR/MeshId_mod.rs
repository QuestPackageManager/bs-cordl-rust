#[cfg(feature = "UnityEngine+XR+MeshId")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct MeshId {
    pub m_SubId1: u64,
    pub m_SubId2: u64,
}
#[cfg(feature = "UnityEngine+XR+MeshId")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::XR::MeshId => "UnityEngine.XR"
    ."MeshId"
);
#[cfg(feature = "UnityEngine+XR+MeshId")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::XR::MeshId {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+XR+MeshId")]
impl crate::UnityEngine::XR::MeshId {
    pub fn Equals_Gc0(
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
    pub fn Equals_MeshId1(
        &mut self,
        other: crate::UnityEngine::XR::MeshId,
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
}
#[cfg(feature = "UnityEngine+XR+MeshId")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::MeshId>>
for crate::UnityEngine::XR::MeshId {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::MeshId> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+XR+MeshId")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::MeshId>>
for crate::UnityEngine::XR::MeshId {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::UnityEngine::XR::MeshId> {
        todo!()
    }
}
