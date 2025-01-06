#[cfg(feature = "UnityEngine+CombineInstance")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CombineInstance {
    pub m_MeshInstanceID: i32,
    pub m_SubMeshIndex: i32,
    pub m_Transform: crate::UnityEngine::Matrix4x4,
    pub m_LightmapScaleOffset: crate::UnityEngine::Vector4,
    pub m_RealtimeLightmapScaleOffset: crate::UnityEngine::Vector4,
}
#[cfg(feature = "UnityEngine+CombineInstance")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::CombineInstance => "UnityEngine"
    ."CombineInstance"
);
#[cfg(feature = "UnityEngine+CombineInstance")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::CombineInstance {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+CombineInstance")]
impl crate::UnityEngine::CombineInstance {
    pub fn get_mesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_mesh",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_mesh(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_mesh",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_subMeshIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_subMeshIndex",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_transform(
        &mut self,
        value: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_transform",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
