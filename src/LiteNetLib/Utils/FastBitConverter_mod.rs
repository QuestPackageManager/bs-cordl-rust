#[cfg(feature = "LiteNetLib+Utils+FastBitConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct FastBitConverter {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "LiteNetLib+Utils+FastBitConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::Utils::FastBitConverter =>
    "LiteNetLib.Utils"."FastBitConverter"
);
#[cfg(feature = "LiteNetLib+Utils+FastBitConverter")]
impl std::ops::Deref for crate::LiteNetLib::Utils::FastBitConverter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+FastBitConverter")]
impl std::ops::DerefMut for crate::LiteNetLib::Utils::FastBitConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LiteNetLib+Utils+FastBitConverter")]
impl crate::LiteNetLib::Utils::FastBitConverter {
    #[cfg(feature = "LiteNetLib+Utils+FastBitConverter+ConverterHelperDouble")]
    pub type ConverterHelperDouble = crate::LiteNetLib::Utils::FastBitConverter_ConverterHelperDouble;
    #[cfg(feature = "LiteNetLib+Utils+FastBitConverter+ConverterHelperFloat")]
    pub type ConverterHelperFloat = crate::LiteNetLib::Utils::FastBitConverter_ConverterHelperFloat;
}
#[cfg(feature = "LiteNetLib+Utils+FastBitConverter")]
impl quest_hook::libil2cpp::ObjectType for crate::LiteNetLib::Utils::FastBitConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LiteNetLib+Utils+FastBitConverter+ConverterHelperDouble")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FastBitConverter_ConverterHelperDouble {
    padding: [u8; 8usize],
}
#[cfg(feature = "LiteNetLib+Utils+FastBitConverter+ConverterHelperDouble")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::FastBitConverter_ConverterHelperDouble => "LiteNetLib.Utils"
    ."FastBitConverter/ConverterHelperDouble"
);
#[cfg(feature = "LiteNetLib+Utils+FastBitConverter+ConverterHelperDouble")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LiteNetLib::Utils::FastBitConverter_ConverterHelperDouble {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LiteNetLib+Utils+FastBitConverter+ConverterHelperDouble")]
impl crate::LiteNetLib::Utils::FastBitConverter_ConverterHelperDouble {}
#[cfg(feature = "LiteNetLib+Utils+FastBitConverter+ConverterHelperFloat")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FastBitConverter_ConverterHelperFloat {
    padding: [u8; 4usize],
}
#[cfg(feature = "LiteNetLib+Utils+FastBitConverter+ConverterHelperFloat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::LiteNetLib::Utils::FastBitConverter_ConverterHelperFloat => "LiteNetLib.Utils"
    ."FastBitConverter/ConverterHelperFloat"
);
#[cfg(feature = "LiteNetLib+Utils+FastBitConverter+ConverterHelperFloat")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::LiteNetLib::Utils::FastBitConverter_ConverterHelperFloat {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "LiteNetLib+Utils+FastBitConverter+ConverterHelperFloat")]
impl crate::LiteNetLib::Utils::FastBitConverter_ConverterHelperFloat {}
