#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSerializerInternalWriter {
    __cordl_parent: crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase,
    pub _rootType: *mut crate::System::Type,
    pub _rootLevel: i32,
    pub _serializeStack: *mut crate::System::Collections::Generic::List_1<
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalWriter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::JsonSerializerInternalWriter =>
    "Newtonsoft.Json.Serialization"."JsonSerializerInternalWriter"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalWriter")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalWriter {
    type Target = crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalWriter")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalWriter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalWriter")]
impl crate::Newtonsoft::Json::Serialization::JsonSerializerInternalWriter {
    pub fn CalculatePropertyValues(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        property: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        memberContract: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        memberValue: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CalculatePropertyValues",
                (writer, value, contract, member, property, memberContract, memberValue),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckForCircularReference(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        property: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        containerContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CheckForCircularReference",
                (writer, value, property, contract, containerContract, containerProperty),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContract(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::JsonContract>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        > = __cordl_object.invoke("GetContract", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContractSafe(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Serialization::JsonContract>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        > = __cordl_object.invoke("GetContractSafe", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInternalSerializer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonSerializerProxy,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonSerializerProxy,
        > = __cordl_object.invoke("GetInternalSerializer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyName(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        escape: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetPropertyName", (writer, name, contract, escape))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReference(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetReference", (writer, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn HandleError(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        initialDepth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleError", (writer, initialDepth))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasCreatorParameter(
        &mut self,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        property: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasCreatorParameter", (contract, property))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasFlag_DefaultValueHandling_DefaultValueHandling0(
        &mut self,
        value: crate::Newtonsoft::Json::DefaultValueHandling,
        flag: crate::Newtonsoft::Json::DefaultValueHandling,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasFlag", (value, flag))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasFlag_PreserveReferencesHandling_PreserveReferencesHandling1(
        &mut self,
        value: crate::Newtonsoft::Json::PreserveReferencesHandling,
        flag: crate::Newtonsoft::Json::PreserveReferencesHandling,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasFlag", (value, flag))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasFlag_TypeNameHandling_TypeNameHandling2(
        &mut self,
        value: crate::Newtonsoft::Json::TypeNameHandling,
        flag: crate::Newtonsoft::Json::TypeNameHandling,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasFlag", (value, flag))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSpecified(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        property: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsSpecified", (writer, property, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        serializer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serializer))?;
        Ok(__cordl_object.into())
    }
    pub fn OnSerialized(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSerialized", (writer, contract, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSerializing(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSerializing", (writer, contract, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ResolveIsReference(
        &mut self,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        property: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<bool> = __cordl_object
            .invoke(
                "ResolveIsReference",
                (contract, property, collectionContract, containerProperty),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        jsonWriter: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        objectType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (jsonWriter, value, objectType))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeConvertable(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        converter: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonConverter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SerializeConvertable",
                (
                    writer,
                    converter,
                    value,
                    contract,
                    collectionContract,
                    containerProperty,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeDictionary(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        values: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonDictionaryContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SerializeDictionary",
                (writer, values, contract, member, collectionContract, containerProperty),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeDynamic(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::IDynamicMetaObjectProvider,
        >,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonDynamicContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SerializeDynamic",
                (writer, value, contract, member, collectionContract, containerProperty),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeISerializable(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::ISerializable,
        >,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonISerializableContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SerializeISerializable",
                (writer, value, contract, member, collectionContract, containerProperty),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeList(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        values: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerable>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonArrayContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SerializeList",
                (writer, values, contract, member, collectionContract, containerProperty),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeMultidimensionalArray_JsonContainerContract_JsonProperty0(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        values: quest_hook::libil2cpp::Gc<crate::System::Array>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonArrayContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SerializeMultidimensionalArray",
                (writer, values, contract, member, collectionContract, containerProperty),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeMultidimensionalArray_i32_Il2CppArray1(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        values: quest_hook::libil2cpp::Gc<crate::System::Array>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonArrayContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        initialDepth: i32,
        indices: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SerializeMultidimensionalArray",
                (writer, values, contract, member, initialDepth, indices),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeObject(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonObjectContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SerializeObject",
                (writer, value, contract, member, collectionContract, containerProperty),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializePrimitive(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonPrimitiveContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        containerContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SerializePrimitive",
                (writer, value, contract, member, containerContract, containerProperty),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeString(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonStringContract,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SerializeString", (writer, value, contract))?;
        Ok(__cordl_ret.into())
    }
    pub fn SerializeValue(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        valueContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        containerContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SerializeValue",
                (
                    writer,
                    value,
                    valueContract,
                    member,
                    containerContract,
                    containerProperty,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldSerialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        property: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldSerialize", (writer, property, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldWriteDynamicProperty(
        &mut self,
        memberValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldWriteDynamicProperty", (memberValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldWriteProperty(
        &mut self,
        memberValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        containerContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonObjectContract,
        >,
        property: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldWriteProperty", (memberValue, containerContract, property))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldWriteReference(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        property: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        valueContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ShouldWriteReference",
                (value, property, valueContract, collectionContract, containerProperty),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldWriteType(
        &mut self,
        typeNameHandlingFlag: crate::Newtonsoft::Json::TypeNameHandling,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        containerContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ShouldWriteType",
                (
                    typeNameHandlingFlag,
                    contract,
                    member,
                    containerContract,
                    containerProperty,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryConvertToString(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        s: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryConvertToString", (value, _cordl_type, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteObjectStart(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        collectionContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteObjectStart",
                (writer, value, contract, member, collectionContract, containerProperty),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteReference(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteReference", (writer, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteReferenceIdProperty(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteReferenceIdProperty", (writer, _cordl_type, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteStartArray(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        contract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonArrayContract,
        >,
        member: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
        containerContract: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        >,
        containerProperty: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::JsonProperty,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "WriteStartArray",
                (writer, values, contract, member, containerContract, containerProperty),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteTypeProperty(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonWriter>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteTypeProperty", (writer, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        serializer: quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::JsonSerializer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (serializer))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalWriter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalWriter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
