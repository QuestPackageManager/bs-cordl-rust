#[cfg(feature = "HoudiniEngineUnity+TransformData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TransformData {
    pub position: crate::UnityEngine::Vector3,
    pub rotation: crate::UnityEngine::Quaternion,
    pub localPosition: crate::UnityEngine::Vector3,
    pub localScale: crate::UnityEngine::Vector3,
    pub localRotation: crate::UnityEngine::Quaternion,
    pub parent: *mut crate::UnityEngine::Transform,
}
#[cfg(feature = "HoudiniEngineUnity+TransformData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::TransformData =>
    "HoudiniEngineUnity"."TransformData"
);
#[cfg(feature = "HoudiniEngineUnity+TransformData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::HoudiniEngineUnity::TransformData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "HoudiniEngineUnity+TransformData")]
impl crate::HoudiniEngineUnity::TransformData {
    pub fn _ctor(
        &mut self,
        other: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CopyTo(
        &mut self,
        other: *mut crate::UnityEngine::Transform,
        copyParent: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyTo",
            (other, copyParent),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CopyToLocal(
        &mut self,
        other: *mut crate::UnityEngine::Transform,
        copyParent: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CopyToLocal",
            (other, copyParent),
        )?;
        Ok(__cordl_ret)
    }
}
