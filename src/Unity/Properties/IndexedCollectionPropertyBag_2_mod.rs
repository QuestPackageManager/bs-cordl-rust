#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
#[repr(C)]
#[derive(Debug)]
pub struct IndexedCollectionPropertyBag_2<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Unity::Properties::PropertyBag_1<TList>,
    pub m_Property: quest_hook::libil2cpp::Gc<
        crate::Unity::Properties::IndexedCollectionPropertyBag_2_ListElementProperty<
            TList,
            TElement,
        >,
    >,
    __cordl_phantom_TList: std::marker::PhantomData<TList>,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::IndexedCollectionPropertyBag_2 < TList, TElement > =>
    "Unity.Properties"."IndexedCollectionPropertyBag`2" < TList, TElement >
);
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    type Target = crate::Unity::Properties::PropertyBag_1<TList>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    #[cfg(
        feature = "Unity+Properties+IndexedCollectionPropertyBag_2+ListElementProperty"
    )]
    pub type ListElementProperty = crate::Unity::Properties::IndexedCollectionPropertyBag_2_ListElementProperty<
        TList,
        TElement,
    >;
    pub fn InstantiateWithCount(
        &mut self,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<TList>
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TList = __cordl_object.invoke("InstantiateWithCount", (count))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Unity_Properties_IConstructorWithCount_TList__InstantiateWithCount(
        &mut self,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<TList>
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TList = __cordl_object
            .invoke(
                "Unity.Properties.IConstructorWithCount<TList>.InstantiateWithCount",
                (count),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IConstructor>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_ref(&self) -> &crate::Unity::Properties::IConstructor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IConstructor>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IConstructor {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IConstructorWithCount_1<TList>>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_ref(&self) -> &crate::Unity::Properties::IConstructorWithCount_1<TList> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IConstructorWithCount_1<TList>>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::IConstructorWithCount_1<TList> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IPropertyBag>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_ref(&self) -> &crate::Unity::Properties::IPropertyBag {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IPropertyBag>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IPropertyBag {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IPropertyBag_1<TList>>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_ref(&self) -> &crate::Unity::Properties::IPropertyBag_1<TList> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IPropertyBag_1<TList>>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IPropertyBag_1<TList> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2+ListElementProperty")]
#[repr(C)]
#[derive(Debug)]
pub struct IndexedCollectionPropertyBag_2_ListElementProperty<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Unity::Properties::Property_2<TList, TElement>,
    pub m_Index: i32,
    __cordl_phantom_TList: std::marker::PhantomData<TList>,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2+ListElementProperty")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Properties::IndexedCollectionPropertyBag_2_ListElementProperty < TList,
    TElement > => "Unity.Properties"."IndexedCollectionPropertyBag`2/ListElementProperty"
    < TList, TElement >
);
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2+ListElementProperty")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Unity::Properties::IndexedCollectionPropertyBag_2_ListElementProperty<
    TList,
    TElement,
> {
    type Target = crate::Unity::Properties::Property_2<TList, TElement>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2+ListElementProperty")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Unity::Properties::IndexedCollectionPropertyBag_2_ListElementProperty<
    TList,
    TElement,
> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2+ListElementProperty")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::IndexedCollectionPropertyBag_2_ListElementProperty<
    TList,
    TElement,
> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Index(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Index", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2+ListElementProperty")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::IndexedCollectionPropertyBag_2_ListElementProperty<
    TList,
    TElement,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
