#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+BatchQueryJobStruct_1")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BatchQueryJobStruct_1<T: quest_hook::libil2cpp::Type> {
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+BatchQueryJobStruct_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Jobs::LowLevel::Unsafe::BatchQueryJobStruct_1 < T > =>
    "Unity.Jobs.LowLevel.Unsafe"."BatchQueryJobStruct`1<T>" < T >
);
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+BatchQueryJobStruct_1")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::Unity::Jobs::LowLevel::Unsafe::BatchQueryJobStruct_1<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+BatchQueryJobStruct_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::Unity::Jobs::LowLevel::Unsafe::BatchQueryJobStruct_1<T> {
    pub fn Initialize() -> quest_hook::libil2cpp::Result<crate::System::IntPtr>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::System::IntPtr = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
}
