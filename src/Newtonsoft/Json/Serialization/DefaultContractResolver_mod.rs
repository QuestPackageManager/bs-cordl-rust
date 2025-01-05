#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultContractResolver")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultContractResolver {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _nameTable: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::DefaultJsonNameTable,
    >,
    pub _contractCache: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<crate::System::Type>,
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::JsonContract>,
    >,
    pub _DefaultMembersSearchFlags_k__BackingField: crate::System::Reflection::BindingFlags,
    pub _SerializeCompilerGeneratedMembers_k__BackingField: bool,
    pub _IgnoreSerializableInterface_k__BackingField: bool,
    pub _IgnoreSerializableAttribute_k__BackingField: bool,
    pub _IgnoreIsSpecifiedMembers_k__BackingField: bool,
    pub _IgnoreShouldSerializeMembers_k__BackingField: bool,
    pub _NamingStrategy_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::NamingStrategy,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultContractResolver")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::DefaultContractResolver =>
    "Newtonsoft.Json.Serialization"."DefaultContractResolver"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultContractResolver")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Serialization::DefaultContractResolver {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultContractResolver")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::DefaultContractResolver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultContractResolver")]
impl crate::Newtonsoft::Json::Serialization::DefaultContractResolver {
    #[cfg(
        feature = "Newtonsoft+Json+Serialization+DefaultContractResolver+EnumerableDictionaryWrapper_2"
    )]
    pub type EnumerableDictionaryWrapper_2<
        TEnumeratorKey: quest_hook::libil2cpp::Type,
        TEnumeratorValue: quest_hook::libil2cpp::Type,
    > = crate::Newtonsoft::Json::Serialization::DefaultContractResolver_EnumerableDictionaryWrapper_2<
        TEnumeratorKey,
        TEnumeratorValue,
    >;
    pub fn CanConvertToString(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CanConvertToString", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateArrayContract(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonArrayContract,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonArrayContract,
        > = __cordl_object.invoke("CreateArrayContract", (objectType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateConstructorParameters(
        &mut self,
        constructor: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ConstructorInfo,
        >,
        memberProperties: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonPropertyCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Newtonsoft::Json::Serialization::JsonProperty,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Newtonsoft::Json::Serialization::JsonProperty,
            >,
        > = __cordl_object
            .invoke("CreateConstructorParameters", (constructor, memberProperties))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateContract(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::JsonContract>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        > = __cordl_object.invoke("CreateContract", (objectType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDictionaryContract(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonDictionaryContract,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonDictionaryContract,
        > = __cordl_object.invoke("CreateDictionaryContract", (objectType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDynamicContract(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonDynamicContract,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonDynamicContract,
        > = __cordl_object.invoke("CreateDynamicContract", (objectType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateISerializableContract(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonISerializableContract,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonISerializableContract,
        > = __cordl_object.invoke("CreateISerializableContract", (objectType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateLinqContract(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonLinqContract,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonLinqContract,
        > = __cordl_object.invoke("CreateLinqContract", (objectType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateMemberValueProvider(
        &mut self,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::IValueProvider>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::IValueProvider,
        > = __cordl_object.invoke("CreateMemberValueProvider", (member))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateObjectContract(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonObjectContract,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonObjectContract,
        > = __cordl_object.invoke("CreateObjectContract", (objectType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePrimitiveContract(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonPrimitiveContract,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonPrimitiveContract,
        > = __cordl_object.invoke("CreatePrimitiveContract", (objectType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateProperties(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        memberSerialization: crate::Newtonsoft::Json::MemberSerialization,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Newtonsoft::Json::Serialization::JsonProperty,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Newtonsoft::Json::Serialization::JsonProperty,
            >,
        > = __cordl_object
            .invoke("CreateProperties", (_cordl_type, memberSerialization))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateProperty(
        &mut self,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        memberSerialization: crate::Newtonsoft::Json::MemberSerialization,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::JsonProperty>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        > = __cordl_object.invoke("CreateProperty", (member, memberSerialization))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePropertyFromConstructorParameter(
        &mut self,
        matchingMemberProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        parameterInfo: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ParameterInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::JsonProperty>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        > = __cordl_object
            .invoke(
                "CreatePropertyFromConstructorParameter",
                (matchingMemberProperty, parameterInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateShouldSerializeTest(
        &mut self,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        > = __cordl_object.invoke("CreateShouldSerializeTest", (member))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateStringContract(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonStringContract,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonStringContract,
        > = __cordl_object.invoke("CreateStringContract", (objectType))?;
        Ok(__cordl_ret.into())
    }
    pub fn FilterMembers(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FilterMembers", (member))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributeConstructor(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ConstructorInfo,
        > = __cordl_object.invoke("GetAttributeConstructor", (objectType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCallbackMethodsForType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        onSerializing: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Serialization::SerializationCallback,
                >,
            >,
        >,
        onSerialized: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Serialization::SerializationCallback,
                >,
            >,
        >,
        onDeserializing: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Serialization::SerializationCallback,
                >,
            >,
        >,
        onDeserialized: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Serialization::SerializationCallback,
                >,
            >,
        >,
        onError: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Serialization::SerializationErrorCallback,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetCallbackMethodsForType",
                (
                    _cordl_type,
                    onSerializing,
                    onSerialized,
                    onDeserializing,
                    onDeserialized,
                    onError,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClassHierarchyForType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Gc<crate::System::Type>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        > = __cordl_object.invoke("GetClassHierarchyForType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetClrTypeFullName(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetClrTypeFullName", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultCreator(
        &mut self,
        createdType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        > = __cordl_object.invoke("GetDefaultCreator", (createdType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetExtensionDataMemberForType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MemberInfo,
        > = __cordl_object.invoke("GetExtensionDataMemberForType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetImmutableConstructor(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        memberProperties: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonPropertyCollection,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ConstructorInfo,
        > = __cordl_object
            .invoke("GetImmutableConstructor", (objectType, memberProperties))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNameTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::DefaultJsonNameTable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::DefaultJsonNameTable,
        > = __cordl_object.invoke("GetNameTable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParameterizedConstructor(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ConstructorInfo,
        > = __cordl_object.invoke("GetParameterizedConstructor", (objectType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResolvedPropertyName(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetResolvedPropertyName", (propertyName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSerializableMembers(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        > = __cordl_object.invoke("GetSerializableMembers", (objectType))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitializeContract(
        &mut self,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeContract", (contract))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsConcurrentOrObservableCollection(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsConcurrentOrObservableCollection", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsIConvertible(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsIConvertible", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsJsonPrimitiveType(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsJsonPrimitiveType", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidCallback(
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        parameters: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
            >,
        >,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        currentCallback: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        >,
        prevAttributeType: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "IsValidCallback",
                (method, parameters, attributeType, currentCallback, prevAttributeType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn MatchProperty(
        &mut self,
        properties: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonPropertyCollection,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::JsonProperty>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        > = __cordl_object.invoke("MatchProperty", (properties, name, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ResolveCallbackMethods(
        &mut self,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResolveCallbackMethods", (contract, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveContract(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::JsonContract>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        > = __cordl_object.invoke("ResolveContract", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveContractConverter(
        &mut self,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonConverter>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::JsonConverter,
        > = __cordl_object.invoke("ResolveContractConverter", (objectType))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveDictionaryKey(
        &mut self,
        dictionaryKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ResolveDictionaryKey", (dictionaryKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveExtensionDataName(
        &mut self,
        extensionDataName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ResolveExtensionDataName", (extensionDataName))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolvePropertyName(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ResolvePropertyName", (propertyName))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetExtensionDataDelegates(
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonObjectContract,
        >,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetExtensionDataDelegates", (contract, member))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIsSpecifiedActions(
        &mut self,
        property: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        allowNonPublicAccess: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetIsSpecifiedActions", (property, member, allowNonPublicAccess))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPropertySettingsFromAttributes(
        &mut self,
        property: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        attributeProvider: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        declaringType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        memberSerialization: crate::Newtonsoft::Json::MemberSerialization,
        allowNonPublicAccess: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetPropertySettingsFromAttributes",
                (
                    property,
                    attributeProvider,
                    name,
                    declaringType,
                    memberSerialization,
                    allowNonPublicAccess,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldSerializeEntityMember(
        &mut self,
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldSerializeEntityMember", (memberInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldSkipDeserialized(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldSkipDeserialized", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldSkipSerializing(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldSkipSerializing", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn ThrowUnableToSerializeError(
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThrowUnableToSerializeError", (o, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DefaultMembersSearchFlags(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Reflection::BindingFlags> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Reflection::BindingFlags = __cordl_object
            .invoke("get_DefaultMembersSearchFlags", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_DynamicCodeGeneration(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_DynamicCodeGeneration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IgnoreIsSpecifiedMembers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IgnoreIsSpecifiedMembers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IgnoreSerializableAttribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IgnoreSerializableAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IgnoreSerializableInterface(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IgnoreSerializableInterface", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IgnoreShouldSerializeMembers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_IgnoreShouldSerializeMembers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::IContractResolver,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::IContractResolver,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Instance", ())?;
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
    pub fn get_SerializeCompilerGeneratedMembers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_SerializeCompilerGeneratedMembers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_DefaultMembersSearchFlags(
        &mut self,
        value: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DefaultMembersSearchFlags", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IgnoreIsSpecifiedMembers(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IgnoreIsSpecifiedMembers", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IgnoreSerializableAttribute(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IgnoreSerializableAttribute", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IgnoreSerializableInterface(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IgnoreSerializableInterface", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IgnoreShouldSerializeMembers(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IgnoreShouldSerializeMembers", (value))?;
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
    pub fn set_SerializeCompilerGeneratedMembers(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SerializeCompilerGeneratedMembers", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultContractResolver")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::DefaultContractResolver {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultContractResolver")]
impl AsRef<
    quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::IContractResolver>,
> for crate::Newtonsoft::Json::Serialization::DefaultContractResolver {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::IContractResolver,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+DefaultContractResolver")]
impl AsMut<
    quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::IContractResolver>,
> for crate::Newtonsoft::Json::Serialization::DefaultContractResolver {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::IContractResolver,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+DefaultContractResolver+EnumerableDictionaryWrapper_2"
)]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultContractResolver_EnumerableDictionaryWrapper_2<
    TEnumeratorKey: quest_hook::libil2cpp::Type,
    TEnumeratorValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _e: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::KeyValuePair_2<
            TEnumeratorKey,
            TEnumeratorValue,
        >,
    >,
    __cordl_phantom_TEnumeratorKey: std::marker::PhantomData<TEnumeratorKey>,
    __cordl_phantom_TEnumeratorValue: std::marker::PhantomData<TEnumeratorValue>,
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+DefaultContractResolver+EnumerableDictionaryWrapper_2"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::DefaultContractResolver_EnumerableDictionaryWrapper_2
    < TEnumeratorKey, TEnumeratorValue > => "Newtonsoft.Json.Serialization"
    ."DefaultContractResolver/EnumerableDictionaryWrapper`2" < TEnumeratorKey,
    TEnumeratorValue >
);
#[cfg(
    feature = "Newtonsoft+Json+Serialization+DefaultContractResolver+EnumerableDictionaryWrapper_2"
)]
impl<
    TEnumeratorKey: quest_hook::libil2cpp::Type,
    TEnumeratorValue: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Newtonsoft::Json::Serialization::DefaultContractResolver_EnumerableDictionaryWrapper_2<
    TEnumeratorKey,
    TEnumeratorValue,
> {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+DefaultContractResolver+EnumerableDictionaryWrapper_2"
)]
impl<
    TEnumeratorKey: quest_hook::libil2cpp::Type,
    TEnumeratorValue: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::DefaultContractResolver_EnumerableDictionaryWrapper_2<
    TEnumeratorKey,
    TEnumeratorValue,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+DefaultContractResolver+EnumerableDictionaryWrapper_2"
)]
impl<
    TEnumeratorKey: quest_hook::libil2cpp::Type,
    TEnumeratorValue: quest_hook::libil2cpp::Type,
> crate::Newtonsoft::Json::Serialization::DefaultContractResolver_EnumerableDictionaryWrapper_2<
    TEnumeratorKey,
    TEnumeratorValue,
> {
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::KeyValuePair_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    >
    where
        TEnumeratorKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TEnumeratorValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::KeyValuePair_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        e: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::KeyValuePair_2<
                TEnumeratorKey,
                TEnumeratorValue,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TEnumeratorKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TEnumeratorValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (e))?;
        Ok(__cordl_object.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    >
    where
        TEnumeratorKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TEnumeratorValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        e: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::KeyValuePair_2<
                TEnumeratorKey,
                TEnumeratorValue,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TEnumeratorKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TEnumeratorValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (e))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+DefaultContractResolver+EnumerableDictionaryWrapper_2"
)]
impl<
    TEnumeratorKey: quest_hook::libil2cpp::Type,
    TEnumeratorValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::DefaultContractResolver_EnumerableDictionaryWrapper_2<
    TEnumeratorKey,
    TEnumeratorValue,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+DefaultContractResolver+EnumerableDictionaryWrapper_2"
)]
impl<
    TEnumeratorKey: quest_hook::libil2cpp::Type,
    TEnumeratorValue: quest_hook::libil2cpp::Type,
> AsRef<
    quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::KeyValuePair_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    >,
>
for crate::Newtonsoft::Json::Serialization::DefaultContractResolver_EnumerableDictionaryWrapper_2<
    TEnumeratorKey,
    TEnumeratorValue,
> {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::KeyValuePair_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+DefaultContractResolver+EnumerableDictionaryWrapper_2"
)]
impl<
    TEnumeratorKey: quest_hook::libil2cpp::Type,
    TEnumeratorValue: quest_hook::libil2cpp::Type,
> AsMut<
    quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::KeyValuePair_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    >,
>
for crate::Newtonsoft::Json::Serialization::DefaultContractResolver_EnumerableDictionaryWrapper_2<
    TEnumeratorKey,
    TEnumeratorValue,
> {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::KeyValuePair_2<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+DefaultContractResolver+EnumerableDictionaryWrapper_2"
)]
impl<
    TEnumeratorKey: quest_hook::libil2cpp::Type,
    TEnumeratorValue: quest_hook::libil2cpp::Type,
> AsRef<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>>
for crate::Newtonsoft::Json::Serialization::DefaultContractResolver_EnumerableDictionaryWrapper_2<
    TEnumeratorKey,
    TEnumeratorValue,
> {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+DefaultContractResolver+EnumerableDictionaryWrapper_2"
)]
impl<
    TEnumeratorKey: quest_hook::libil2cpp::Type,
    TEnumeratorValue: quest_hook::libil2cpp::Type,
> AsMut<quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>>
for crate::Newtonsoft::Json::Serialization::DefaultContractResolver_EnumerableDictionaryWrapper_2<
    TEnumeratorKey,
    TEnumeratorValue,
> {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable> {
        unsafe { std::mem::transmute(self) }
    }
}
