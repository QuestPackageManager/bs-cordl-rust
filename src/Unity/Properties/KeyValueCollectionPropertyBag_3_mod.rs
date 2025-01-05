#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyValueCollectionPropertyBag_3<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Unity::Properties::PropertyBag_1<TDictionary>,
    pub m_KeyValuePairProperty: quest_hook::libil2cpp::Gc<
        crate::Unity::Properties::KeyValueCollectionPropertyBag_3_KeyValuePairProperty<
            TDictionary,
            TKey,
            TValue,
        >,
    >,
    __cordl_phantom_TDictionary: std::marker::PhantomData<TDictionary>,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::KeyValueCollectionPropertyBag_3 < TDictionary, TKey, TValue > =>
    "Unity.Properties"."KeyValueCollectionPropertyBag`3" < TDictionary, TKey, TValue >
);
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3<
    TDictionary,
    TKey,
    TValue,
> {
    type Target = crate::Unity::Properties::PropertyBag_1<TDictionary>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3<
    TDictionary,
    TKey,
    TValue,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::KeyValueCollectionPropertyBag_3<TDictionary, TKey, TValue> {
    #[cfg(
        feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+KeyValuePairProperty"
    )]
    pub type KeyValuePairProperty = crate::Unity::Properties::KeyValueCollectionPropertyBag_3_KeyValuePairProperty<
        TDictionary,
        TKey,
        TValue,
    >;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IPropertyBag>
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_ref(&self) -> &crate::Unity::Properties::IPropertyBag {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IPropertyBag>
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IPropertyBag {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IPropertyBag_1<TDictionary>>
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_ref(&self) -> &crate::Unity::Properties::IPropertyBag_1<TDictionary> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IPropertyBag_1<TDictionary>>
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IPropertyBag_1<TDictionary> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+KeyValuePairProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyValueCollectionPropertyBag_3_KeyValuePairProperty<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Unity::Properties::Property_2<
        TDictionary,
        crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
    >,
    pub _Key_k__BackingField: TKey,
    __cordl_phantom_TDictionary: std::marker::PhantomData<TDictionary>,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+KeyValuePairProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::KeyValueCollectionPropertyBag_3_KeyValuePairProperty <
    TDictionary, TKey, TValue > => "Unity.Properties"
    ."KeyValueCollectionPropertyBag`3/KeyValuePairProperty" < TDictionary, TKey, TValue >
);
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+KeyValuePairProperty")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3_KeyValuePairProperty<
    TDictionary,
    TKey,
    TValue,
> {
    type Target = crate::Unity::Properties::Property_2<
        TDictionary,
        crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+KeyValuePairProperty")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3_KeyValuePairProperty<
    TDictionary,
    TKey,
    TValue,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+KeyValuePairProperty")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::KeyValueCollectionPropertyBag_3_KeyValuePairProperty<
    TDictionary,
    TKey,
    TValue,
> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Key(&mut self) -> quest_hook::libil2cpp::Result<TKey>
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TKey = __cordl_object.invoke("get_Key", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_Name", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+KeyValuePairProperty")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3_KeyValuePairProperty<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
