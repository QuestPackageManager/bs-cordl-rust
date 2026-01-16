#[cfg(feature = "cordl_class_Unity+Properties+KeyValuePairPropertyBag_2")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyValuePairPropertyBag_2<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Unity::Properties::PropertyBag_1<
        crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
    >,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "cordl_class_Unity+Properties+KeyValuePairPropertyBag_2")]
unsafe impl<TKey: quest_hook::libil2cpp::Type, TValue: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Type
    for crate::Unity::Properties::KeyValuePairPropertyBag_2<TKey, TValue>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "KeyValuePairPropertyBag`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "Unity.Properties",
                "KeyValuePairPropertyBag`2",
            )
            .unwrap()
            .make_generic::<(TKey, TValue)>()
            .unwrap()
            .unwrap()
        })
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Unity+Properties+KeyValuePairPropertyBag_2")]
impl<TKey: quest_hook::libil2cpp::Type, TValue: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::Unity::Properties::KeyValuePairPropertyBag_2<TKey, TValue>
{
    type Target = crate::Unity::Properties::PropertyBag_1<
        crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+KeyValuePairPropertyBag_2")]
impl<TKey: quest_hook::libil2cpp::Type, TValue: quest_hook::libil2cpp::Type> std::ops::DerefMut
    for crate::Unity::Properties::KeyValuePairPropertyBag_2<TKey, TValue>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+KeyValuePairPropertyBag_2")]
impl<TKey: quest_hook::libil2cpp::Type, TValue: quest_hook::libil2cpp::Type>
    crate::Unity::Properties::KeyValuePairPropertyBag_2<TKey, TValue>
{
    pub fn GetPropertiesEnumerable() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::Unity::Properties::IProperty_1<
                        crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
                    >,
                >,
            >,
        >,
    >
    where
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<
                            quest_hook::libil2cpp::Gc<
                                crate::Unity::Properties::IProperty_1<
                                    crate::System::Collections::Generic::KeyValuePair_2<
                                        TKey,
                                        TValue,
                                    >,
                                >,
                            >,
                        >,
                    >, 0usize>("GetPropertiesEnumerable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetPropertiesEnumerable",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::Unity::Properties::IProperty_1<
                        crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
                    >,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetProperties_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Properties::PropertyCollection_1<
            crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
        >,
    >
    where
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::Unity::Properties::PropertyCollection_1<
                        crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
                    >, 0usize>("GetProperties")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetProperties",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Properties::PropertyCollection_1<
            crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetProperties_ByRefMut1(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<
            crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::Unity::Properties::PropertyCollection_1<
            crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
        >,
    >
    where
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
                    >), crate::Unity::Properties::PropertyCollection_1<
                        crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
                    >, 1usize>("GetProperties")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetProperties",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Properties::PropertyCollection_1<
            crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (container))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn TryGetProperty(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<
            crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
        >,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        property: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::Unity::Properties::IProperty_1<
                    crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::Unity::Properties::IProperty_1<
                                    crate::System::Collections::Generic::KeyValuePair_2<
                                        TKey,
                                        TValue,
                                    >,
                                >,
                            >,
                        >,
                    ), bool, 3usize>("TryGetProperty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetProperty",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (container, name, property))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+KeyValuePairPropertyBag_2")]
impl<TKey: quest_hook::libil2cpp::Type, TValue: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::ObjectType
    for crate::Unity::Properties::KeyValuePairPropertyBag_2<TKey, TValue>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+KeyValuePairPropertyBag_2")]
impl<TKey: quest_hook::libil2cpp::Type, TValue: quest_hook::libil2cpp::Type>
    AsRef<
        crate::Unity::Properties::INamedProperties_1<
            crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
        >,
    > for crate::Unity::Properties::KeyValuePairPropertyBag_2<TKey, TValue>
{
    fn as_ref(
        &self,
    ) -> &crate::Unity::Properties::INamedProperties_1<
        crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+KeyValuePairPropertyBag_2")]
impl<TKey: quest_hook::libil2cpp::Type, TValue: quest_hook::libil2cpp::Type>
    AsMut<
        crate::Unity::Properties::INamedProperties_1<
            crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
        >,
    > for crate::Unity::Properties::KeyValuePairPropertyBag_2<TKey, TValue>
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::INamedProperties_1<
        crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
