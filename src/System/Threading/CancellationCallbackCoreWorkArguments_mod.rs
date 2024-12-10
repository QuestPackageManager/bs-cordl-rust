#[cfg(feature = "System+Threading+CancellationCallbackCoreWorkArguments")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CancellationCallbackCoreWorkArguments {
    pub _currArrayFragment: *mut crate::System::Threading::SparselyPopulatedArrayFragment_1<
        *mut crate::System::Threading::CancellationCallbackInfo,
    >,
    pub _currArrayIndex: i32,
}
#[cfg(feature = "System+Threading+CancellationCallbackCoreWorkArguments")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::CancellationCallbackCoreWorkArguments => "System.Threading"
    ."CancellationCallbackCoreWorkArguments"
);
#[cfg(feature = "System+Threading+CancellationCallbackCoreWorkArguments")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Threading::CancellationCallbackCoreWorkArguments {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Threading+CancellationCallbackCoreWorkArguments")]
impl crate::System::Threading::CancellationCallbackCoreWorkArguments {
    pub fn _ctor(
        &mut self,
        currArrayFragment: quest_hook::libil2cpp::Gc<
            crate::System::Threading::SparselyPopulatedArrayFragment_1<
                *mut crate::System::Threading::CancellationCallbackInfo,
            >,
        >,
        currArrayIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (currArrayFragment, currArrayIndex),
        )?;
        Ok(__cordl_ret.into())
    }
}
