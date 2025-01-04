#[cfg(feature = "OVR+OpenVR+EIOBufferError")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EIOBufferError {
    #[default]
    IOBuffer_InvalidArgument = 102i32,
    IOBuffer_InvalidHandle = 101i32,
    IOBuffer_OperationFailed = 100i32,
    IOBuffer_PathDoesNotExist = 104i32,
    IOBuffer_PathExists = 103i32,
    IOBuffer_Permission = 105i32,
    IOBuffer_Success = 0i32,
}
#[cfg(feature = "OVR+OpenVR+EIOBufferError")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::OVR::OpenVR::EIOBufferError => "OVR.OpenVR"
    ."EIOBufferError"
);
