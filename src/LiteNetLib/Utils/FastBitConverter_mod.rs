#[cfg(feature = "LiteNetLib+Utils+FastBitConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct FastBitConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "LiteNetLib+Utils+FastBitConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::LiteNetLib::Utils::FastBitConverter =>
    "LiteNetLib.Utils"."FastBitConverter"
);
#[cfg(feature = "LiteNetLib+Utils+FastBitConverter")]
impl std::ops::Deref for crate::LiteNetLib::Utils::FastBitConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn GetBytes_f32_1(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        startIndex: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBytes", (bytes, startIndex, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBytes_f64_0(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        startIndex: i32,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBytes", (bytes, startIndex, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBytes_i16_2(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        startIndex: i32,
        value: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBytes", (bytes, startIndex, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBytes_i32_4(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        startIndex: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBytes", (bytes, startIndex, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBytes_i64_6(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        startIndex: i32,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBytes", (bytes, startIndex, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBytes_u16_3(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        startIndex: i32,
        value: u16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBytes", (bytes, startIndex, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBytes_u32_5(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        startIndex: i32,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBytes", (bytes, startIndex, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBytes_u64_7(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        startIndex: i32,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBytes", (bytes, startIndex, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteLittleEndian_i16_2(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        data: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteLittleEndian", (buffer, offset, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteLittleEndian_i32_1(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        data: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteLittleEndian", (buffer, offset, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteLittleEndian_u64_0(
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        data: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteLittleEndian", (buffer, offset, data))?;
        Ok(__cordl_ret.into())
    }
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FastBitConverter_ConverterHelperDouble {
    padding: quest_hook::libil2cpp::ValueTypePadding<8usize>,
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FastBitConverter_ConverterHelperFloat {
    padding: quest_hook::libil2cpp::ValueTypePadding<4usize>,
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
