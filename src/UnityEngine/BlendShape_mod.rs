#[cfg(feature = "UnityEngine+BlendShape")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BlendShape {
    pub m_FirstVertex: u32,
    pub m_VertexCount: u32,
    pub m_HasNormals: bool,
    pub m_HasTangents: bool,
}
#[cfg(feature = "UnityEngine+BlendShape")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::BlendShape => "UnityEngine"
    ."BlendShape"
);
#[cfg(feature = "UnityEngine+BlendShape")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::BlendShape {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+BlendShape")]
impl crate::UnityEngine::BlendShape {
    pub fn get_firstVertex(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_firstVertex",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_vertexCount(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_vertexCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
