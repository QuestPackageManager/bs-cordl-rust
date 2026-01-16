#[cfg(
    feature = "cordl_class_Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable+Enumerator"
)]
#[repr(C)]
#[derive(Debug)]
pub struct Enumerable_KeyValueCollectionPropertyBag_3_Enumerator<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Dictionary: TDictionary,
    pub m_Property: quest_hook::libil2cpp::Gc<
        crate::Unity::Properties::KeyValueCollectionPropertyBag_3_KeyValuePairProperty<
            TDictionary,
            TKey,
            TValue,
        >,
    >,
    pub m_Previous: TKey,
    pub m_Keys: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<TKey>,
    >,
    pub m_Position: i32,
    __cordl_phantom_TDictionary: std::marker::PhantomData<TDictionary>,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(
    feature = "cordl_class_Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable+Enumerator"
)]
unsafe impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::Unity::Properties::Enumerable_KeyValueCollectionPropertyBag_3_Enumerator<
    TDictionary,
    TKey,
    TValue,
> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "KeyValueCollectionPropertyBag`3/Enumerable/Enumerator";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Unity.Properties",
                        "KeyValueCollectionPropertyBag`3/Enumerable/Enumerator",
                    )
                    .unwrap()
                    .make_generic::<(TDictionary, TKey, TValue)>()
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
#[cfg(
    feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable+Enumerator"
)]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Unity::Properties::Enumerable_KeyValueCollectionPropertyBag_3_Enumerator<
    TDictionary,
    TKey,
    TValue,
> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable+Enumerator"
)]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Unity::Properties::Enumerable_KeyValueCollectionPropertyBag_3_Enumerator<
    TDictionary,
    TKey,
    TValue,
> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable+Enumerator"
)]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::Enumerable_KeyValueCollectionPropertyBag_3_Enumerator<
    TDictionary,
    TKey,
    TValue,
> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("MoveNext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MoveNext", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        dictionary: TDictionary,
        property: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::KeyValueCollectionPropertyBag_3_KeyValuePairProperty<
                TDictionary,
                TKey,
                TValue,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
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
            .invoke_void(".ctor", (dictionary, property))?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Reset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("System.Collections.IEnumerator.get_Current")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Collections.IEnumerator.get_Current", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        dictionary: TDictionary,
        property: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::KeyValueCollectionPropertyBag_3_KeyValuePairProperty<
                TDictionary,
                TKey,
                TValue,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            TDictionary,
                            quest_hook::libil2cpp::Gc<
                                crate::Unity::Properties::KeyValueCollectionPropertyBag_3_KeyValuePairProperty<
                                    TDictionary,
                                    TKey,
                                    TValue,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (dictionary, property))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty_1<TDictionary>>,
    >
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::Unity::Properties::IProperty_1<TDictionary>,
                        >,
                        0usize,
                    >("get_Current")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Current", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IProperty_1<TDictionary>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable+Enumerator"
)]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Enumerable_KeyValueCollectionPropertyBag_3_Enumerator<
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
#[cfg(
    feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable+Enumerator"
)]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<
    crate::System::Collections::Generic::IEnumerator_1<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty_1<TDictionary>>,
    >,
>
for crate::Unity::Properties::Enumerable_KeyValueCollectionPropertyBag_3_Enumerator<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerator_1<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty_1<TDictionary>>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable+Enumerator"
)]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<
    crate::System::Collections::Generic::IEnumerator_1<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty_1<TDictionary>>,
    >,
