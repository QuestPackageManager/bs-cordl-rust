#[cfg(feature = "UnityEngine+UIElements+MeshWriteDataInterface")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MeshWriteDataInterface {
    pub vertices: crate::System::IntPtr,
    pub indices: crate::System::IntPtr,
    pub vertexCount: i32,
    pub indexCount: i32,
}
#[cfg(feature = "UnityEngine+UIElements+MeshWriteDataInterface")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::MeshWriteDataInterface
    => "UnityEngine.UIElements"."MeshWriteDataInterface"
);
#[cfg(feature = "UnityEngine+UIElements+MeshWriteDataInterface")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::MeshWriteDataInterface {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MeshWriteDataInterface")]
impl crate::UnityEngine::UIElements::MeshWriteDataInterface {}
