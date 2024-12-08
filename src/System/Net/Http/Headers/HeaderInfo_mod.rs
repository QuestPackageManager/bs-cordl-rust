#[cfg(feature = "System+Net+Http+Headers+HeaderInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct HeaderInfo {
    __cordl_parent: crate::System::Object,
    pub AllowsMany: bool,
    pub HeaderKind: crate::System::Net::Http::Headers::HttpHeaderKind,
    pub Name: *mut crate::System::String,
    pub _CustomToString_k__BackingField: *mut crate::System::Func_2<
        *mut crate::System::Object,
        *mut crate::System::String,
    >,
}
#[cfg(feature = "System+Net+Http+Headers+HeaderInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::HeaderInfo =>
    "System.Net.Http.Headers"."HeaderInfo"
);
#[cfg(feature = "System+Net+Http+Headers+HeaderInfo")]
impl std::ops::Deref for crate::System::Net::Http::Headers::HeaderInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+HeaderInfo")]
impl std::ops::DerefMut for crate::System::Net::Http::Headers::HeaderInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+HeaderInfo")]
impl crate::System::Net::Http::Headers::HeaderInfo {
    #[cfg(feature = "System+Net+Http+Headers+HeaderInfo+HeaderTypeInfo_2")]
    pub type HeaderTypeInfo_2<
        T: quest_hook::libil2cpp::Type,
        U: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::HeaderInfo_HeaderTypeInfo_2<T, U>;
    #[cfg(feature = "System+Net+Http+Headers+HeaderInfo+CollectionHeaderTypeInfo_2")]
    pub type CollectionHeaderTypeInfo_2<
        T: quest_hook::libil2cpp::Type,
        U: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::HeaderInfo_CollectionHeaderTypeInfo_2<T, U>;
    pub fn get_CustomToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Func_2<
            *mut crate::System::Object,
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Func_2<
            *mut crate::System::Object,
            *mut crate::System::String,
        > = __cordl_object.invoke("get_CustomToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToStringCollection(
        &mut self,
        collection: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("ToStringCollection", (collection))?;
        Ok(__cordl_ret)
    }
    pub fn get_Separator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Separator", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        name: *mut crate::System::String,
        headerKind: crate::System::Net::Http::Headers::HttpHeaderKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, headerKind))?;
        Ok(__cordl_ret)
    }
    pub fn TryParse(
        &mut self,
        value: *mut crate::System::String,
        result: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryParse", (value, result))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCollection_HttpHeaders0(
        &mut self,
        headers: *mut crate::System::Net::Http::Headers::HttpHeaders,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("CreateCollection", (headers))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCollection_HeaderInfo1(
        &mut self,
        headers: *mut crate::System::Net::Http::Headers::HttpHeaders,
        headerInfo: *mut crate::System::Net::Http::Headers::HeaderInfo,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("CreateCollection", (headers, headerInfo))?;
        Ok(__cordl_ret)
    }
    pub fn AddToCollection(
        &mut self,
        collection: *mut crate::System::Object,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddToCollection", (collection, value))?;
        Ok(__cordl_ret)
    }
    pub fn set_CustomToString(
        &mut self,
        value: *mut crate::System::Func_2<
            *mut crate::System::Object,
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CustomToString", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        name: *mut crate::System::String,
        headerKind: crate::System::Net::Http::Headers::HttpHeaderKind,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, headerKind))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+Http+Headers+HeaderInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Http::Headers::HeaderInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