>
for crate::Unity::Properties::Enumerable_KeyValueCollectionPropertyBag_3_Enumerator<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerator_1<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty_1<TDictionary>>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable+Enumerator"
)]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::IEnumerator>
for crate::Unity::Properties::Enumerable_KeyValueCollectionPropertyBag_3_Enumerator<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable+Enumerator"
)]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::IEnumerator>
for crate::Unity::Properties::Enumerable_KeyValueCollectionPropertyBag_3_Enumerator<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable+Enumerator"
)]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::System::IDisposable>
for crate::Unity::Properties::Enumerable_KeyValueCollectionPropertyBag_3_Enumerator<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable+Enumerator"
)]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::System::IDisposable>
for crate::Unity::Properties::Enumerable_KeyValueCollectionPropertyBag_3_Enumerator<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+KeyValueCollectionPropertyBag_3")]
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
#[cfg(feature = "cordl_class_Unity+Properties+KeyValueCollectionPropertyBag_3")]
unsafe impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3<
    TDictionary,
    TKey,
    TValue,
> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "KeyValueCollectionPropertyBag`3";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Unity.Properties",
                        "KeyValueCollectionPropertyBag`3",
                    )
                    .unwrap()
                    .make_generic::<(TDictionary, TKey, TValue)>()
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
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
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
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::KeyValueCollectionPropertyBag_3<TDictionary, TKey, TValue> {
    #[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable")]
    pub type Enumerable = crate::Unity::Properties::KeyValueCollectionPropertyBag_3_Enumerable<
        TDictionary,
        TKey,
        TValue,
    >;
    #[cfg(
        feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+KeyValuePairProperty"
    )]
    pub type KeyValuePairProperty = crate::Unity::Properties::KeyValueCollectionPropertyBag_3_KeyValuePairProperty<
        TDictionary,
        TKey,
        TValue,
    >;
    pub fn GetProperties_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Properties::PropertyCollection_1<TDictionary>,
    >
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::Unity::Properties::PropertyCollection_1<TDictionary>,
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
        let __cordl_ret: crate::Unity::Properties::PropertyCollection_1<TDictionary> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetProperties_ByRefMut1(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<TDictionary>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Properties::PropertyCollection_1<TDictionary>,
    >
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<TDictionary>),
                        crate::Unity::Properties::PropertyCollection_1<TDictionary>,
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
        let __cordl_ret: crate::Unity::Properties::PropertyCollection_1<TDictionary> = unsafe {
            cordl_method_info.invoke_unchecked(self, (container))?
        };
        Ok(__cordl_ret.into())
    }
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
    pub fn Unity_Properties_ICollectionPropertyBagAccept_TDictionary__Accept(
        &mut self,
        visitor: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::ICollectionPropertyBagVisitor,
        >,
        container: quest_hook::libil2cpp::ByRefMut<TDictionary>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
                            quest_hook::libil2cpp::ByRefMut<TDictionary>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(
                        "Unity.Properties.ICollectionPropertyBagAccept<TDictionary>.Accept",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Properties.ICollectionPropertyBagAccept<TDictionary>.Accept",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (visitor, container))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_IDictionaryPropertyBagAccept_TDictionary__Accept(
        &mut self,
        visitor: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IDictionaryPropertyBagVisitor,
        >,
        container: quest_hook::libil2cpp::ByRefMut<TDictionary>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Unity::Properties::IDictionaryPropertyBagVisitor,
                            >,
                            quest_hook::libil2cpp::ByRefMut<TDictionary>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(
                        "Unity.Properties.IDictionaryPropertyBagAccept<TDictionary>.Accept",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Properties.IDictionaryPropertyBagAccept<TDictionary>.Accept",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (visitor, container))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_IKeyedProperties_TDictionary_System_Object__TryGetProperty(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<TDictionary>,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        property: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty_1<TDictionary>>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<TDictionary>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::Unity::Properties::IProperty_1<TDictionary>,
                                >,
                            >,
                        ),
                        bool,
                        3usize,
                    >(
                        "Unity.Properties.IKeyedProperties<TDictionary,System.Object>.TryGetProperty",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "Unity.Properties.IKeyedProperties<TDictionary,System.Object>.TryGetProperty",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (container, key, property))?
        };
        Ok(__cordl_ret.into())
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
#[cfg(feature = "cordl_class_Unity+Properties+KeyValueCollectionPropertyBag_3")]
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
> AsRef<crate::Unity::Properties::ICollectionPropertyBagAccept_1<TDictionary>>
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_ref(
        &self,
    ) -> &crate::Unity::Properties::ICollectionPropertyBagAccept_1<TDictionary> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::ICollectionPropertyBagAccept_1<TDictionary>>
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::ICollectionPropertyBagAccept_1<TDictionary> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<
    crate::Unity::Properties::ICollectionPropertyBag_2<
        TDictionary,
        crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
    >,
>
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_ref(
        &self,
    ) -> &crate::Unity::Properties::ICollectionPropertyBag_2<
        TDictionary,
        crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<
    crate::Unity::Properties::ICollectionPropertyBag_2<
        TDictionary,
        crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
    >,
>
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::ICollectionPropertyBag_2<
        TDictionary,
        crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IDictionaryPropertyBagAccept_1<TDictionary>>
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_ref(
        &self,
    ) -> &crate::Unity::Properties::IDictionaryPropertyBagAccept_1<TDictionary> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IDictionaryPropertyBagAccept_1<TDictionary>>
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::IDictionaryPropertyBagAccept_1<TDictionary> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IDictionaryPropertyBag_3<TDictionary, TKey, TValue>>
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_ref(
        &self,
    ) -> &crate::Unity::Properties::IDictionaryPropertyBag_3<TDictionary, TKey, TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IDictionaryPropertyBag_3<TDictionary, TKey, TValue>>
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::IDictionaryPropertyBag_3<
        TDictionary,
        TKey,
        TValue,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<
    crate::Unity::Properties::IKeyedProperties_2<
        TDictionary,
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >,
>
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_ref(
        &self,
    ) -> &crate::Unity::Properties::IKeyedProperties_2<
        TDictionary,
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<
    crate::Unity::Properties::IKeyedProperties_2<
        TDictionary,
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >,
>
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::IKeyedProperties_2<
        TDictionary,
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        unsafe { std::mem::transmute(self) }
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
#[cfg(
    feature = "cordl_class_Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct KeyValueCollectionPropertyBag_3_Enumerable<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    pub m_Dictionary: TDictionary,
    pub m_Property: quest_hook::libil2cpp::Gc<
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
#[cfg(
    feature = "cordl_class_Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable"
)]
unsafe impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3_Enumerable<
    TDictionary,
    TKey,
    TValue,
> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "KeyValueCollectionPropertyBag`3/Enumerable";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Unity.Properties",
                        "KeyValueCollectionPropertyBag`3/Enumerable",
                    )
                    .unwrap()
                    .make_generic::<(TDictionary, TKey, TValue)>()
                    .unwrap()
                    .unwrap()
            })
    }
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable"
)]
unsafe impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Argument
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3_Enumerable<
    TDictionary,
    TKey,
    TValue,
> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable"
)]
unsafe impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Parameter
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3_Enumerable<
    TDictionary,
    TKey,
    TValue,
> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable"
)]
unsafe impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Returned
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3_Enumerable<
    TDictionary,
    TKey,
    TValue,
> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(
    feature = "cordl_class_Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable"
)]
unsafe impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Return
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3_Enumerable<
    TDictionary,
    TKey,
    TValue,
> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(
    feature = "cordl_class_Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable"
)]
unsafe impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ThisArgument
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3_Enumerable<
    TDictionary,
    TKey,
    TValue,
> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::KeyValueCollectionPropertyBag_3_Enumerable<
    TDictionary,
    TKey,
    TValue,
> {
    #[cfg(
        feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable+Enumerator"
    )]
    pub type Enumerator = crate::Unity::Properties::Enumerable_KeyValueCollectionPropertyBag_3_Enumerator<
        TDictionary,
        TKey,
        TValue,
    >;
    pub fn System_Collections_Generic_IEnumerable_Unity_Properties_IProperty_TDictionary___GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                quest_hook::libil2cpp::Gc<
                    crate::Unity::Properties::IProperty_1<TDictionary>,
                >,
            >,
        >,
    >
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerator_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::Unity::Properties::IProperty_1<TDictionary>,
                                >,
                            >,
                        >,
                        0usize,
                    >(
                        "System.Collections.Generic.IEnumerable<Unity.Properties.IProperty<TDictionary>>.GetEnumerator",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Collections.Generic.IEnumerable<Unity.Properties.IProperty<TDictionary>>.GetEnumerator",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                quest_hook::libil2cpp::Gc<
                    crate::Unity::Properties::IProperty_1<TDictionary>,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    >
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerator,
                        >,
                        0usize,
                    >("System.Collections.IEnumerable.GetEnumerator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Collections.IEnumerable.GetEnumerator", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        dictionary: TDictionary,
        property: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::KeyValueCollectionPropertyBag_3_KeyValuePairProperty<
                TDictionary,
                TKey,
                TValue,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            TDictionary,
                            quest_hook::libil2cpp::Gc<
                                crate::Unity::Properties::KeyValueCollectionPropertyBag_3_KeyValuePairProperty<
                                    TDictionary,
                                    TKey,
                                    TValue,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (dictionary, property))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty_1<TDictionary>>,
    >,
>
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3_Enumerable<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty_1<TDictionary>>,
    > {
        todo!()
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty_1<TDictionary>>,
    >,
>
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3_Enumerable<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty_1<TDictionary>>,
    > {
        todo!()
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::IEnumerable>
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3_Enumerable<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+Enumerable")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::IEnumerable>
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3_Enumerable<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        todo!()
    }
}
#[cfg(
    feature = "cordl_class_Unity+Properties+KeyValueCollectionPropertyBag_3+KeyValuePairProperty"
)]
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
#[cfg(
    feature = "cordl_class_Unity+Properties+KeyValueCollectionPropertyBag_3+KeyValuePairProperty"
)]
unsafe impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3_KeyValuePairProperty<
    TDictionary,
    TKey,
    TValue,
> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "KeyValueCollectionPropertyBag`3/KeyValuePairProperty";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Unity.Properties",
                        "KeyValueCollectionPropertyBag`3/KeyValuePairProperty",
                    )
                    .unwrap()
                    .make_generic::<(TDictionary, TKey, TValue)>()
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
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
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
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
    pub fn GetValue(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<TDictionary>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
    >
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<TDictionary>),
                        crate::System::Collections::Generic::KeyValuePair_2<
                            TKey,
                            TValue,
                        >,
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
        let __cordl_ret: crate::System::Collections::Generic::KeyValuePair_2<
            TKey,
            TValue,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (container))? };
        Ok(__cordl_ret.into())
    }
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
    pub fn SetValue(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<TDictionary>,
        value: crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<TDictionary>,
                            crate::System::Collections::Generic::KeyValuePair_2<
                                TKey,
                                TValue,
                            >,
                        ),
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
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
    pub fn get_Key(&mut self) -> quest_hook::libil2cpp::Result<TKey>
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), TKey, 0usize>("get_Key")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "get_Key",
                            0usize
                        )
                    })
            });
        let __cordl_ret: TKey = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
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
    pub fn get_ObjectKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("get_ObjectKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ObjectKey", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_Key(
        &mut self,
        value: TKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDictionary: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (TKey),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_Key")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "set_Key",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_Unity+Properties+KeyValueCollectionPropertyBag_3+KeyValuePairProperty"
)]
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
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+KeyValuePairProperty")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::Unity::Properties::IDictionaryElementProperty>
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3_KeyValuePairProperty<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_ref(&self) -> &crate::Unity::Properties::IDictionaryElementProperty {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+KeyValueCollectionPropertyBag_3+KeyValuePairProperty")]
impl<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::Unity::Properties::IDictionaryElementProperty>
for crate::Unity::Properties::KeyValueCollectionPropertyBag_3_KeyValuePairProperty<
    TDictionary,
    TKey,
    TValue,
> {
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IDictionaryElementProperty {
        unsafe { std::mem::transmute(self) }
    }
}
