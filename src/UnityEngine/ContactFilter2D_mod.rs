#[cfg(feature = "UnityEngine+ContactFilter2D")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ContactFilter2D {
    pub useTriggers: bool,
    pub useLayerMask: bool,
    pub useDepth: bool,
    pub useOutsideDepth: bool,
    pub useNormalAngle: bool,
    pub useOutsideNormalAngle: bool,
    pub layerMask: crate::UnityEngine::LayerMask,
    pub minDepth: f32,
    pub maxDepth: f32,
    pub minNormalAngle: f32,
    pub maxNormalAngle: f32,
}
#[cfg(feature = "UnityEngine+ContactFilter2D")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ContactFilter2D => "UnityEngine"
    ."ContactFilter2D"
);
#[cfg(feature = "UnityEngine+ContactFilter2D")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::ContactFilter2D {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ContactFilter2D")]
impl crate::UnityEngine::ContactFilter2D {
    pub fn CheckConsistency(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CheckConsistency",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckConsistency_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::ContactFilter2D>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CheckConsistency_Injected", (_unity_self))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateLegacyFilter(
        layerMask: i32,
        minDepth: f32,
        maxDepth: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ContactFilter2D> {
        let __cordl_ret: crate::UnityEngine::ContactFilter2D = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateLegacyFilter", (layerMask, minDepth, maxDepth))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDepth(
        &mut self,
        minDepth: f32,
        maxDepth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetDepth",
            (minDepth, maxDepth),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLayerMask(
        &mut self,
        layerMask: crate::UnityEngine::LayerMask,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetLayerMask",
            (layerMask),
        )?;
        Ok(__cordl_ret.into())
    }
}
