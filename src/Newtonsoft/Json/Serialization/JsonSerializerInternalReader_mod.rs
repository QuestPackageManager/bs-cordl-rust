#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalReader+CreatorPropertyContext"
)]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSerializerInternalReader_CreatorPropertyContext {
    __cordl_parent: crate::System::Object,
    pub Name: *mut crate::System::String,
    pub Property: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    pub ConstructorProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    pub Presence: crate::System::Nullable_1<
        crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader_PropertyPresence,
    >,
    pub Value: *mut crate::System::Object,
    pub Used: bool,
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalReader+CreatorPropertyContext"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::JsonSerializerInternalReader_CreatorPropertyContext
    => "Newtonsoft.Json.Serialization"
    ."JsonSerializerInternalReader/CreatorPropertyContext"
);
#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalReader+CreatorPropertyContext"
)]
impl std::ops::Deref
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader_CreatorPropertyContext {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalReader+CreatorPropertyContext"
)]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader_CreatorPropertyContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalReader+CreatorPropertyContext"
)]
impl crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader_CreatorPropertyContext {
    pub fn New(
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalReader+CreatorPropertyContext"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader_CreatorPropertyContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalReader")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSerializerInternalReader {
    __cordl_parent: crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::JsonSerializerInternalReader =>
    "Newtonsoft.Json.Serialization"."JsonSerializerInternalReader"
);
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalReader")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader {
    type Target = crate::Newtonsoft::Json::Serialization::JsonSerializerInternalBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalReader")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalReader")]
