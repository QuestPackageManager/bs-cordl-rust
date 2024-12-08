#[cfg(feature = "Newtonsoft+Json+Converters+StringEnumConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct StringEnumConverter {
    __cordl_parent: crate::Newtonsoft::Json::JsonConverter,
    pub _NamingStrategy_k__BackingField: *mut crate::Newtonsoft::Json::Serialization::NamingStrategy,
    pub _AllowIntegerValues_k__BackingField: bool,
}
#[cfg(feature = "Newtonsoft+Json+Converters+StringEnumConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Converters::StringEnumConverter => "Newtonsoft.Json.Converters"
    ."StringEnumConverter"
);
#[cfg(feature = "Newtonsoft+Json+Converters+StringEnumConverter")]
impl std::ops::Deref for crate::Newtonsoft::Json::Converters::StringEnumConverter {
    type Target = crate::Newtonsoft::Json::JsonConverter;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+StringEnumConverter")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Converters::StringEnumConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+StringEnumConverter")]
impl crate::Newtonsoft::Json::Converters::StringEnumConverter {
    pub fn set_AllowIntegerValues(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AllowIntegerValues", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_NamingStrategy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::NamingStrategy,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::NamingStrategy = __cordl_object
            .invoke("get_NamingStrategy", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CamelCaseText(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CamelCaseText", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteJson(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        value: *mut crate::System::Object,
        serializer: *mut crate::Newtonsoft::Json::JsonSerializer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteJson", (writer, value, serializer))?;
        Ok(__cordl_ret)
    }
    pub fn CanConvert(
        &mut self,
        objectType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanConvert", (objectType))?;
        Ok(__cordl_ret)
    }
    pub fn set_CamelCaseText(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CamelCaseText", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_AllowIntegerValues(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AllowIntegerValues", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor__cordl_bool1(
        &mut self,
        camelCaseText: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (camelCaseText))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_NamingStrategy__cordl_bool2(
        &mut self,
        namingStrategy: *mut crate::Newtonsoft::Json::Serialization::NamingStrategy,
        allowIntegerValues: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (namingStrategy, allowIntegerValues))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Type3(
        &mut self,
        namingStrategyType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (namingStrategyType))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Type_Il2CppArray4(
        &mut self,
        namingStrategyType: *mut crate::System::Type,
        namingStrategyParameters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (namingStrategyType, namingStrategyParameters))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Type_Il2CppArray__cordl_bool5(
        &mut self,
        namingStrategyType: *mut crate::System::Type,
        namingStrategyParameters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Object,
        >,
        allowIntegerValues: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (namingStrategyType, namingStrategyParameters, allowIntegerValues),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ReadJson(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        objectType: *mut crate::System::Type,
        existingValue: *mut crate::System::Object,
        serializer: *mut crate::Newtonsoft::Json::JsonSerializer,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ReadJson", (reader, objectType, existingValue, serializer))?;
        Ok(__cordl_ret)
    }
    pub fn set_NamingStrategy(
        &mut self,
        value: *mut crate::Newtonsoft::Json::Serialization::NamingStrategy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_NamingStrategy", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New__cordl_bool1(
        camelCaseText: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (camelCaseText))?;
        Ok(__cordl_object)
    }
    pub fn New_NamingStrategy__cordl_bool2(
        namingStrategy: *mut crate::Newtonsoft::Json::Serialization::NamingStrategy,
        allowIntegerValues: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (namingStrategy, allowIntegerValues))?;
        Ok(__cordl_object)
    }
    pub fn New_Type3(
        namingStrategyType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (namingStrategyType))?;
        Ok(__cordl_object)
    }
    pub fn New_Type_Il2CppArray4(
        namingStrategyType: *mut crate::System::Type,
        namingStrategyParameters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (namingStrategyType, namingStrategyParameters))?;
        Ok(__cordl_object)
    }
    pub fn New_Type_Il2CppArray__cordl_bool5(
        namingStrategyType: *mut crate::System::Type,
        namingStrategyParameters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Object,
        >,
        allowIntegerValues: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (namingStrategyType, namingStrategyParameters, allowIntegerValues),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Converters+StringEnumConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Converters::StringEnumConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
