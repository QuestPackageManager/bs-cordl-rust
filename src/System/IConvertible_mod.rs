#[cfg(feature = "System+IConvertible")]
#[repr(C)]
#[derive(Debug)]
pub struct IConvertible {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+IConvertible")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::IConvertible => "System"."IConvertible"
);
#[cfg(feature = "System+IConvertible")]
impl std::ops::Deref for crate::System::IConvertible {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+IConvertible")]
impl std::ops::DerefMut for crate::System::IConvertible {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+IConvertible")]
impl crate::System::IConvertible {
    pub fn ToInt16(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i16 = __cordl_object.invoke("ToInt16", (provider))?;
        Ok(__cordl_ret)
    }
    pub fn ToUInt64(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("ToUInt64", (provider))?;
        Ok(__cordl_ret)
    }
    pub fn ToDateTime(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("ToDateTime", (provider))?;
        Ok(__cordl_ret)
    }
    pub fn ToSByte(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i8 = __cordl_object.invoke("ToSByte", (provider))?;
        Ok(__cordl_ret)
    }
    pub fn ToInt32(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ToInt32", (provider))?;
        Ok(__cordl_ret)
    }
    pub fn ToBoolean(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ToBoolean", (provider))?;
        Ok(__cordl_ret)
    }
    pub fn ToByte(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object.invoke("ToByte", (provider))?;
        Ok(__cordl_ret)
    }
    pub fn GetTypeCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TypeCode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TypeCode = __cordl_object
            .invoke("GetTypeCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToDouble(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("ToDouble", (provider))?;
        Ok(__cordl_ret)
    }
    pub fn ToUInt16(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u16 = __cordl_object.invoke("ToUInt16", (provider))?;
        Ok(__cordl_ret)
    }
    pub fn ToType(
        &mut self,
        conversionType: *mut crate::System::Type,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ToType", (conversionType, provider))?;
        Ok(__cordl_ret)
    }
    pub fn ToUInt32(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("ToUInt32", (provider))?;
        Ok(__cordl_ret)
    }
    pub fn ToSingle(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("ToSingle", (provider))?;
        Ok(__cordl_ret)
    }
    pub fn ToDecimal(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Decimal = __cordl_object
            .invoke("ToDecimal", (provider))?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", (provider))?;
        Ok(__cordl_ret)
    }
    pub fn ToInt64(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("ToInt64", (provider))?;
        Ok(__cordl_ret)
    }
    pub fn ToChar(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("ToChar", (provider))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+IConvertible")]
impl quest_hook::libil2cpp::ObjectType for crate::System::IConvertible {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
