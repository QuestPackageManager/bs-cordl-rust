#[cfg(feature = "System+IO+SeekOrigin")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeekOrigin {
    Begin = 0i32,
    Current = 1i32,
    End = 2i32,
}
#[cfg(feature = "System+IO+SeekOrigin")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::IO::SeekOrigin => "System.IO"
    ."SeekOrigin"
);
