#[cfg(feature = "UnityEngine+TransformDispatchData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TransformDispatchData {
    pub transformedID: crate::Unity::Collections::NativeArray_1<i32>,
    pub parentID: crate::Unity::Collections::NativeArray_1<i32>,
    pub localToWorldMatrices: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Matrix4x4,
    >,
    pub positions: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector3>,
    pub rotations: crate::Unity::Collections::NativeArray_1<
        crate::UnityEngine::Quaternion,
    >,
    pub scales: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector3>,
}
#[cfg(feature = "UnityEngine+TransformDispatchData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TransformDispatchData =>
    "UnityEngine"."TransformDispatchData"
);
#[cfg(feature = "UnityEngine+TransformDispatchData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TransformDispatchData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TransformDispatchData")]
impl crate::UnityEngine::TransformDispatchData {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+TransformDispatchData")]
impl AsRef<crate::System::IDisposable> for crate::UnityEngine::TransformDispatchData {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+TransformDispatchData")]
impl AsMut<crate::System::IDisposable> for crate::UnityEngine::TransformDispatchData {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
