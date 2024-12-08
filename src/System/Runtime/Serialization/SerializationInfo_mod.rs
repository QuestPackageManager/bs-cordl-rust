#[cfg(feature = "System+Runtime+Serialization+SerializationInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct SerializationInfo {
    __cordl_parent: crate::System::Object,
    pub m_members: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub m_data: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    pub m_types: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    pub m_nameToIndex: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        i32,
    >,
    pub m_currMember: i32,
    pub m_converter: *mut crate::System::Runtime::Serialization::IFormatterConverter,
    pub m_fullTypeName: *mut crate::System::String,
    pub m_assemName: *mut crate::System::String,
    pub objectType: *mut crate::System::Type,
    pub isFullTypeNameSetExplicit: bool,
    pub isAssemblyNameSetExplicit: bool,
    pub requireSameTokenInPartialTrust: bool,
}
#[cfg(feature = "System+Runtime+Serialization+SerializationInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::SerializationInfo => "System.Runtime.Serialization"
    ."SerializationInfo"
);
#[cfg(feature = "System+Runtime+Serialization+SerializationInfo")]
impl std::ops::Deref for crate::System::Runtime::Serialization::SerializationInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SerializationInfo")]
impl std::ops::DerefMut for crate::System::Runtime::Serialization::SerializationInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Serialization+SerializationInfo")]
impl crate::System::Runtime::Serialization::SerializationInfo {
    pub const defaultSize: i32 = 4i32;
    pub const s_mscorlibAssemblySimpleName: &'static str = "mscorlib";
    pub const s_mscorlibFileName: &'static str = "mscorlib.dll";
    pub fn AddValueInternal(
        &mut self,
        name: *mut crate::System::String,
        value: *mut crate::System::Object,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddValueInternal", (name, value, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn AddValue_DateTime9(
        &mut self,
        name: *mut crate::System::String,
        value: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddValue", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn AddValue_Object1(
        &mut self,
        name: *mut crate::System::String,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddValue", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn AddValue_Object_Type0(
        &mut self,
        name: *mut crate::System::String,
        value: *mut crate::System::Object,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddValue", (name, value, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn AddValue__cordl_bool2(
        &mut self,
        name: *mut crate::System::String,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddValue", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn AddValue_f32_8(
        &mut self,
        name: *mut crate::System::String,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddValue", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn AddValue_i16_4(
        &mut self,
        name: *mut crate::System::String,
        value: i16,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddValue", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn AddValue_i32_5(
        &mut self,
        name: *mut crate::System::String,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddValue", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn AddValue_i64_6(
        &mut self,
        name: *mut crate::System::String,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddValue", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn AddValue_u64_7(
        &mut self,
        name: *mut crate::System::String,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddValue", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn AddValue_u8_3(
        &mut self,
        name: *mut crate::System::String,
        value: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddValue", (name, value))?;
        Ok(__cordl_ret)
    }
    pub fn ExpandArrays(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExpandArrays", ())?;
        Ok(__cordl_ret)
    }
    pub fn FindElement(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindElement", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetBoolean(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetBoolean", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetElement(
        &mut self,
        name: *mut crate::System::String,
        foundType: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetElement", (name, foundType))?;
        Ok(__cordl_ret)
    }
    pub fn GetElementNoThrow(
        &mut self,
        name: *mut crate::System::String,
        foundType: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetElementNoThrow", (name, foundType))?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::SerializationInfoEnumerator,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::SerializationInfoEnumerator = __cordl_object
            .invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetInt32(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetInt32", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetInt64(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("GetInt64", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetSingle(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("GetSingle", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetString(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetString", (name))?;
        Ok(__cordl_ret)
    }
    pub fn GetValue(
        &mut self,
        name: *mut crate::System::String,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetValue", (name, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetValueNoThrow(
        &mut self,
        name: *mut crate::System::String,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("GetValueNoThrow", (name, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn New_Type_IFormatterConverter0(
        _cordl_type: *mut crate::System::Type,
        converter: *mut crate::System::Runtime::Serialization::IFormatterConverter,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, converter))?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool1(
        _cordl_type: *mut crate::System::Type,
        converter: *mut crate::System::Runtime::Serialization::IFormatterConverter,
        requireSameTokenInPartialTrust: bool,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (_cordl_type, converter, requireSameTokenInPartialTrust),
            )?;
        Ok(__cordl_object)
    }
    pub fn SetType(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetType", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateValue(
        &mut self,
        name: *mut crate::System::String,
        value: *mut crate::System::Object,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateValue", (name, value, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Type_IFormatterConverter0(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        converter: *mut crate::System::Runtime::Serialization::IFormatterConverter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, converter))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        converter: *mut crate::System::Runtime::Serialization::IFormatterConverter,
        requireSameTokenInPartialTrust: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, converter, requireSameTokenInPartialTrust))?;
        Ok(__cordl_ret)
    }
    pub fn get_AssemblyName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AssemblyName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FullTypeName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_FullTypeName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsAssemblyNameSetExplicit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsAssemblyNameSetExplicit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsFullTypeNameSetExplicit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IsFullTypeNameSetExplicit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MemberCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MemberCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ObjectType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_ObjectType", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+Serialization+SerializationInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Serialization::SerializationInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
