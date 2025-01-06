#[cfg(feature = "Newtonsoft+Json+Serialization+JsonTypeReflector")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonTypeReflector {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonTypeReflector")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::JsonTypeReflector =>
    "Newtonsoft.Json.Serialization"."JsonTypeReflector"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonTypeReflector")]
impl std::ops::Deref for crate::Newtonsoft::Json::Serialization::JsonTypeReflector {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonTypeReflector")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Serialization::JsonTypeReflector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonTypeReflector")]
impl crate::Newtonsoft::Json::Serialization::JsonTypeReflector {
    pub const ArrayValuesPropertyName: &'static str = "$values";
    pub const ConcurrentDictionaryTypeName: &'static str = "System.Collections.Concurrent.ConcurrentDictionary`2";
    pub const IdPropertyName: &'static str = "$id";
    pub const RefPropertyName: &'static str = "$ref";
    pub const ShouldSerializePrefix: &'static str = "ShouldSerialize";
    pub const SpecifiedPostfix: &'static str = "Specified";
    pub const TypePropertyName: &'static str = "$type";
    pub const ValuePropertyName: &'static str = "$value";
    pub fn CanTypeDescriptorConvertString(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        typeConverter: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::ComponentModel::TypeConverter>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CanTypeDescriptorConvertString", (_cordl_type, typeConverter))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateJsonConverterInstance(
        converterType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonConverter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonConverter,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateJsonConverterInstance", (converterType, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateNamingStrategyInstance(
        namingStrategyType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::NamingStrategy>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::NamingStrategy,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateNamingStrategyInstance", (namingStrategyType, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssociateMetadataTypeFromAttribute(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssociateMetadataTypeFromAttribute", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssociatedMetadataType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssociatedMetadataType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttribute_Il2CppObject2<T>(
        provider: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAttribute", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttribute_MemberInfo1<T>(
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAttribute", (memberInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttribute_Type0<T>(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAttribute", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCachedAttribute<T>(
        attributeProvider: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCachedAttribute", (attributeProvider))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContainerNamingStrategy(
        containerAttribute: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonContainerAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::NamingStrategy>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::NamingStrategy,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetContainerNamingStrategy", (containerAttribute))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCreator(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    >,
                >,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<
                    quest_hook::libil2cpp::Il2CppArray<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    >,
                >,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCreator", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDataContractAttribute(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContractAttribute,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataContractAttribute,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDataContractAttribute", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDataMemberAttribute(
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataMemberAttribute,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::DataMemberAttribute,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDataMemberAttribute", (memberInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetJsonConverter(
        attributeProvider: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonConverter>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonConverter,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetJsonConverter", (attributeProvider))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectMemberSerialization(
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        ignoreSerializableAttribute: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::MemberSerialization> {
        let __cordl_ret: crate::Newtonsoft::Json::MemberSerialization = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetObjectMemberSerialization",
                (objectType, ignoreSerializableAttribute),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNonSerializable(
        provider: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNonSerializable", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSerializable(
        provider: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSerializable", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DynamicCodeGeneration() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_DynamicCodeGeneration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FullyTrusted() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_FullyTrusted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ReflectionDelegateFactory() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::ReflectionDelegateFactory,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::ReflectionDelegateFactory,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ReflectionDelegateFactory", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonTypeReflector")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonTypeReflector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
