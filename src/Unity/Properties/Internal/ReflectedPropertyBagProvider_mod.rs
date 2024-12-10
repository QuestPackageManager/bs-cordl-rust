#[cfg(feature = "Unity+Properties+Internal+ReflectedPropertyBagProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectedPropertyBagProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_CreatePropertyMethod: *mut crate::System::Reflection::MethodInfo,
    pub m_CreatePropertyBagMethod: *mut crate::System::Reflection::MethodInfo,
    pub m_CreateIndexedCollectionPropertyBagMethod: *mut crate::System::Reflection::MethodInfo,
    pub m_CreateSetPropertyBagMethod: *mut crate::System::Reflection::MethodInfo,
    pub m_CreateKeyValueCollectionPropertyBagMethod: *mut crate::System::Reflection::MethodInfo,
    pub m_CreateKeyValuePairPropertyBagMethod: *mut crate::System::Reflection::MethodInfo,
    pub m_CreateArrayPropertyBagMethod: *mut crate::System::Reflection::MethodInfo,
    pub m_CreateListPropertyBagMethod: *mut crate::System::Reflection::MethodInfo,
    pub m_CreateHashSetPropertyBagMethod: *mut crate::System::Reflection::MethodInfo,
    pub m_CreateDictionaryPropertyBagMethod: *mut crate::System::Reflection::MethodInfo,
}
#[cfg(feature = "Unity+Properties+Internal+ReflectedPropertyBagProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::Internal::ReflectedPropertyBagProvider =>
    "Unity.Properties.Internal"."ReflectedPropertyBagProvider"
);
#[cfg(feature = "Unity+Properties+Internal+ReflectedPropertyBagProvider")]
impl std::ops::Deref
for crate::Unity::Properties::Internal::ReflectedPropertyBagProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+ReflectedPropertyBagProvider")]
impl std::ops::DerefMut
for crate::Unity::Properties::Internal::ReflectedPropertyBagProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+ReflectedPropertyBagProvider")]
impl crate::Unity::Properties::Internal::ReflectedPropertyBagProvider {
    #[cfg(
        feature = "Unity+Properties+Internal+ReflectedPropertyBagProvider+_GetPropertyMembers_d__22"
    )]
    pub type _GetPropertyMembers_d__22 = crate::Unity::Properties::Internal::ReflectedPropertyBagProvider__GetPropertyMembers_d__22;
    #[cfg(feature = "Unity+Properties+Internal+ReflectedPropertyBagProvider+__c")]
    pub type __c = crate::Unity::Properties::Internal::ReflectedPropertyBagProvider___c;
    pub fn CreateArrayPropertyBag<TElement>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag_1<
                *mut quest_hook::libil2cpp::Il2CppArray<TElement>,
            >,
        >,
    >
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag_1<
                *mut quest_hook::libil2cpp::Il2CppArray<TElement>,
            >,
        > = __cordl_object.invoke("CreateArrayPropertyBag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateDictionaryPropertyBag<TKey, TValue>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag_1<
                *mut crate::System::Collections::Generic::Dictionary_2<TKey, TValue>,
            >,
        >,
    >
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag_1<
                *mut crate::System::Collections::Generic::Dictionary_2<TKey, TValue>,
            >,
        > = __cordl_object.invoke("CreateDictionaryPropertyBag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateHashSetPropertyBag<TElement>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag_1<
                *mut crate::System::Collections::Generic::HashSet_1<TElement>,
            >,
        >,
    >
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag_1<
                *mut crate::System::Collections::Generic::HashSet_1<TElement>,
            >,
        > = __cordl_object.invoke("CreateHashSetPropertyBag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateIndexedCollectionPropertyBag<TList, TElement>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::IPropertyBag_1<TList>>,
    >
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag_1<TList>,
        > = __cordl_object.invoke("CreateIndexedCollectionPropertyBag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateKeyValueCollectionPropertyBag<TDictionary, TKey, TValue>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::IPropertyBag_1<TDictionary>>,
    >
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag_1<TDictionary>,
        > = __cordl_object.invoke("CreateKeyValueCollectionPropertyBag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateKeyValuePairPropertyBag<TKey, TValue>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag_1<
                crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
            >,
        >,
    >
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag_1<
                crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
            >,
        > = __cordl_object.invoke("CreateKeyValuePairPropertyBag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateListPropertyBag<TElement>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag_1<
                *mut crate::System::Collections::Generic::List_1<TElement>,
            >,
        >,
    >
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag_1<
                *mut crate::System::Collections::Generic::List_1<TElement>,
            >,
        > = __cordl_object.invoke("CreateListPropertyBag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateProperty<TContainer, TValue>(
        &mut self,
        member: quest_hook::libil2cpp::Gc<crate::Unity::Properties::IMemberInfo>,
        propertyBag: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::Internal::ReflectedPropertyBag_1<TContainer>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateProperty", (member, propertyBag))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePropertyBag_1<TContainer>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::IPropertyBag_1<TContainer>>,
    >
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag_1<TContainer>,
        > = __cordl_object.invoke("CreatePropertyBag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreatePropertyBag_Type0(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::IPropertyBag>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag,
        > = __cordl_object.invoke("CreatePropertyBag", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSetPropertyBag<TSet, TValue>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::IPropertyBag_1<TSet>>,
    >
    where
        TSet: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag_1<TSet>,
        > = __cordl_object.invoke("CreateSetPropertyBag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Properties+Internal+ReflectedPropertyBagProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::ReflectedPropertyBagProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
