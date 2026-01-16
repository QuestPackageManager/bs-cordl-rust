#[cfg(feature = "cordl_class_Unity+Properties+IKeyedProperties_2")]
#[repr(C)]
#[derive(Debug)]
pub struct IKeyedProperties_2<
    TContainer: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TContainer: std::marker::PhantomData<TContainer>,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
}
#[cfg(feature = "cordl_class_Unity+Properties+IKeyedProperties_2")]
unsafe impl<
    TContainer: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::Unity::Properties::IKeyedProperties_2<TContainer, TKey> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "IKeyedProperties`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Unity.Properties",
                        "IKeyedProperties`2",
                    )
                    .unwrap()
                    .make_generic::<(TContainer, TKey)>()
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
#[cfg(feature = "Unity+Properties+IKeyedProperties_2")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::Unity::Properties::IKeyedProperties_2<TContainer, TKey> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+IKeyedProperties_2")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::Unity::Properties::IKeyedProperties_2<TContainer, TKey> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+IKeyedProperties_2")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::IKeyedProperties_2<TContainer, TKey> {
    pub fn TryGetProperty(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<TContainer>,
        key: TKey,
        property: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty_1<TContainer>>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::ByRefMut<TContainer>,
                            TKey,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::Unity::Properties::IProperty_1<TContainer>,
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
            cordl_method_info.invoke_unchecked(self, (container, key, property))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+IKeyedProperties_2")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::IKeyedProperties_2<TContainer, TKey> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
