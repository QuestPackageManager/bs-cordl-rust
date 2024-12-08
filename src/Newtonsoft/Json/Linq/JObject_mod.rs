#[cfg(feature = "Newtonsoft+Json+Linq+JObject+__WriteToAsync_g__AwaitProperties_0_0_d")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct JObject___WriteToAsync_g__AwaitProperties_0_0_d {
    pub __1__state: i32,
    pub __t__builder: crate::System::Runtime::CompilerServices::AsyncTaskMethodBuilder,
    pub task: *mut crate::System::Threading::Tasks::Task,
    pub __4__this: *mut crate::Newtonsoft::Json::Linq::JObject,
    pub i: i32,
    pub Writer: *mut crate::Newtonsoft::Json::JsonWriter,
    pub CancellationToken: crate::System::Threading::CancellationToken,
    pub Converters: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::Newtonsoft::Json::JsonConverter,
    >,
    pub __u__1: crate::System::Runtime::CompilerServices::ConfiguredTaskAwaitable_ConfiguredTaskAwaiter,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JObject+__WriteToAsync_g__AwaitProperties_0_0_d")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Linq::JObject___WriteToAsync_g__AwaitProperties_0_0_d =>
    "Newtonsoft.Json.Linq"."JObject/<<WriteToAsync>g__AwaitProperties|0_0>d"
);
#[cfg(feature = "Newtonsoft+Json+Linq+JObject+__WriteToAsync_g__AwaitProperties_0_0_d")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Newtonsoft::Json::Linq::JObject___WriteToAsync_g__AwaitProperties_0_0_d {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JObject+__WriteToAsync_g__AwaitProperties_0_0_d")]
impl crate::Newtonsoft::Json::Linq::JObject___WriteToAsync_g__AwaitProperties_0_0_d {
    pub fn MoveNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SetStateMachine(
        &mut self,
        stateMachine: *mut crate::System::Runtime::CompilerServices::IAsyncStateMachine,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetStateMachine",
            (stateMachine),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JObject")]
#[repr(C)]
#[derive(Debug)]
pub struct JObject {
    __cordl_parent: crate::Newtonsoft::Json::Linq::JContainer,
    pub _properties: *mut crate::Newtonsoft::Json::Linq::JPropertyKeyedCollection,
    pub PropertyChanged: *mut crate::System::ComponentModel::PropertyChangedEventHandler,
    pub PropertyChanging: *mut crate::System::ComponentModel::PropertyChangingEventHandler,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JObject")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Linq::JObject =>
    "Newtonsoft.Json.Linq"."JObject"
);
#[cfg(feature = "Newtonsoft+Json+Linq+JObject")]
impl std::ops::Deref for crate::Newtonsoft::Json::Linq::JObject {
    type Target = crate::Newtonsoft::Json::Linq::JContainer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JObject")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Linq::JObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JObject")]
impl crate::Newtonsoft::Json::Linq::JObject {
    #[cfg(feature = "Newtonsoft+Json+Linq+JObject+JObjectDynamicProxy")]
    pub type JObjectDynamicProxy = crate::Newtonsoft::Json::Linq::JObject_JObjectDynamicProxy;
    #[cfg(feature = "Newtonsoft+Json+Linq+JObject+_GetEnumerator_d__64")]
    pub type _GetEnumerator_d__64 = crate::Newtonsoft::Json::Linq::JObject__GetEnumerator_d__64;
    #[cfg(feature = "Newtonsoft+Json+Linq+JObject+_LoadAsync_d__2")]
    pub type _LoadAsync_d__2 = crate::Newtonsoft::Json::Linq::JObject__LoadAsync_d__2;
    #[cfg(
        feature = "Newtonsoft+Json+Linq+JObject+__WriteToAsync_g__AwaitProperties_0_0_d"
    )]
    pub type __WriteToAsync_g__AwaitProperties_0_0_d = crate::Newtonsoft::Json::Linq::JObject___WriteToAsync_g__AwaitProperties_0_0_d;
    #[cfg(feature = "Newtonsoft+Json+Linq+JObject+__c")]
    pub type __c = crate::Newtonsoft::Json::Linq::JObject___c;
    pub fn Add(
        &mut self,
        propertyName: *mut crate::System::String,
        value: *mut crate::Newtonsoft::Json::Linq::JToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (propertyName, value))?;
        Ok(__cordl_ret)
    }
    pub fn CloneToken(
        &mut self,
        settings: *mut crate::Newtonsoft::Json::Linq::JsonCloneSettings,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Newtonsoft::Json::Linq::JToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Linq::JToken = __cordl_object
            .invoke("CloneToken", (settings))?;
        Ok(__cordl_ret)
    }
    pub fn ContainsKey(
        &mut self,
        propertyName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ContainsKey", (propertyName))?;
        Ok(__cordl_ret)
    }
    pub fn DeepEquals(
        &mut self,
        node: *mut crate::Newtonsoft::Json::Linq::JToken,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("DeepEquals", (node))?;
        Ok(__cordl_ret)
    }
    pub fn GetDeepHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetDeepHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<
            crate::System::Collections::Generic::KeyValuePair_2<
                *mut crate::System::String,
                *mut crate::Newtonsoft::Json::Linq::JToken,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<
            crate::System::Collections::Generic::KeyValuePair_2<
                *mut crate::System::String,
                *mut crate::Newtonsoft::Json::Linq::JToken,
            >,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetMetaObject(
        &mut self,
        parameter: *mut crate::System::Linq::Expressions::Expression,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Dynamic::DynamicMetaObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Dynamic::DynamicMetaObject = __cordl_object
            .invoke("GetMetaObject", (parameter))?;
        Ok(__cordl_ret)
    }
    pub fn GetValue_String0(
        &mut self,
        propertyName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Newtonsoft::Json::Linq::JToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Linq::JToken = __cordl_object
            .invoke("GetValue", (propertyName))?;
        Ok(__cordl_ret)
    }
    pub fn GetValue_StringComparison1(
        &mut self,
        propertyName: *mut crate::System::String,
        comparison: crate::System::StringComparison,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Newtonsoft::Json::Linq::JToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Linq::JToken = __cordl_object
            .invoke("GetValue", (propertyName, comparison))?;
        Ok(__cordl_ret)
    }
    pub fn IndexOfItem(
        &mut self,
        item: *mut crate::Newtonsoft::Json::Linq::JToken,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("IndexOfItem", (item))?;
        Ok(__cordl_ret)
    }
    pub fn InsertItem(
        &mut self,
        index: i32,
        item: *mut crate::Newtonsoft::Json::Linq::JToken,
        skipParentCheck: bool,
        copyAnnotations: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InsertItem", (index, item, skipParentCheck, copyAnnotations))?;
        Ok(__cordl_ret)
    }
    pub fn InternalPropertyChanged(
        &mut self,
        childProperty: *mut crate::Newtonsoft::Json::Linq::JProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalPropertyChanged", (childProperty))?;
        Ok(__cordl_ret)
    }
    pub fn InternalPropertyChanging(
        &mut self,
        childProperty: *mut crate::Newtonsoft::Json::Linq::JProperty,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalPropertyChanging", (childProperty))?;
        Ok(__cordl_ret)
    }
    pub fn MergeItem(
        &mut self,
        content: *mut crate::System::Object,
        settings: *mut crate::Newtonsoft::Json::Linq::JsonMergeSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MergeItem", (content, settings))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray3(
        content: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (content))?;
        Ok(__cordl_object)
    }
    pub fn New_JObject1(
        other: *mut crate::Newtonsoft::Json::Linq::JObject,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (other))?;
        Ok(__cordl_object)
    }
    pub fn New_JObject_JsonCloneSettings2(
        other: *mut crate::Newtonsoft::Json::Linq::JObject,
        settings: *mut crate::Newtonsoft::Json::Linq::JsonCloneSettings,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (other, settings))?;
        Ok(__cordl_object)
    }
    pub fn New_Object4(
        content: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (content))?;
        Ok(__cordl_object)
    }
    pub fn OnPropertyChanged(
        &mut self,
        propertyName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPropertyChanged", (propertyName))?;
        Ok(__cordl_ret)
    }
    pub fn OnPropertyChanging(
        &mut self,
        propertyName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPropertyChanging", (propertyName))?;
        Ok(__cordl_ret)
    }
    pub fn Properties(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Newtonsoft::Json::Linq::JProperty,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::Newtonsoft::Json::Linq::JProperty,
        > = __cordl_object.invoke("Properties", ())?;
        Ok(__cordl_ret)
    }
    pub fn PropertyValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Newtonsoft::Json::Linq::JEnumerable_1<
            *mut crate::Newtonsoft::Json::Linq::JToken,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Linq::JEnumerable_1<
            *mut crate::Newtonsoft::Json::Linq::JToken,
        > = __cordl_object.invoke("PropertyValues", ())?;
        Ok(__cordl_ret)
    }
    pub fn Property_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Newtonsoft::Json::Linq::JProperty> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Linq::JProperty = __cordl_object
            .invoke("Property", (name))?;
        Ok(__cordl_ret)
    }
    pub fn Property_StringComparison1(
        &mut self,
        name: *mut crate::System::String,
        comparison: crate::System::StringComparison,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Newtonsoft::Json::Linq::JProperty> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Linq::JProperty = __cordl_object
            .invoke("Property", (name, comparison))?;
        Ok(__cordl_ret)
    }
    pub fn Remove(
        &mut self,
        propertyName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Remove", (propertyName))?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_ICollection_System_Collections_Generic_KeyValuePair_System_String_Newtonsoft_Json_Linq_JToken___Add(
        &mut self,
        item: crate::System::Collections::Generic::KeyValuePair_2<
            *mut crate::System::String,
            *mut crate::Newtonsoft::Json::Linq::JToken,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Collections.Generic.ICollection<System.Collections.Generic.KeyValuePair<System.String,Newtonsoft.Json.Linq.JToken>>.Add",
                (item),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_ICollection_System_Collections_Generic_KeyValuePair_System_String_Newtonsoft_Json_Linq_JToken___Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Collections.Generic.ICollection<System.Collections.Generic.KeyValuePair<System.String,Newtonsoft.Json.Linq.JToken>>.Clear",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_ICollection_System_Collections_Generic_KeyValuePair_System_String_Newtonsoft_Json_Linq_JToken___Contains(
        &mut self,
        item: crate::System::Collections::Generic::KeyValuePair_2<
            *mut crate::System::String,
            *mut crate::Newtonsoft::Json::Linq::JToken,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "System.Collections.Generic.ICollection<System.Collections.Generic.KeyValuePair<System.String,Newtonsoft.Json.Linq.JToken>>.Contains",
                (item),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_ICollection_System_Collections_Generic_KeyValuePair_System_String_Newtonsoft_Json_Linq_JToken___CopyTo(
        &mut self,
        array: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::System::Collections::Generic::KeyValuePair_2<
                *mut crate::System::String,
                *mut crate::Newtonsoft::Json::Linq::JToken,
            >,
        >,
        arrayIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Collections.Generic.ICollection<System.Collections.Generic.KeyValuePair<System.String,Newtonsoft.Json.Linq.JToken>>.CopyTo",
                (array, arrayIndex),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_ICollection_System_Collections_Generic_KeyValuePair_System_String_Newtonsoft_Json_Linq_JToken___Remove(
        &mut self,
        item: crate::System::Collections::Generic::KeyValuePair_2<
            *mut crate::System::String,
            *mut crate::Newtonsoft::Json::Linq::JToken,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "System.Collections.Generic.ICollection<System.Collections.Generic.KeyValuePair<System.String,Newtonsoft.Json.Linq.JToken>>.Remove",
                (item),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_ICollection_System_Collections_Generic_KeyValuePair_System_String_Newtonsoft_Json_Linq_JToken___get_IsReadOnly(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "System.Collections.Generic.ICollection<System.Collections.Generic.KeyValuePair<System.String,Newtonsoft.Json.Linq.JToken>>.get_IsReadOnly",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IDictionary_System_String_Newtonsoft_Json_Linq_JToken__get_Keys(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::ICollection_1<
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::ICollection_1<
            *mut crate::System::String,
        > = __cordl_object
            .invoke(
                "System.Collections.Generic.IDictionary<System.String,Newtonsoft.Json.Linq.JToken>.get_Keys",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_Generic_IDictionary_System_String_Newtonsoft_Json_Linq_JToken__get_Values(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::ICollection_1<
            *mut crate::Newtonsoft::Json::Linq::JToken,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::ICollection_1<
            *mut crate::Newtonsoft::Json::Linq::JToken,
        > = __cordl_object
            .invoke(
                "System.Collections.Generic.IDictionary<System.String,Newtonsoft.Json.Linq.JToken>.get_Values",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetAttributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::AttributeCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::AttributeCollection = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetAttributes", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetClassName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetClassName", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetComponentName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetComponentName", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetConverter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::TypeConverter,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::TypeConverter = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetConverter", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetDefaultEvent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::EventDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::EventDescriptor = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetDefaultEvent", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetDefaultProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptor,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptor = __cordl_object
            .invoke(
                "System.ComponentModel.ICustomTypeDescriptor.GetDefaultProperty",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEditor(
        &mut self,
        editorBaseType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "System.ComponentModel.ICustomTypeDescriptor.GetEditor",
                (editorBaseType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEvents_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::EventDescriptorCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::EventDescriptorCollection = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetEvents_Il2CppArray0(
        &mut self,
        attributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Attribute,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::EventDescriptorCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::EventDescriptorCollection = __cordl_object
            .invoke(
                "System.ComponentModel.ICustomTypeDescriptor.GetEvents",
                (attributes),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetProperties_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptorCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptorCollection = __cordl_object
            .invoke("System.ComponentModel.ICustomTypeDescriptor.GetProperties", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetProperties_Il2CppArray1(
        &mut self,
        attributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Attribute,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::ComponentModel::PropertyDescriptorCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::ComponentModel::PropertyDescriptorCollection = __cordl_object
            .invoke(
                "System.ComponentModel.ICustomTypeDescriptor.GetProperties",
                (attributes),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_ComponentModel_ICustomTypeDescriptor_GetPropertyOwner(
        &mut self,
        pd: *mut crate::System::ComponentModel::PropertyDescriptor,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "System.ComponentModel.ICustomTypeDescriptor.GetPropertyOwner",
                (pd),
            )?;
        Ok(__cordl_ret)
    }
    pub fn TryGetValue_ByRefMut1(
        &mut self,
        propertyName: *mut crate::System::String,
        value: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Newtonsoft::Json::Linq::JToken,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetValue", (propertyName, value))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetValue_StringComparison_ByRefMut0(
        &mut self,
        propertyName: *mut crate::System::String,
        comparison: crate::System::StringComparison,
        value: quest_hook::libil2cpp::ByRefMut<
            *mut crate::Newtonsoft::Json::Linq::JToken,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetValue", (propertyName, comparison, value))?;
        Ok(__cordl_ret)
    }
    pub fn ValidateToken(
        &mut self,
        o: *mut crate::Newtonsoft::Json::Linq::JToken,
        existing: *mut crate::Newtonsoft::Json::Linq::JToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateToken", (o, existing))?;
        Ok(__cordl_ret)
    }
    pub fn WriteTo(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        converters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Newtonsoft::Json::JsonConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteTo", (writer, converters))?;
        Ok(__cordl_ret)
    }
    pub fn WriteToAsync(
        &mut self,
        writer: *mut crate::Newtonsoft::Json::JsonWriter,
        cancellationToken: crate::System::Threading::CancellationToken,
        converters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Newtonsoft::Json::JsonConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("WriteToAsync", (writer, cancellationToken, converters))?;
        Ok(__cordl_ret)
    }
    pub fn _WriteToAsync_g__AwaitProperties_0_0(
        &mut self,
        task: *mut crate::System::Threading::Tasks::Task,
        i: i32,
        Writer: *mut crate::Newtonsoft::Json::JsonWriter,
        CancellationToken: crate::System::Threading::CancellationToken,
        Converters: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Newtonsoft::Json::JsonConverter,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke(
                "<WriteToAsync>g__AwaitProperties|0_0",
                (task, i, Writer, CancellationToken, Converters),
            )?;
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
    pub fn _ctor_Il2CppArray3(
        &mut self,
        content: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (content))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_JObject1(
        &mut self,
        other: *mut crate::Newtonsoft::Json::Linq::JObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (other))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_JObject_JsonCloneSettings2(
        &mut self,
        other: *mut crate::Newtonsoft::Json::Linq::JObject,
        settings: *mut crate::Newtonsoft::Json::Linq::JsonCloneSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (other, settings))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Object4(
        &mut self,
        content: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (content))?;
        Ok(__cordl_ret)
    }
    pub fn add_PropertyChanged(
        &mut self,
        value: *mut crate::System::ComponentModel::PropertyChangedEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_PropertyChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_PropertyChanging(
        &mut self,
        value: *mut crate::System::ComponentModel::PropertyChangingEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_PropertyChanging", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_ChildrenTokens(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IList_1<
            *mut crate::Newtonsoft::Json::Linq::JToken,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IList_1<
            *mut crate::Newtonsoft::Json::Linq::JToken,
        > = __cordl_object.invoke("get_ChildrenTokens", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_Object0(
        &mut self,
        key: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Newtonsoft::Json::Linq::JToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Linq::JToken = __cordl_object
            .invoke("get_Item", (key))?;
        Ok(__cordl_ret)
    }
    pub fn get_Item_String1(
        &mut self,
        propertyName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Newtonsoft::Json::Linq::JToken> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Linq::JToken = __cordl_object
            .invoke("get_Item", (propertyName))?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Linq::JTokenType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Linq::JTokenType = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_PropertyChanged(
        &mut self,
        value: *mut crate::System::ComponentModel::PropertyChangedEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_PropertyChanged", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_PropertyChanging(
        &mut self,
        value: *mut crate::System::ComponentModel::PropertyChangingEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_PropertyChanging", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Item_Object0(
        &mut self,
        key: *mut crate::System::Object,
        value: *mut crate::Newtonsoft::Json::Linq::JToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Item", (key, value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Item_String1(
        &mut self,
        propertyName: *mut crate::System::String,
        value: *mut crate::Newtonsoft::Json::Linq::JToken,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Item", (propertyName, value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JObject")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::Linq::JObject {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JObject+JObjectDynamicProxy")]
#[repr(C)]
#[derive(Debug)]
pub struct JObject_JObjectDynamicProxy {
    __cordl_parent: crate::Newtonsoft::Json::Utilities::DynamicProxy_1<
        *mut crate::Newtonsoft::Json::Linq::JObject,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Linq+JObject+JObjectDynamicProxy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Linq::JObject_JObjectDynamicProxy => "Newtonsoft.Json.Linq"
    ."JObject/JObjectDynamicProxy"
);
#[cfg(feature = "Newtonsoft+Json+Linq+JObject+JObjectDynamicProxy")]
impl std::ops::Deref for crate::Newtonsoft::Json::Linq::JObject_JObjectDynamicProxy {
    type Target = crate::Newtonsoft::Json::Utilities::DynamicProxy_1<
        *mut crate::Newtonsoft::Json::Linq::JObject,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JObject+JObjectDynamicProxy")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Linq::JObject_JObjectDynamicProxy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JObject+JObjectDynamicProxy")]
impl crate::Newtonsoft::Json::Linq::JObject_JObjectDynamicProxy {
    #[cfg(feature = "Newtonsoft+Json+Linq+JObject+JObjectDynamicProxy+__c")]
    pub type __c = crate::Newtonsoft::Json::Linq::JObjectDynamicProxy___c;
    pub fn GetDynamicMemberNames(
        &mut self,
        instance: *mut crate::Newtonsoft::Json::Linq::JObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("GetDynamicMemberNames", (instance))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn TryGetMember(
        &mut self,
        instance: *mut crate::Newtonsoft::Json::Linq::JObject,
        binder: *mut crate::System::Dynamic::GetMemberBinder,
        result: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetMember", (instance, binder, result))?;
        Ok(__cordl_ret)
    }
    pub fn TrySetMember(
        &mut self,
        instance: *mut crate::Newtonsoft::Json::Linq::JObject,
        binder: *mut crate::System::Dynamic::SetMemberBinder,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TrySetMember", (instance, binder, value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Linq+JObject+JObjectDynamicProxy")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Linq::JObject_JObjectDynamicProxy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
