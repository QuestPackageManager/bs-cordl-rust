#[cfg(feature = "cordl_class_System+Net+Sockets+AddressFamily")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AddressFamily {
    #[default]
    AppleTalk = 16i32,
    Atm = 22i32,
    Banyan = 21i32,
    Ccitt = 10i32,
    Chaos = 5i32,
    Cluster = 24i32,
    DataKit = 9i32,
    DataLink = 13i32,
    DecNet = 12i32,
    Ecma = 8i32,
    FireFox = 19i32,
    HyperChannel = 15i32,
    Ieee12844 = 25i32,
    ImpLink = 3i32,
    InterNetwork = 2i32,
    InterNetworkV6 = 23i32,
    Ipx = 6i32,
    Irda = 26i32,
    Iso = 7i32,
    Lat = 14i32,
    Max = 29i32,
    NetBios = 17i32,
    NetworkDesigners = 28i32,
    Pup = 4i32,
    Sna = 11i32,
    Unix = 1i32,
    Unknown = -1i32,
    Unspecified = 0i32,
    VoiceView = 18i32,
}
#[cfg(feature = "cordl_class_System+Net+Sockets+AddressFamily")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::Sockets::AddressFamily {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net.Sockets";
    const CLASS_NAME: &'static str = "AddressFamily";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_System+Net+Sockets+AddressFamily")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Net::Sockets::AddressFamily {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_System+Net+Sockets+AddressFamily")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Net::Sockets::AddressFamily {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_System+Net+Sockets+AddressFamily")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Net::Sockets::AddressFamily {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_System+Net+Sockets+AddressFamily")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Net::Sockets::AddressFamily {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
