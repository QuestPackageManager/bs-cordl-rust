#[cfg(feature = "System+Net+Http+Headers+HeaderInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct HeaderInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub AllowsMany: bool,
    pub HeaderKind: crate::System::Net::Http::Headers::HttpHeaderKind,
    pub Name: *mut quest_hook::libil2cpp::Il2CppString,
    pub _CustomToString_k__BackingField: *mut crate::System::Func_2<
        *mut quest_hook::libil2cpp::Il2CppObject,
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "System+Net+Http+Headers+HeaderInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Http::Headers::HeaderInfo =>
    "System.Net.Http.Headers"."HeaderInfo"
);
#[cfg(feature = "System+Net+Http+Headers+HeaderInfo")]
impl std::ops::Deref for crate::System::Net::Http::Headers::HeaderInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    #[cfg(feature = "System+Net+Http+Headers+HeaderInfo+CollectionHeaderTypeInfo_2")]
    pub type CollectionHeaderTypeInfo_2<
        T: quest_hook::libil2cpp::Type,
        U: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::HeaderInfo_CollectionHeaderTypeInfo_2<T, U>;
    #[cfg(feature = "System+Net+Http+Headers+HeaderInfo+HeaderTypeInfo_2")]
    pub type HeaderTypeInfo_2<
        T: quest_hook::libil2cpp::Type,
        U: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::HeaderInfo_HeaderTypeInfo_2<T, U>;
    pub fn AddToCollection(
        &mut self,
        collection: *mut quest_hook::libil2cpp::Il2CppObject,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddToCollection", (collection, value))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCollection_HeaderInfo1(
        &mut self,
        headers: *mut crate::System::Net::Http::Headers::HttpHeaders,
        headerInfo: *mut crate::System::Net::Http::Headers::HeaderInfo,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("CreateCollection", (headers, headerInfo))?;
        Ok(__cordl_ret)
    }
    pub fn CreateCollection_HttpHeaders0(
        &mut self,
        headers: *mut crate::System::Net::Http::Headers::HttpHeaders,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke("CreateCollection", (headers))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        name: *mut quest_hook::libil2cpp::Il2CppString,
        headerKind: crate::System::Net::Http::Headers::HttpHeaderKind,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, headerKind))?;
        Ok(__cordl_object)
    }
    pub fn ToStringCollection(
        &mut self,
        collection: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToStringCollection", (collection))?;
        Ok(__cordl_ret)
    }
    pub fn TryParse(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
        result: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryParse", (value, result))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        name: *mut quest_hook::libil2cpp::Il2CppString,
        headerKind: crate::System::Net::Http::Headers::HttpHeaderKind,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, headerKind))?;
        Ok(__cordl_ret)
    }
    pub fn get_CustomToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Func_2<
            *mut quest_hook::libil2cpp::Il2CppObject,
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Func_2<
            *mut quest_hook::libil2cpp::Il2CppObject,
            *mut quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_CustomToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Separator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Separator", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_CustomToString(
        &mut self,
        value: *mut crate::System::Func_2<
            *mut quest_hook::libil2cpp::Il2CppObject,
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CustomToString", (value))?;
        Ok(__cordl_ret)
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
