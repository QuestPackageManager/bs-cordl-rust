#[cfg(feature = "System+Net+Http+Headers+HeaderInfo+CollectionHeaderTypeInfo_2")]
#[repr(C)]
#[derive(Debug)]
pub struct HeaderInfo_CollectionHeaderTypeInfo_2<
    T: quest_hook::libil2cpp::Type,
    U: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::GlobalNamespace::HeaderInfo_HeaderTypeInfo_2<T, U>,
    pub minimalCount: i32,
    pub separator: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub parser: quest_hook::libil2cpp::Gc<
        crate::System::Net::Http::Headers::TryParseListDelegate_1<T>,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
    __cordl_phantom_U: std::marker::PhantomData<U>,
}
#[cfg(feature = "System+Net+Http+Headers+HeaderInfo+CollectionHeaderTypeInfo_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::HeaderInfo_CollectionHeaderTypeInfo_2 < T, U > =>
    "System.Net.Http.Headers"."HeaderInfo/CollectionHeaderTypeInfo`2" < T, U >
);
#[cfg(feature = "System+Net+Http+Headers+HeaderInfo+CollectionHeaderTypeInfo_2")]
impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::GlobalNamespace::HeaderInfo_CollectionHeaderTypeInfo_2<T, U> {
    type Target = crate::GlobalNamespace::HeaderInfo_HeaderTypeInfo_2<T, U>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+HeaderInfo+CollectionHeaderTypeInfo_2")]
impl<T: quest_hook::libil2cpp::Type, U: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::GlobalNamespace::HeaderInfo_CollectionHeaderTypeInfo_2<T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+Headers+HeaderInfo+CollectionHeaderTypeInfo_2")]
impl<
    T: quest_hook::libil2cpp::Type,
    U: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::HeaderInfo_CollectionHeaderTypeInfo_2<T, U> {
    pub fn New(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parser: quest_hook::libil2cpp::Gc<
            crate::System::Net::Http::Headers::TryParseListDelegate_1<T>,
        >,
        headerKind: crate::System::Net::Http::Headers::HttpHeaderKind,
        minimalCount: i32,
        separator: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, parser, headerKind, minimalCount, separator))?;
        Ok(__cordl_object.into())
    }
    pub fn TryParse(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<*mut quest_hook::libil2cpp::Il2CppObject>,
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parser: quest_hook::libil2cpp::Gc<
            crate::System::Net::Http::Headers::TryParseListDelegate_1<T>,
        >,
        headerKind: crate::System::Net::Http::Headers::HttpHeaderKind,
        minimalCount: i32,
        separator: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
            .invoke(".ctor", (name, parser, headerKind, minimalCount, separator))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Separator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Separator", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+Headers+HeaderInfo+CollectionHeaderTypeInfo_2")]
impl<
    T: quest_hook::libil2cpp::Type,
    U: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HeaderInfo_CollectionHeaderTypeInfo_2<T, U> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
