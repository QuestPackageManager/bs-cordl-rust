#[cfg(feature = "UnityEngine+LOD")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LOD {
    pub screenRelativeTransitionHeight: f32,
    pub fadeTransitionWidth: f32,
    pub renderers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Renderer,
    >,
}
#[cfg(feature = "UnityEngine+LOD")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::LOD => "UnityEngine"."LOD"
);
#[cfg(feature = "UnityEngine+LOD")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::LOD {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+LOD")]
impl crate::UnityEngine::LOD {
    pub fn _ctor(
        &mut self,
        screenRelativeTransitionHeight: f32,
        renderers: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Renderer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (screenRelativeTransitionHeight, renderers),
        )?;
        Ok(__cordl_ret)
    }
}