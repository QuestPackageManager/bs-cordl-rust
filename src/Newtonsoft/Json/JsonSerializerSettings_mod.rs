#[cfg(feature = "Newtonsoft+Json+JsonSerializerSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonSerializerSettings {
    __cordl_parent: crate::System::Object,
    pub _formatting: crate::System::Nullable_1<crate::Newtonsoft::Json::Formatting>,
    pub _dateFormatHandling: crate::System::Nullable_1<
        crate::Newtonsoft::Json::DateFormatHandling,
    >,
    pub _dateTimeZoneHandling: crate::System::Nullable_1<
        crate::Newtonsoft::Json::DateTimeZoneHandling,
    >,
    pub _dateParseHandling: crate::System::Nullable_1<
        crate::Newtonsoft::Json::DateParseHandling,
    >,
    pub _floatFormatHandling: crate::System::Nullable_1<
        crate::Newtonsoft::Json::FloatFormatHandling,
    >,
    pub _floatParseHandling: crate::System::Nullable_1<
        crate::Newtonsoft::Json::FloatParseHandling,
    >,
    pub _stringEscapeHandling: crate::System::Nullable_1<
        crate::Newtonsoft::Json::StringEscapeHandling,
    >,
    pub _culture: *mut crate::System::Globalization::CultureInfo,
    pub _checkAdditionalContent: crate::System::Nullable_1<bool>,
    pub _maxDepth: crate::System::Nullable_1<i32>,
    pub _maxDepthSet: bool,
    pub _dateFormatString: *mut crate::System::String,
    pub _dateFormatStringSet: bool,
    pub _typeNameAssemblyFormatHandling: crate::System::Nullable_1<
        crate::Newtonsoft::Json::TypeNameAssemblyFormatHandling,
    >,
    pub _defaultValueHandling: crate::System::Nullable_1<
        crate::Newtonsoft::Json::DefaultValueHandling,
    >,
    pub _preserveReferencesHandling: crate::System::Nullable_1<
        crate::Newtonsoft::Json::PreserveReferencesHandling,
    >,
    pub _nullValueHandling: crate::System::Nullable_1<
        crate::Newtonsoft::Json::NullValueHandling,
    >,
    pub _objectCreationHandling: crate::System::Nullable_1<
        crate::Newtonsoft::Json::ObjectCreationHandling,
    >,
    pub _missingMemberHandling: crate::System::Nullable_1<
        crate::Newtonsoft::Json::MissingMemberHandling,
    >,
    pub _referenceLoopHandling: crate::System::Nullable_1<
        crate::Newtonsoft::Json::ReferenceLoopHandling,
    >,
    pub _context: crate::System::Nullable_1<
        crate::System::Runtime::Serialization::StreamingContext,
    >,
    pub _constructorHandling: crate::System::Nullable_1<
        crate::Newtonsoft::Json::ConstructorHandling,
    >,
    pub _typeNameHandling: crate::System::Nullable_1<
        crate::Newtonsoft::Json::TypeNameHandling,
    >,
    pub _metadataPropertyHandling: crate::System::Nullable_1<
        crate::Newtonsoft::Json::MetadataPropertyHandling,
    >,
    pub _Converters_k__BackingField: *mut crate::System::Collections::Generic::IList_1<
        *mut crate::Newtonsoft::Json::JsonConverter,
    >,
    pub _ContractResolver_k__BackingField: *mut crate::Newtonsoft::Json::Serialization::IContractResolver,
    pub _EqualityComparer_k__BackingField: *mut crate::System::Collections::IEqualityComparer,
    pub _ReferenceResolverProvider_k__BackingField: *mut crate::System::Func_1<
        *mut crate::Newtonsoft::Json::Serialization::IReferenceResolver,
    >,
    pub _TraceWriter_k__BackingField: *mut crate::Newtonsoft::Json::Serialization::ITraceWriter,
    pub _SerializationBinder_k__BackingField: *mut crate::Newtonsoft::Json::Serialization::ISerializationBinder,
    pub _Error_k__BackingField: *mut crate::System::EventHandler_1<
        *mut crate::Newtonsoft::Json::Serialization::ErrorEventArgs,
    >,
}
#[cfg(feature = "Newtonsoft+Json+JsonSerializerSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::JsonSerializerSettings =>
    "Newtonsoft.Json"."JsonSerializerSettings"
);
#[cfg(feature = "Newtonsoft+Json+JsonSerializerSettings")]
impl std::ops::Deref for crate::Newtonsoft::Json::JsonSerializerSettings {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonSerializerSettings")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::JsonSerializerSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonSerializerSettings")]
impl crate::Newtonsoft::Json::JsonSerializerSettings {
    pub const DefaultCheckAdditionalContent: bool = false;
    pub const DefaultDateFormatString: &'static str = "yyyy\\'-\\'MM\\'-\\'dd\\'T\\'HH\\':\\'mm\\':\\'ss.FFFFFFFK";
    pub const DefaultMaxDepth: i32 = 64i32;
    #[cfg(feature = "Newtonsoft+Json+JsonSerializerSettings+__c__DisplayClass93_0")]
    pub type __c__DisplayClass93_0 = crate::Newtonsoft::Json::JsonSerializerSettings___c__DisplayClass93_0;
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_JsonSerializerSettings1(
        original: *mut crate::Newtonsoft::Json::JsonSerializerSettings,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (original))?;
        Ok(__cordl_object)
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
    pub fn _ctor_JsonSerializerSettings1(
        &mut self,
        original: *mut crate::Newtonsoft::Json::JsonSerializerSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (original))?;
        Ok(__cordl_ret)
    }
    pub fn get_Binder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::Serialization::SerializationBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::Serialization::SerializationBinder = __cordl_object
            .invoke("get_Binder", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CheckAdditionalContent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CheckAdditionalContent", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ConstructorHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::ConstructorHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::ConstructorHandling = __cordl_object
            .invoke("get_ConstructorHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Context(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::Serialization::StreamingContext,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Runtime::Serialization::StreamingContext = __cordl_object
            .invoke("get_Context", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ContractResolver(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::IContractResolver,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::IContractResolver = __cordl_object
            .invoke("get_ContractResolver", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Converters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IList_1<
            *mut crate::Newtonsoft::Json::JsonConverter,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::Newtonsoft::Json::JsonConverter,
        > = __cordl_object.invoke("get_Converters", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Culture(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Globalization::CultureInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Globalization::CultureInfo = __cordl_object
            .invoke("get_Culture", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DateFormatHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::DateFormatHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::DateFormatHandling = __cordl_object
            .invoke("get_DateFormatHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DateFormatString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_DateFormatString", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DateParseHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::DateParseHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::DateParseHandling = __cordl_object
            .invoke("get_DateParseHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DateTimeZoneHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::DateTimeZoneHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::DateTimeZoneHandling = __cordl_object
            .invoke("get_DateTimeZoneHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_DefaultValueHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::DefaultValueHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::DefaultValueHandling = __cordl_object
            .invoke("get_DefaultValueHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EqualityComparer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::IEqualityComparer,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEqualityComparer = __cordl_object
            .invoke("get_EqualityComparer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Error(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::EventHandler_1<
            *mut crate::Newtonsoft::Json::Serialization::ErrorEventArgs,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::EventHandler_1<
            *mut crate::Newtonsoft::Json::Serialization::ErrorEventArgs,
        > = __cordl_object.invoke("get_Error", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FloatFormatHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::FloatFormatHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::FloatFormatHandling = __cordl_object
            .invoke("get_FloatFormatHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FloatParseHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::FloatParseHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::FloatParseHandling = __cordl_object
            .invoke("get_FloatParseHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Formatting(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Formatting> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Formatting = __cordl_object
            .invoke("get_Formatting", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MaxDepth(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<i32> = __cordl_object
            .invoke("get_MaxDepth", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MetadataPropertyHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Newtonsoft::Json::MetadataPropertyHandling,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::MetadataPropertyHandling = __cordl_object
            .invoke("get_MetadataPropertyHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MissingMemberHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::MissingMemberHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::MissingMemberHandling = __cordl_object
            .invoke("get_MissingMemberHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NullValueHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::NullValueHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::NullValueHandling = __cordl_object
            .invoke("get_NullValueHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ObjectCreationHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::ObjectCreationHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::ObjectCreationHandling = __cordl_object
            .invoke("get_ObjectCreationHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PreserveReferencesHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Newtonsoft::Json::PreserveReferencesHandling,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::PreserveReferencesHandling = __cordl_object
            .invoke("get_PreserveReferencesHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ReferenceLoopHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::ReferenceLoopHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::ReferenceLoopHandling = __cordl_object
            .invoke("get_ReferenceLoopHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ReferenceResolver(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::IReferenceResolver,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::IReferenceResolver = __cordl_object
            .invoke("get_ReferenceResolver", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ReferenceResolverProvider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Func_1<
            *mut crate::Newtonsoft::Json::Serialization::IReferenceResolver,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Func_1<
            *mut crate::Newtonsoft::Json::Serialization::IReferenceResolver,
        > = __cordl_object.invoke("get_ReferenceResolverProvider", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SerializationBinder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::ISerializationBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::ISerializationBinder = __cordl_object
            .invoke("get_SerializationBinder", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_StringEscapeHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::StringEscapeHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::StringEscapeHandling = __cordl_object
            .invoke("get_StringEscapeHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TraceWriter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::ITraceWriter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::ITraceWriter = __cordl_object
            .invoke("get_TraceWriter", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TypeNameAssemblyFormat(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::Serialization::Formatters::FormatterAssemblyStyle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Runtime::Serialization::Formatters::FormatterAssemblyStyle = __cordl_object
            .invoke("get_TypeNameAssemblyFormat", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TypeNameAssemblyFormatHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Newtonsoft::Json::TypeNameAssemblyFormatHandling,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::TypeNameAssemblyFormatHandling = __cordl_object
            .invoke("get_TypeNameAssemblyFormatHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TypeNameHandling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::TypeNameHandling> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::TypeNameHandling = __cordl_object
            .invoke("get_TypeNameHandling", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Binder(
        &mut self,
        value: *mut crate::System::Runtime::Serialization::SerializationBinder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Binder", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_CheckAdditionalContent(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CheckAdditionalContent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ConstructorHandling(
        &mut self,
        value: crate::Newtonsoft::Json::ConstructorHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ConstructorHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Context(
        &mut self,
        value: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Context", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ContractResolver(
        &mut self,
        value: *mut crate::Newtonsoft::Json::Serialization::IContractResolver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ContractResolver", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Converters(
        &mut self,
        value: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::Newtonsoft::Json::JsonConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Converters", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Culture(
        &mut self,
        value: *mut crate::System::Globalization::CultureInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Culture", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_DateFormatHandling(
        &mut self,
        value: crate::Newtonsoft::Json::DateFormatHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DateFormatHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_DateFormatString(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DateFormatString", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_DateParseHandling(
        &mut self,
        value: crate::Newtonsoft::Json::DateParseHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DateParseHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_DateTimeZoneHandling(
        &mut self,
        value: crate::Newtonsoft::Json::DateTimeZoneHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DateTimeZoneHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_DefaultValueHandling(
        &mut self,
        value: crate::Newtonsoft::Json::DefaultValueHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_DefaultValueHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_EqualityComparer(
        &mut self,
        value: *mut crate::System::Collections::IEqualityComparer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_EqualityComparer", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Error(
        &mut self,
        value: *mut crate::System::EventHandler_1<
            *mut crate::Newtonsoft::Json::Serialization::ErrorEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Error", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_FloatFormatHandling(
        &mut self,
        value: crate::Newtonsoft::Json::FloatFormatHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FloatFormatHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_FloatParseHandling(
        &mut self,
        value: crate::Newtonsoft::Json::FloatParseHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FloatParseHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Formatting(
        &mut self,
        value: crate::Newtonsoft::Json::Formatting,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Formatting", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_MaxDepth(
        &mut self,
        value: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MaxDepth", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_MetadataPropertyHandling(
        &mut self,
        value: crate::Newtonsoft::Json::MetadataPropertyHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MetadataPropertyHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_MissingMemberHandling(
        &mut self,
        value: crate::Newtonsoft::Json::MissingMemberHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MissingMemberHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_NullValueHandling(
        &mut self,
        value: crate::Newtonsoft::Json::NullValueHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_NullValueHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ObjectCreationHandling(
        &mut self,
        value: crate::Newtonsoft::Json::ObjectCreationHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ObjectCreationHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_PreserveReferencesHandling(
        &mut self,
        value: crate::Newtonsoft::Json::PreserveReferencesHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PreserveReferencesHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ReferenceLoopHandling(
        &mut self,
        value: crate::Newtonsoft::Json::ReferenceLoopHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ReferenceLoopHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ReferenceResolver(
        &mut self,
        value: *mut crate::Newtonsoft::Json::Serialization::IReferenceResolver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ReferenceResolver", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ReferenceResolverProvider(
        &mut self,
        value: *mut crate::System::Func_1<
            *mut crate::Newtonsoft::Json::Serialization::IReferenceResolver,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ReferenceResolverProvider", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_SerializationBinder(
        &mut self,
        value: *mut crate::Newtonsoft::Json::Serialization::ISerializationBinder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SerializationBinder", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_StringEscapeHandling(
        &mut self,
        value: crate::Newtonsoft::Json::StringEscapeHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_StringEscapeHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_TraceWriter(
        &mut self,
        value: *mut crate::Newtonsoft::Json::Serialization::ITraceWriter,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TraceWriter", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_TypeNameAssemblyFormat(
        &mut self,
        value: crate::System::Runtime::Serialization::Formatters::FormatterAssemblyStyle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TypeNameAssemblyFormat", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_TypeNameAssemblyFormatHandling(
        &mut self,
        value: crate::Newtonsoft::Json::TypeNameAssemblyFormatHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TypeNameAssemblyFormatHandling", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_TypeNameHandling(
        &mut self,
        value: crate::Newtonsoft::Json::TypeNameHandling,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TypeNameHandling", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonSerializerSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::JsonSerializerSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
