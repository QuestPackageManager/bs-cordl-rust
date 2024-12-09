#[cfg(feature = "UnityEngine+SkeletonBone")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SkeletonBone {
    pub name: *mut quest_hook::libil2cpp::Il2CppString,
    pub parentName: *mut quest_hook::libil2cpp::Il2CppString,
    pub position: crate::UnityEngine::Vector3,
    pub rotation: crate::UnityEngine::Quaternion,
    pub scale: crate::UnityEngine::Vector3,
}
#[cfg(feature = "UnityEngine+SkeletonBone")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::SkeletonBone => "UnityEngine"
    ."SkeletonBone"
);
#[cfg(feature = "UnityEngine+SkeletonBone")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::SkeletonBone {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+SkeletonBone")]
impl crate::UnityEngine::SkeletonBone {}