impl crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader {
    #[cfg(
        feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalReader+CreatorPropertyContext"
    )]
    pub type CreatorPropertyContext = crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader_CreatorPropertyContext;
    #[cfg(
        feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalReader+PropertyPresence"
    )]
    pub type PropertyPresence = crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader_PropertyPresence;
    #[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalReader+__c")]
    pub type __c = crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader___c;
    #[cfg(
        feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalReader+__c__DisplayClass38_0"
    )]
    pub type __c__DisplayClass38_0 = crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader___c__DisplayClass38_0;
    pub fn AddReference(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        id: *mut crate::System::String,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddReference", (reader, id, value))?;
        Ok(__cordl_ret)
    }
    pub fn CalculatePropertyDetails(
        &mut self,
        property: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        propertyConverter: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Newtonsoft::Json::JsonConverter,
        >,
        containerContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        target: *mut crate::System::Object,
        useExistingValue: quest_hook::libil2cpp::ByRefMut<bool>,
        currentValue: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Object>,
        propertyContract: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        gottenCurrentValue: quest_hook::libil2cpp::ByRefMut<bool>,
        ignoredValue: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CalculatePropertyDetails",
                (
                    property,
                    propertyConverter,
                    containerContract,
                    containerProperty,
                    reader,
                    target,
                    useExistingValue,
                    currentValue,
                    propertyContract,
                    gottenCurrentValue,
                    ignoredValue,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CheckPropertyName(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        memberName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckPropertyName", (reader, memberName))?;
        Ok(__cordl_ret)
    }
    pub fn CreateDynamic(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonDynamicContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("CreateDynamic", (reader, contract, member, id))?;
        Ok(__cordl_ret)
    }
    pub fn CreateISerializable(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonISerializableContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("CreateISerializable", (reader, contract, member, id))?;
        Ok(__cordl_ret)
    }
    pub fn CreateISerializableItem(
        &mut self,
        token: *mut crate::Newtonsoft::Json::Linq::JToken,
        _cordl_type: *mut crate::System::Type,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonISerializableContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("CreateISerializableItem", (token, _cordl_type, contract, member))?;
        Ok(__cordl_ret)
    }
    pub fn CreateJObject(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Newtonsoft::Json::Linq::JToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Linq::JToken = __cordl_object
            .invoke("CreateJObject", (reader))?;
        Ok(__cordl_ret)
    }
    pub fn CreateJToken(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Newtonsoft::Json::Linq::JToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Linq::JToken = __cordl_object
            .invoke("CreateJToken", (reader, contract))?;
        Ok(__cordl_ret)
    }
    pub fn CreateList(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        objectType: *mut crate::System::Type,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        existingValue: *mut crate::System::Object,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "CreateList",
                (reader, objectType, contract, member, existingValue, id),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateNewDictionary(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonDictionaryContract,
        createdFromNonDefaultCreator: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IDictionary> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IDictionary = __cordl_object
            .invoke(
                "CreateNewDictionary",
                (reader, contract, createdFromNonDefaultCreator),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateNewList(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonArrayContract,
        createdFromNonDefaultCreator: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IList = __cordl_object
            .invoke("CreateNewList", (reader, contract, createdFromNonDefaultCreator))?;
        Ok(__cordl_ret)
    }
    pub fn CreateNewObject(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        objectContract: *mut crate::Newtonsoft::Json::Serialization::JsonObjectContract,
        containerMember: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        id: *mut crate::System::String,
        createdFromNonDefaultCreator: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "CreateNewObject",
                (
                    reader,
                    objectContract,
                    containerMember,
                    containerProperty,
                    id,
                    createdFromNonDefaultCreator,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateObject(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        objectType: *mut crate::System::Type,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        containerContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerMember: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        existingValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "CreateObject",
                (
                    reader,
                    objectType,
                    contract,
                    member,
                    containerContract,
                    containerMember,
                    existingValue,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateObjectUsingCreatorWithParameters(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonObjectContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        creator: *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            *mut crate::System::Object,
        >,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "CreateObjectUsingCreatorWithParameters",
                (reader, contract, containerProperty, creator, id),
            )?;
        Ok(__cordl_ret)
    }
    pub fn CreateValueInternal(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        objectType: *mut crate::System::Type,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        containerContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerMember: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        existingValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "CreateValueInternal",
                (
                    reader,
                    objectType,
                    contract,
                    member,
                    containerContract,
                    containerMember,
                    existingValue,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Deserialize(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        objectType: *mut crate::System::Type,
        checkAdditionalContent: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("Deserialize", (reader, objectType, checkAdditionalContent))?;
        Ok(__cordl_ret)
    }
    pub fn DeserializeConvertable(
        &mut self,
        converter: *mut crate::Newtonsoft::Json::JsonConverter,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        objectType: *mut crate::System::Type,
        existingValue: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "DeserializeConvertable",
                (converter, reader, objectType, existingValue),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndProcessProperty(
        &mut self,
        newObject: *mut crate::System::Object,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonObjectContract,
        initialDepth: i32,
        property: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        presence: crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader_PropertyPresence,
        setDefaultValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "EndProcessProperty",
                (
                    newObject,
                    reader,
                    contract,
                    initialDepth,
                    property,
                    presence,
                    setDefaultValue,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EnsureArrayContract(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        objectType: *mut crate::System::Type,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::JsonArrayContract,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::JsonArrayContract = __cordl_object
            .invoke("EnsureArrayContract", (reader, objectType, contract))?;
        Ok(__cordl_ret)
    }
    pub fn EnsureType(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        value: *mut crate::System::Object,
        culture: *mut crate::System::Globalization::CultureInfo,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        targetType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("EnsureType", (reader, value, culture, contract, targetType))?;
        Ok(__cordl_ret)
    }
    pub fn GetContract(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::JsonContract,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::JsonContract = __cordl_object
            .invoke("GetContract", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetContractSafe(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::JsonContract,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::JsonContract = __cordl_object
            .invoke("GetContractSafe", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetConverter(
        &mut self,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        memberConverter: *mut crate::Newtonsoft::Json::JsonConverter,
        containerContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Newtonsoft::Json::JsonConverter> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::JsonConverter = __cordl_object
            .invoke(
                "GetConverter",
                (contract, memberConverter, containerContract, containerProperty),
            )?;
        Ok(__cordl_ret)
    }
    pub fn GetExpectedDescription(
        &mut self,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetExpectedDescription", (contract))?;
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
    pub fn HandleError(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        readPastError: bool,
        initialDepth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleError", (reader, readPastError, initialDepth))?;
        Ok(__cordl_ret)
    }
    pub fn HasFlag(
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
    pub fn HasNoDefinedType(
        &mut self,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasNoDefinedType", (contract))?;
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
    pub fn OnDeserialized(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeserialized", (reader, contract, value))?;
        Ok(__cordl_ret)
    }
    pub fn OnDeserializing(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeserializing", (reader, contract, value))?;
        Ok(__cordl_ret)
    }
    pub fn Populate(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        target: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Populate", (reader, target))?;
        Ok(__cordl_ret)
    }
    pub fn PopulateDictionary(
        &mut self,
        dictionary: *mut crate::System::Collections::IDictionary,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonDictionaryContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "PopulateDictionary",
                (dictionary, reader, contract, containerProperty, id),
            )?;
        Ok(__cordl_ret)
    }
    pub fn PopulateList(
        &mut self,
        list: *mut crate::System::Collections::IList,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonArrayContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("PopulateList", (list, reader, contract, containerProperty, id))?;
        Ok(__cordl_ret)
    }
    pub fn PopulateMultidimensionalArray(
        &mut self,
        list: *mut crate::System::Collections::IList,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonArrayContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "PopulateMultidimensionalArray",
                (list, reader, contract, containerProperty, id),
            )?;
        Ok(__cordl_ret)
    }
    pub fn PopulateObject(
        &mut self,
        newObject: *mut crate::System::Object,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonObjectContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        id: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("PopulateObject", (newObject, reader, contract, member, id))?;
        Ok(__cordl_ret)
    }
    pub fn ReadExtensionDataValue(
        &mut self,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonObjectContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("ReadExtensionDataValue", (contract, member, reader))?;
        Ok(__cordl_ret)
    }
    pub fn ReadMetadataProperties(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        objectType: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Type>,
        contract: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        containerContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerMember: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        existingValue: *mut crate::System::Object,
        newValue: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Object>,
        id: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ReadMetadataProperties",
                (
                    reader,
                    objectType,
                    contract,
                    member,
                    containerContract,
                    containerMember,
                    existingValue,
                    newValue,
                    id,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ReadMetadataPropertiesToken(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::Linq::JTokenReader,
        objectType: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Type>,
        contract: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        containerContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerMember: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        existingValue: *mut crate::System::Object,
        newValue: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Object>,
        id: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "ReadMetadataPropertiesToken",
                (
                    reader,
                    objectType,
                    contract,
                    member,
                    containerContract,
                    containerMember,
                    existingValue,
                    newValue,
                    id,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ResolvePropertyAndCreatorValues(
        &mut self,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonObjectContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        objectType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader_CreatorPropertyContext,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader_CreatorPropertyContext,
        > = __cordl_object
            .invoke(
                "ResolvePropertyAndCreatorValues",
                (contract, containerProperty, reader, objectType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ResolveTypeName(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        objectType: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Type>,
        contract: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        >,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        containerContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerMember: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        qualifiedTypeName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ResolveTypeName",
                (
                    reader,
                    objectType,
                    contract,
                    member,
                    containerContract,
                    containerMember,
                    qualifiedTypeName,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SetExtensionData(
        &mut self,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonObjectContract,
        member: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        memberName: *mut crate::System::String,
        o: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetExtensionData", (contract, member, reader, memberName, o))?;
        Ok(__cordl_ret)
    }
    pub fn SetPropertyPresence(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        property: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        requiredProperties: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
            crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader_PropertyPresence,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPropertyPresence", (reader, property, requiredProperties))?;
        Ok(__cordl_ret)
    }
    pub fn SetPropertyValue(
        &mut self,
        property: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        propertyConverter: *mut crate::Newtonsoft::Json::JsonConverter,
        containerContract: *mut crate::Newtonsoft::Json::Serialization::JsonContainerContract,
        containerProperty: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        target: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "SetPropertyValue",
                (
                    property,
                    propertyConverter,
                    containerContract,
                    containerProperty,
                    reader,
                    target,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ShouldDeserialize(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        property: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        target: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldDeserialize", (reader, property, target))?;
        Ok(__cordl_ret)
    }
    pub fn ShouldSetPropertyValue(
        &mut self,
        property: *mut crate::Newtonsoft::Json::Serialization::JsonProperty,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonObjectContract,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldSetPropertyValue", (property, contract, value))?;
        Ok(__cordl_ret)
    }
    pub fn ThrowUnexpectedEndException(
        &mut self,
        reader: *mut crate::Newtonsoft::Json::JsonReader,
        contract: *mut crate::Newtonsoft::Json::Serialization::JsonContract,
        currentObject: *mut crate::System::Object,
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ThrowUnexpectedEndException",
                (reader, contract, currentObject, message),
            )?;
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
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalReader")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonSerializerInternalReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalReader+PropertyPresence"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonSerializerInternalReader_PropertyPresence {
    None = 0i32,
    Null = 1i32,
    Value = 2i32,
}
#[cfg(
    feature = "Newtonsoft+Json+Serialization+JsonSerializerInternalReader+PropertyPresence"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Serialization::JsonSerializerInternalReader_PropertyPresence =>
    "Newtonsoft.Json.Serialization"."JsonSerializerInternalReader/PropertyPresence"
);
