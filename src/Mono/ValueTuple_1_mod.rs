#[cfg(feature = "Mono+ValueTuple_1")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ValueTuple_1<T1: quest_hook::libil2cpp::Type> {
    pub Item1: T1,
    __cordl_phantom_T1: std::marker::PhantomData<T1>,
}
#[cfg(feature = "Mono+ValueTuple_1")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::ValueTuple_1 < T1 > => "Mono"
    ."ValueTuple`1<T1>" < T1 >
);
#[cfg(feature = "Mono+ValueTuple_1")]
unsafe impl<T1: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::Mono::ValueTuple_1<T1> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+ValueTuple_1")]
impl<T1: quest_hook::libil2cpp::Type> crate::Mono::ValueTuple_1<T1> {}
