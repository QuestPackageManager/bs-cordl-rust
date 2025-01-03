#[cfg(feature = "Unity+Collections+NativeArrayDisposeJob")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct NativeArrayDisposeJob {
    pub Data: crate::Unity::Collections::NativeArrayDispose,
}
#[cfg(feature = "Unity+Collections+NativeArrayDisposeJob")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Collections::NativeArrayDisposeJob =>
    "Unity.Collections"."NativeArrayDisposeJob"
);
#[cfg(feature = "Unity+Collections+NativeArrayDisposeJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Collections::NativeArrayDisposeJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Collections+NativeArrayDisposeJob")]
impl crate::Unity::Collections::NativeArrayDisposeJob {
    pub fn Execute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterNativeArrayDisposeJobReflectionData() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterNativeArrayDisposeJobReflectionData", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Collections+NativeArrayDisposeJob")]
impl AsRef<crate::Unity::Jobs::IJob>
for crate::Unity::Collections::NativeArrayDisposeJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJob {
        todo!()
    }
}
#[cfg(feature = "Unity+Collections+NativeArrayDisposeJob")]
impl AsMut<crate::Unity::Jobs::IJob>
for crate::Unity::Collections::NativeArrayDisposeJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJob {
        todo!()
    }
}
