#[cfg(feature = "System+Collections+Generic+CollectionExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct CollectionExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Collections+Generic+CollectionExtensions")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Collections::Generic::CollectionExtensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Collections.Generic";
    const CLASS_NAME: &'static str = "CollectionExtensions";
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
#[cfg(feature = "System+Collections+Generic+CollectionExtensions")]
impl std::ops::Deref for crate::System::Collections::Generic::CollectionExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+CollectionExtensions")]
impl std::ops::DerefMut for crate::System::Collections::Generic::CollectionExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+CollectionExtensions")]
impl crate::System::Collections::Generic::CollectionExtensions {
    pub fn GetValueOrDefault_IReadOnlyDictionary_2_TKey0<TKey, TValue>(
        dictionary: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyDictionary_2<TKey, TValue>,
        >,
        key: TKey,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IReadOnlyDictionary_2<
                                    TKey,
                                    TValue,
                                >,
                            >,
                            TKey,
                        ),
                        TValue,
                        2usize,
                    >("GetValueOrDefault")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetValueOrDefault", 2usize
                        )
                    })
            });
        let __cordl_ret: TValue = unsafe {
            method.invoke_unchecked((), (dictionary, key))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetValueOrDefault_TValue1<TKey, TValue>(
        dictionary: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyDictionary_2<TKey, TValue>,
        >,
        key: TKey,
        defaultValue: TValue,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IReadOnlyDictionary_2<
                                    TKey,
                                    TValue,
                                >,
                            >,
                            TKey,
                            TValue,
                        ),
                        TValue,
                        3usize,
                    >("GetValueOrDefault")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "GetValueOrDefault", 3usize
                        )
                    })
            });
        let __cordl_ret: TValue = unsafe {
            method.invoke_unchecked((), (dictionary, key, defaultValue))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Collections+Generic+CollectionExtensions")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Generic::CollectionExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
