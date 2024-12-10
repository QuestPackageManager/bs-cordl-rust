#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+BatchQueryJob_2")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BatchQueryJob_2<
    CommandT: quest_hook::libil2cpp::Type,
    ResultT: quest_hook::libil2cpp::Type,
> {
    pub commands: crate::Unity::Collections::NativeArray_1<CommandT>,
    pub results: crate::Unity::Collections::NativeArray_1<ResultT>,
    __cordl_phantom_CommandT: std::marker::PhantomData<CommandT>,
    __cordl_phantom_ResultT: std::marker::PhantomData<ResultT>,
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+BatchQueryJob_2")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Jobs::LowLevel::Unsafe::BatchQueryJob_2 <
    CommandT, ResultT > => "Unity.Jobs.LowLevel.Unsafe"
    ."BatchQueryJob`2<CommandT,ResultT>" < CommandT, ResultT >
);
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+BatchQueryJob_2")]
unsafe impl<
    CommandT: quest_hook::libil2cpp::Type,
    ResultT: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ThisArgument
for crate::Unity::Jobs::LowLevel::Unsafe::BatchQueryJob_2<CommandT, ResultT> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+BatchQueryJob_2")]
impl<
    CommandT: quest_hook::libil2cpp::Type,
    ResultT: quest_hook::libil2cpp::Type,
> crate::Unity::Jobs::LowLevel::Unsafe::BatchQueryJob_2<CommandT, ResultT> {
    pub fn _ctor(
        &mut self,
        commands: crate::Unity::Collections::NativeArray_1<CommandT>,
        results: crate::Unity::Collections::NativeArray_1<ResultT>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        CommandT: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        ResultT: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (commands, results),
        )?;
        Ok(__cordl_ret.into())
    }
}
