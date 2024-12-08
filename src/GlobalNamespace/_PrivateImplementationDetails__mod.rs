#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_10")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct _PrivateImplementationDetails____StaticArrayInitTypeSize_10 {}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_10")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_10 => ""
    ."<PrivateImplementationDetails>/__StaticArrayInitTypeSize=10"
);
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_10")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_10 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "_PrivateImplementationDetails_+__StaticArrayInitTypeSize_10")]
impl crate::GlobalNamespace::_PrivateImplementationDetails____StaticArrayInitTypeSize_10 {}
