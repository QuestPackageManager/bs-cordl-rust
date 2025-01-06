#[cfg(feature = "Mono+ValueTuple_3")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ValueTuple_3<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
    T3: quest_hook::libil2cpp::Type,
> {
    pub Item1: T1,
    pub Item2: T2,
    pub Item3: T3,
    __cordl_phantom_T1: std::marker::PhantomData<T1>,
    __cordl_phantom_T2: std::marker::PhantomData<T2>,
    __cordl_phantom_T3: std::marker::PhantomData<T3>,
}
#[cfg(feature = "Mono+ValueTuple_3")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::ValueTuple_3 < T1, T2, T3 > => "Mono"
    ."ValueTuple`3<T1,T2,T3>" < T1, T2, T3 >
);
#[cfg(feature = "Mono+ValueTuple_3")]
unsafe impl<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
    T3: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ThisArgument for crate::Mono::ValueTuple_3<T1, T2, T3> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Mono+ValueTuple_3")]
impl<
    T1: quest_hook::libil2cpp::Type,
    T2: quest_hook::libil2cpp::Type,
    T3: quest_hook::libil2cpp::Type,
> crate::Mono::ValueTuple_3<T1, T2, T3> {}
