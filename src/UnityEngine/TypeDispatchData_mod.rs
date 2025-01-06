#[cfg(feature = "UnityEngine+TypeDispatchData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TypeDispatchData {
    pub changed: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        >,
    >,
    pub changedID: crate::Unity::Collections::NativeArray_1<i32>,
    pub destroyedID: crate::Unity::Collections::NativeArray_1<i32>,
}
#[cfg(feature = "UnityEngine+TypeDispatchData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::TypeDispatchData => "UnityEngine"
    ."TypeDispatchData"
);
#[cfg(feature = "UnityEngine+TypeDispatchData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::TypeDispatchData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+TypeDispatchData")]
impl crate::UnityEngine::TypeDispatchData {
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
#[cfg(feature = "UnityEngine+TypeDispatchData")]
impl AsRef<crate::System::IDisposable> for crate::UnityEngine::TypeDispatchData {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+TypeDispatchData")]
impl AsMut<crate::System::IDisposable> for crate::UnityEngine::TypeDispatchData {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
