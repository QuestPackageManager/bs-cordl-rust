#[cfg(feature = "OVRGLTFScene")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRGLTFScene {
    pub root: *mut crate::UnityEngine::GameObject,
    pub nodes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::GameObject,
    >,
    pub animationNodes: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::GlobalNamespace::OVRGLTFInputNode,
        *mut crate::GlobalNamespace::OVRGLTFAnimatinonNode,
    >,
    pub animationNodeLookup: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::OVRGLTFAnimatinonNode,
        >,
    >,
    pub morphTargetHandlers: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::OVRGLTFAnimationNodeMorphTargetHandler,
    >,
}
#[cfg(feature = "OVRGLTFScene")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRGLTFScene => ""
    ."OVRGLTFScene"
);
#[cfg(feature = "OVRGLTFScene")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRGLTFScene {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRGLTFScene")]
impl crate::GlobalNamespace::OVRGLTFScene {}
