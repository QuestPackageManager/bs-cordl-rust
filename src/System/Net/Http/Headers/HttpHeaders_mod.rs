#[cfg(feature = "System+Net+Http+Headers+HttpHeaders+HeaderBucket")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpHeaders_HeaderBucket {
    __cordl_parent: crate::System::Object,
    pub Parsed: *mut crate::System::Object,
    pub values: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub CustomToString: *mut crate::System::Func_2<
        *mut crate::System::Object,
        *mut crate::System::String,
    >,
}
#[cfg(feature = "System+Net+Http+Headers+HttpHeaders+HeaderBucket")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::Http::Headers::HttpHeaders_HeaderBucket => "System.Net.Http.Headers"
    ."HttpHeaders/HeaderBucket"
);
#[cfg(feature = "System+Net+Http+Headers+HttpHeaders+HeaderBucket")]
impl std::ops::Deref for crate::System::Net::Http::Headers::HttpHeaders_HeaderBucket {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+HttpHeaders+HeaderBucket")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::HttpHeaders_HeaderBucket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+HttpHeaders+HeaderBucket")]
impl crate::System::Net::Http::Headers::HttpHeaders_HeaderBucket {
    pub fn ParsedToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ParsedToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Values(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_Values", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        parsed: *mut crate::System::Object,
        converter: *mut crate::System::Func_2<
            *mut crate::System::Object,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parsed, converter))?;
        Ok(__cordl_ret)
    }
    pub fn set_Values(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Values", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_HasStringValues(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasStringValues", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        parsed: *mut crate::System::Object,
        converter: *mut crate::System::Func_2<
            *mut crate::System::Object,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parsed, converter))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+Http+Headers+HttpHeaders+HeaderBucket")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::HttpHeaders_HeaderBucket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+Http+Headers+HttpHeaders")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpHeaders {
    __cordl_parent: crate::System::Object,
    pub headers: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::System::Net::Http::Headers::HttpHeaders_HeaderBucket,
    >,
    pub HeaderKind: crate::System::Net::Http::Headers::HttpHeaderKind,
    pub connectionclose: crate::System::Nullable_1<bool>,
    pub transferEncodingChunked: crate::System::Nullable_1<bool>,
}
#[cfg(feature = "System+Net+Http+Headers+HttpHeaders")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::HttpHeaders =>
    "System.Net.Http.Headers"."HttpHeaders"
);
#[cfg(feature = "System+Net+Http+Headers+HttpHeaders")]
impl std::ops::Deref for crate::System::Net::Http::Headers::HttpHeaders {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+HttpHeaders")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::HttpHeaders {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+HttpHeaders")]
impl crate::System::Net::Http::Headers::HttpHeaders {
    #[cfg(feature = "System+Net+Http+Headers+HttpHeaders+HeaderBucket")]
    pub type HeaderBucket = crate::System::Net::Http::Headers::HttpHeaders_HeaderBucket;
    #[cfg(feature = "System+Net+Http+Headers+HttpHeaders+_GetEnumerator_d__19")]
    pub type _GetEnumerator_d__19 = crate::System::Net::Http::Headers::HttpHeaders__GetEnumerator_d__19;
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn Remove(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Remove", (name))?;
        Ok(__cordl_ret)
    }
    pub fn TryCheckName(
        &mut self,
        name: *mut crate::System::String,
        headerInfo: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::Net::Http::Headers::HeaderInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryCheckName", (name, headerInfo))?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerator_1<
            crate::System::Collections::Generic::KeyValuePair_2<
                *mut crate::System::String,
                *mut crate::System::Collections::Generic::IEnumerable_1<
                    *mut crate::System::String,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerator_1<
            crate::System::Collections::Generic::KeyValuePair_2<
                *mut crate::System::String,
                *mut crate::System::Collections::Generic::IEnumerable_1<
                    *mut crate::System::String,
                >,
            >,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetValue<T>(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object.invoke("GetValue", (name))?;
        Ok(__cordl_ret)
    }
    pub fn TryAddWithoutValidation(
        &mut self,
        name: *mut crate::System::String,
        values: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryAddWithoutValidation", (name, values))?;
        Ok(__cordl_ret)
    }
    pub fn SetValue<T>(
        &mut self,
        name: *mut crate::System::String,
        value: T,
        toStringConverter: *mut crate::System::Func_2<
            *mut crate::System::Object,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetValue", (name, value, toStringConverter))?;
        Ok(__cordl_ret)
    }
    pub fn AddInternal(
        &mut self,
        name: *mut crate::System::String,
        values: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::String,
        >,
        headerInfo: *mut crate::System::Net::Http::Headers::HeaderInfo,
        ignoreInvalid: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AddInternal", (name, values, headerInfo, ignoreInvalid))?;
        Ok(__cordl_ret)
    }
    pub fn CheckName(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Net::Http::Headers::HeaderInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Http::Headers::HeaderInfo = __cordl_object
            .invoke("CheckName", (name))?;
        Ok(__cordl_ret)
    }
    pub fn AddOrRemove<T>(
        &mut self,
        name: *mut crate::System::String,
        value: T,
        converter: *mut crate::System::Func_2<
            *mut crate::System::Object,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddOrRemove", (name, value, converter))?;
        Ok(__cordl_ret)
    }
    pub fn GetAllHeaderValues(
        &mut self,
        bucket: *mut crate::System::Net::Http::Headers::HttpHeaders_HeaderBucket,
        headerInfo: *mut crate::System::Net::Http::Headers::HeaderInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("GetAllHeaderValues", (bucket, headerInfo))?;
        Ok(__cordl_ret)
    }
    pub fn GetValues<T>(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Net::Http::Headers::HttpHeaderValueCollection_1<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Http::Headers::HttpHeaderValueCollection_1<
            T,
        > = __cordl_object.invoke("GetValues", (name))?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
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
    pub fn _ctor_HttpHeaderKind1(
        &mut self,
        headerKind: crate::System::Net::Http::Headers::HttpHeaderKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (headerKind))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_HttpHeaderKind1(
        headerKind: crate::System::Net::Http::Headers::HttpHeaderKind,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (headerKind))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+Http+Headers+HttpHeaders")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::HttpHeaders {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
