#[cfg(feature = "cordl_class_Unity+Properties+IndexedCollectionPropertyBag_2")]
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
#[cfg(feature = "cordl_class_Unity+Properties+IndexedCollectionPropertyBag_2")]
unsafe impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "IndexedCollectionPropertyBag`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Unity.Properties",
                        "IndexedCollectionPropertyBag`2",
                    )
                    .unwrap()
                    .make_generic::<(TList, TElement)>()
                    .unwrap()
                    .unwrap()
            })
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    type Target = crate::Unity::Properties::PropertyBag_1<TList>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
    pub fn GetProperties_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Properties::PropertyCollection_1<TList>,
    >
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Properties::PropertyCollection_1<TList>,
                        0usize,
                    >("GetProperties")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetProperties", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Properties::PropertyCollection_1<TList> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetProperties_ByRefMut1(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<TList>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Properties::PropertyCollection_1<TList>,
    >
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<TList>),
                        crate::Unity::Properties::PropertyCollection_1<TList>,
                        1usize,
                    >("GetProperties")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetProperties", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Properties::PropertyCollection_1<TList> = unsafe {
            cordl_method_info.invoke_unchecked(self, (container))?
        };
        Ok(__cordl_ret.into())
    }
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), TList, 1usize>("InstantiateWithCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InstantiateWithCount", 1usize
                        )
                    })
            });
        let __cordl_ret: TList = unsafe {
            cordl_method_info.invoke_unchecked(self, (count))?
        };
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
    pub fn TryGetProperty(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<TList>,
        index: i32,
        property: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty_1<TList>>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<TList>,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::Unity::Properties::IProperty_1<TList>,
                                >,
                            >,
                        ),
                        bool,
                        3usize,
                    >("TryGetProperty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "TryGetProperty", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (container, index, property))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_ICollectionPropertyBagAccept_TList__Accept(
        &mut self,
        visitor: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::ICollectionPropertyBagVisitor,
        >,
        container: quest_hook::libil2cpp::ByRefMut<TList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Unity::Properties::ICollectionPropertyBagVisitor,
                            >,
                            quest_hook::libil2cpp::ByRefMut<TList>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Unity.Properties.ICollectionPropertyBagAccept<TList>.Accept")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Properties.ICollectionPropertyBagAccept<TList>.Accept",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (visitor, container))?
        };
        Ok(__cordl_ret.into())
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32),
                        TList,
                        1usize,
                    >(
                        "Unity.Properties.IConstructorWithCount<TList>.InstantiateWithCount",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Properties.IConstructorWithCount<TList>.InstantiateWithCount",
                            1usize
                        )
                    })
            });
        let __cordl_ret: TList = unsafe {
            cordl_method_info.invoke_unchecked(self, (count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_IIndexedCollectionPropertyBagEnumerator_TList__GetCount(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<TList>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<TList>),
                        i32,
                        1usize,
                    >(
                        "Unity.Properties.IIndexedCollectionPropertyBagEnumerator<TList>.GetCount",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Properties.IIndexedCollectionPropertyBagEnumerator<TList>.GetCount",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked(self, (container))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_IIndexedCollectionPropertyBagEnumerator_TList__GetSharedProperty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty_1<TList>>,
    >
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Unity::Properties::IProperty_1<TList>,
                        >,
                        0usize,
                    >(
                        "Unity.Properties.IIndexedCollectionPropertyBagEnumerator<TList>.GetSharedProperty",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Properties.IIndexedCollectionPropertyBagEnumerator<TList>.GetSharedProperty",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IProperty_1<TList>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_IIndexedCollectionPropertyBagEnumerator_TList__GetSharedPropertyState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Properties::IndexedCollectionSharedPropertyState,
    >
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Properties::IndexedCollectionSharedPropertyState,
                        0usize,
                    >(
                        "Unity.Properties.IIndexedCollectionPropertyBagEnumerator<TList>.GetSharedPropertyState",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Properties.IIndexedCollectionPropertyBagEnumerator<TList>.GetSharedPropertyState",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Properties::IndexedCollectionSharedPropertyState = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_IIndexedCollectionPropertyBagEnumerator_TList__SetSharedPropertyState(
        &mut self,
        state: crate::Unity::Properties::IndexedCollectionSharedPropertyState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Unity::Properties::IndexedCollectionSharedPropertyState),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(
                        "Unity.Properties.IIndexedCollectionPropertyBagEnumerator<TList>.SetSharedPropertyState",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Properties.IIndexedCollectionPropertyBagEnumerator<TList>.SetSharedPropertyState",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (state))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_IListPropertyAccept_TList__Accept<TContainer>(
        &mut self,
        visitor: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IListPropertyVisitor,
        >,
        property: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::Property_2<TContainer, TList>,
        >,
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
        list: quest_hook::libil2cpp::ByRefMut<TList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Unity::Properties::IListPropertyVisitor,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::Unity::Properties::Property_2<TContainer, TList>,
                            >,
                            quest_hook::libil2cpp::ByRefMut<TContainer>,
                            quest_hook::libil2cpp::ByRefMut<TList>,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("Unity.Properties.IListPropertyAccept<TList>.Accept")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Properties.IListPropertyAccept<TList>.Accept", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (visitor, property, container, list))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_IListPropertyBagAccept_TList__Accept(
        &mut self,
        visitor: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IListPropertyBagVisitor,
        >,
        list: quest_hook::libil2cpp::ByRefMut<TList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Unity::Properties::IListPropertyBagVisitor,
                            >,
                            quest_hook::libil2cpp::ByRefMut<TList>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Unity.Properties.IListPropertyBagAccept<TList>.Accept")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Properties.IListPropertyBagAccept<TList>.Accept",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (visitor, list))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+IndexedCollectionPropertyBag_2")]
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
> AsRef<crate::Unity::Properties::ICollectionPropertyBagAccept_1<TList>>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_ref(
        &self,
    ) -> &crate::Unity::Properties::ICollectionPropertyBagAccept_1<TList> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::ICollectionPropertyBagAccept_1<TList>>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::ICollectionPropertyBagAccept_1<TList> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::ICollectionPropertyBag_2<TList, TElement>>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_ref(
        &self,
    ) -> &crate::Unity::Properties::ICollectionPropertyBag_2<TList, TElement> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::ICollectionPropertyBag_2<TList, TElement>>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::ICollectionPropertyBag_2<TList, TElement> {
        unsafe { std::mem::transmute(self) }
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
> AsRef<crate::Unity::Properties::IIndexedCollectionPropertyBagEnumerator_1<TList>>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_ref(
        &self,
    ) -> &crate::Unity::Properties::IIndexedCollectionPropertyBagEnumerator_1<TList> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IIndexedCollectionPropertyBagEnumerator_1<TList>>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::IIndexedCollectionPropertyBagEnumerator_1<
        TList,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IIndexedProperties_1<TList>>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_ref(&self) -> &crate::Unity::Properties::IIndexedProperties_1<TList> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IIndexedProperties_1<TList>>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IIndexedProperties_1<TList> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IListPropertyAccept_1<TList>>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_ref(&self) -> &crate::Unity::Properties::IListPropertyAccept_1<TList> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IListPropertyAccept_1<TList>>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IListPropertyAccept_1<TList> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IListPropertyBagAccept_1<TList>>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_ref(&self) -> &crate::Unity::Properties::IListPropertyBagAccept_1<TList> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IListPropertyBagAccept_1<TList>>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::IListPropertyBagAccept_1<TList> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IListPropertyBag_2<TList, TElement>>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_ref(&self) -> &crate::Unity::Properties::IListPropertyBag_2<TList, TElement> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IListPropertyBag_2<TList, TElement>>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2<TList, TElement> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::IListPropertyBag_2<TList, TElement> {
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
#[cfg(
    feature = "cordl_class_Unity+Properties+IndexedCollectionPropertyBag_2+ListElementProperty"
)]
#[repr(C)]
#[derive(Debug)]
pub struct IndexedCollectionPropertyBag_2_ListElementProperty<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Unity::Properties::Property_2<TList, TElement>,
    pub m_Index: i32,
    pub m_IsReadOnly: bool,
    __cordl_phantom_TList: std::marker::PhantomData<TList>,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
}
#[cfg(
    feature = "cordl_class_Unity+Properties+IndexedCollectionPropertyBag_2+ListElementProperty"
)]
unsafe impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::Unity::Properties::IndexedCollectionPropertyBag_2_ListElementProperty<
    TList,
    TElement,
> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "IndexedCollectionPropertyBag`2/ListElementProperty";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Unity.Properties",
                        "IndexedCollectionPropertyBag`2/ListElementProperty",
                    )
                    .unwrap()
                    .make_generic::<(TList, TElement)>()
                    .unwrap()
                    .unwrap()
            })
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
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
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
    pub fn GetValue(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<TList>,
    ) -> quest_hook::libil2cpp::Result<TElement>
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<TList>),
                        TElement,
                        1usize,
                    >("GetValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetValue", 1usize
                        )
                    })
            });
        let __cordl_ret: TElement = unsafe {
            cordl_method_info.invoke_unchecked(self, (container))?
        };
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
    pub fn SetValue(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<TList>,
        value: TElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<TList>, TElement),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetValue", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (container, value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Index(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_Index")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Index", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TList: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsReadOnly")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_IsReadOnly", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_Name")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Name", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_Unity+Properties+IndexedCollectionPropertyBag_2+ListElementProperty"
)]
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
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2+ListElementProperty")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IListElementProperty>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2_ListElementProperty<
    TList,
    TElement,
> {
    fn as_ref(&self) -> &crate::Unity::Properties::IListElementProperty {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IndexedCollectionPropertyBag_2+ListElementProperty")]
impl<
    TList: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IListElementProperty>
for crate::Unity::Properties::IndexedCollectionPropertyBag_2_ListElementProperty<
    TList,
    TElement,
> {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IListElementProperty {
        unsafe { std::mem::transmute(self) }
    }
}
