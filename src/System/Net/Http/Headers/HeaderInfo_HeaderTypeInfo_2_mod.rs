#[cfg(feature = "System+Net+Http+Headers+HeaderInfo+HeaderTypeInfo_2")]
#[repr(C)]
#[derive(Debug)]
pub struct HeaderInfo_HeaderTypeInfo_2<
    T: quest_hook::libil2cpp::Type,
    U: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Net::Http::Headers::HeaderInfo,
    pub parser: *mut crate::System::Net::Http::Headers::TryParseDelegate_1<T>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
    __cordl_phantom_U: std::marker::PhantomData<U>,
}
#[cfg(feature = "System+Net+Http+Headers+HeaderInfo+HeaderTypeInfo_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::HeaderInfo_HeaderTypeInfo_2 <
    T, U > => "System.Net.Http.Headers"."HeaderInfo/HeaderTypeInfo`2" < T, U >
);
#[cfg(feature = "System+Net+Http+Headers+HeaderInfo+HeaderTypeInfo_2")]
impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::GlobalNamespace::HeaderInfo_HeaderTypeInfo_2<T, U> {
    type Target = crate::System::Net::Http::Headers::HeaderInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+HeaderInfo+HeaderTypeInfo_2")]
impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::GlobalNamespace::HeaderInfo_HeaderTypeInfo_2<T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+HeaderInfo+HeaderTypeInfo_2")]
impl<
    T: quest_hook::libil2cpp::Type,
    U: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::HeaderInfo_HeaderTypeInfo_2<T, U> {
    pub fn AddToCollection(
        &mut self,
        collection: *mut crate::System::Object,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddToCollection", (collection, value))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCollection(
        &mut self,
        headers: *mut crate::System::Net::Http::Headers::HttpHeaders,
        headerInfo: *mut crate::System::Net::Http::Headers::HeaderInfo,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("CreateCollection", (headers, headerInfo))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        name: *mut crate::System::String,
        parser: *mut crate::System::Net::Http::Headers::TryParseDelegate_1<T>,
        headerKind: crate::System::Net::Http::Headers::HttpHeaderKind,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, parser, headerKind))?;
        Ok(__cordl_object)
    }
    pub fn ToStringCollection(
        &mut self,
        collection: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::String>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("ToStringCollection", (collection))?;
        Ok(__cordl_ret)
    }
    pub fn TryParse(
        &mut self,
        value: *mut crate::System::String,
        result: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryParse", (value, result))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        name: *mut crate::System::String,
        parser: *mut crate::System::Net::Http::Headers::TryParseDelegate_1<T>,
        headerKind: crate::System::Net::Http::Headers::HttpHeaderKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, parser, headerKind))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+Http+Headers+HeaderInfo+HeaderTypeInfo_2")]
impl<
    T: quest_hook::libil2cpp::Type,
    U: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HeaderInfo_HeaderTypeInfo_2<T, U> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}