#[cfg(feature = "Newtonsoft+Json+Converters+StringEnumConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct StringEnumConverter {
    __cordl_parent: crate::Newtonsoft::Json::JsonConverter,
    pub _NamingStrategy_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::NamingStrategy,
    >,
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
    pub fn CanConvert(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanConvert", (objectType))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_NamingStrategy__cordl_bool2(
        namingStrategy: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::NamingStrategy,
        >,
        allowIntegerValues: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (namingStrategy, allowIntegerValues))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Type3(
        namingStrategyType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (namingStrategyType))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Type_Il2CppArray4(
        namingStrategyType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        namingStrategyParameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (namingStrategyType, namingStrategyParameters))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Type_Il2CppArray__cordl_bool5(
        namingStrategyType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        namingStrategyParameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
        allowIntegerValues: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (namingStrategyType, namingStrategyParameters, allowIntegerValues),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool1(
        camelCaseText: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (camelCaseText))?;
        Ok(__cordl_object.into())
    }
    pub fn ReadJson(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonReader>,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        existingValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        serializer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke("ReadJson", (reader, objectType, existingValue, serializer))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteJson(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        serializer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteJson", (writer, value, serializer))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_NamingStrategy__cordl_bool2(
        &mut self,
        namingStrategy: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::NamingStrategy,
        >,
        allowIntegerValues: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (namingStrategy, allowIntegerValues))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Type3(
        &mut self,
        namingStrategyType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (namingStrategyType))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Type_Il2CppArray4(
        &mut self,
        namingStrategyType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        namingStrategyParameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (namingStrategyType, namingStrategyParameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Type_Il2CppArray__cordl_bool5(
        &mut self,
        namingStrategyType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        namingStrategyParameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_AllowIntegerValues(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_AllowIntegerValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CamelCaseText(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CamelCaseText", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NamingStrategy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::NamingStrategy>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::NamingStrategy,
        > = __cordl_object.invoke("get_NamingStrategy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AllowIntegerValues(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AllowIntegerValues", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_NamingStrategy(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::NamingStrategy,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_NamingStrategy", (value))?;
        Ok(__cordl_ret.into())
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
