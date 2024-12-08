#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalWriter")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSerializerInternalWriter {
    __cordl_parent: crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase,
    pub _rootType: *mut crate::System::Type,
    pub _rootLevel: i32,
    pub _serializeStack: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Object,
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
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        value: *mut crate::System::Object,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        property: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        memberContract: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        memberValue: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CalculatePropertyValues",
                (writer, value, contract, member, property, memberContract, memberValue),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CheckForCircularReference(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        value: *mut crate::System::Object,
        property: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        containerContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CheckForCircularReference",
                (writer, value, property, contract, containerContract, containerProperty),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetContract(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::JsonContract,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::JsonContract = __cordl_object
            .invoke("GetContract", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetContractSafe(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::JsonContract,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::JsonContract = __cordl_object
            .invoke("GetContractSafe", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetInternalSerializer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::JsonSerializerProxy,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::JsonSerializerProxy = __cordl_object
            .invoke("GetInternalSerializer", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPropertyName(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        name: *mut crate::System::Object,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        escape: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetPropertyName", (writer, name, contract, escape))?;
        Ok(__cordl_ret)
    }
    pub fn GetReference(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetReference", (writer, value))?;
        Ok(__cordl_ret)
    }
    pub fn HandleError(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        initialDepth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleError", (writer, initialDepth))?;
        Ok(__cordl_ret)
    }
    pub fn HasCreatorParameter(
        &mut self,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        property: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("HasCreatorParameter", (contract, property))?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn IsSpecified(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        property: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        target: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsSpecified", (writer, property, target))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        serializer: *mut crate::Newtonsoft::Json::JsonSerializer,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (serializer))?;
        Ok(__cordl_object)
    }
    pub fn OnSerialized(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSerialized", (writer, contract, value))?;
        Ok(__cordl_ret)
    }
    pub fn OnSerializing(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSerializing", (writer, contract, value))?;
        Ok(__cordl_ret)
    }
    pub fn ResolveIsReference(
        &mut self,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        property: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        collectionContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<bool> = __cordl_object
            .invoke(
                "ResolveIsReference",
                (contract, property, collectionContract, containerProperty),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Serialize(
        &mut self,
        jsonWriter: *mut crate::Newtonsoft::Json::JsonWriter,
        value: *mut crate::System::Object,
        objectType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (jsonWriter, value, objectType))?;
        Ok(__cordl_ret)
    }
    pub fn SerializeConvertable(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        converter: *mut crate::Newtonsoft::Json::JsonConverter,
        value: *mut crate::System::Object,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        collectionContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
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
        Ok(__cordl_ret)
    }
    pub fn SerializeDictionary(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        values: *mut crate::System::Collections::IDictionary,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonDictionaryContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        collectionContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SerializeDictionary",
                (writer, values, contract, member, collectionContract, containerProperty),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SerializeDynamic(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        value: *mut crate::System::Dynamic::IDynamicMetaObjectProvider,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonDynamicContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        collectionContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SerializeDynamic",
                (writer, value, contract, member, collectionContract, containerProperty),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SerializeISerializable(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        value: *mut crate::System::Runtime::Serialization::ISerializable,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonISerializableContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        collectionContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SerializeISerializable",
                (writer, value, contract, member, collectionContract, containerProperty),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SerializeList(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        values: *mut crate::System::Collections::IEnumerable,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonArrayContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        collectionContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SerializeList",
                (writer, values, contract, member, collectionContract, containerProperty),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SerializeMultidimensionalArray_JsonContainerContract_JsonProperty0(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        values: *mut crate::System::Array,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonArrayContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        collectionContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SerializeMultidimensionalArray",
                (writer, values, contract, member, collectionContract, containerProperty),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SerializeMultidimensionalArray_i32_Il2CppArray1(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        values: *mut crate::System::Array,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonArrayContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        initialDepth: i32,
        indices: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SerializeMultidimensionalArray",
                (writer, values, contract, member, initialDepth, indices),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SerializeObject(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        value: *mut crate::System::Object,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonObjectContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        collectionContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SerializeObject",
                (writer, value, contract, member, collectionContract, containerProperty),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SerializePrimitive(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        value: *mut crate::System::Object,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonPrimitiveContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        containerContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SerializePrimitive",
                (writer, value, contract, member, containerContract, containerProperty),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SerializeString(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        value: *mut crate::System::Object,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonStringContract,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SerializeString", (writer, value, contract))?;
        Ok(__cordl_ret)
    }
    pub fn SerializeValue(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        value: *mut crate::System::Object,
        valueContract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        containerContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
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
        Ok(__cordl_ret)
    }
    pub fn ShouldSerialize(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        property: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        target: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldSerialize", (writer, property, target))?;
        Ok(__cordl_ret)
    }
    pub fn ShouldWriteDynamicProperty(
        &mut self,
        memberValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldWriteDynamicProperty", (memberValue))?;
        Ok(__cordl_ret)
    }
    pub fn ShouldWriteProperty(
        &mut self,
        memberValue: *mut crate::System::Object,
        containerContract: *mut crate::Newtonsoft::Json::Serialization::JsonObjectContract,
        property: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldWriteProperty", (memberValue, containerContract, property))?;
        Ok(__cordl_ret)
    }
    pub fn ShouldWriteReference(
        &mut self,
        value: *mut crate::System::Object,
        property: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        valueContract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        collectionContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ShouldWriteReference",
                (value, property, valueContract, collectionContract, containerProperty),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ShouldWriteType(
        &mut self,
        typeNameHandlingFlag: crate::Newtonsoft::Json::TypeNameHandling,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        containerContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
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
        Ok(__cordl_ret)
    }
    pub fn WriteObjectStart(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        value: *mut crate::System::Object,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        collectionContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "WriteObjectStart",
                (writer, value, contract, member, collectionContract, containerProperty),
            )?;
        Ok(__cordl_ret)
    }
    pub fn WriteReference(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteReference", (writer, value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteReferenceIdProperty(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        _cordl_type: *mut crate::System::Type,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteReferenceIdProperty", (writer, _cordl_type, value))?;
        Ok(__cordl_ret)
    }
    pub fn WriteStartArray(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        values: *mut crate::System::Object,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonArrayContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        containerContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "WriteStartArray",
                (writer, values, contract, member, containerContract, containerProperty),
            )?;
        Ok(__cordl_ret)
    }
    pub fn WriteTypeProperty(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteTypeProperty", (writer, _cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        serializer: *mut crate::Newtonsoft::Json::JsonSerializer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (serializer))?;
        Ok(__cordl_ret)
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
