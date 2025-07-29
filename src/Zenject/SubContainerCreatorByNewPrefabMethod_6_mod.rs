#[cfg(feature = "cordl_class_Zenject+SubContainerCreatorByNewPrefabMethod_6")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerCreatorByNewPrefabMethod_6<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TParam5: quest_hook::libil2cpp::Type,
    TParam6: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Zenject::SubContainerCreatorByNewPrefabDynamicContext,
    pub _installerMethod: quest_hook::libil2cpp::Gc<
        crate::System::Action_7<
            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
            TParam1,
            TParam2,
            TParam3,
            TParam4,
            TParam5,
            TParam6,
        >,
    >,
    __cordl_phantom_TParam1: std::marker::PhantomData<TParam1>,
    __cordl_phantom_TParam2: std::marker::PhantomData<TParam2>,
    __cordl_phantom_TParam3: std::marker::PhantomData<TParam3>,
    __cordl_phantom_TParam4: std::marker::PhantomData<TParam4>,
    __cordl_phantom_TParam5: std::marker::PhantomData<TParam5>,
    __cordl_phantom_TParam6: std::marker::PhantomData<TParam6>,
}
#[cfg(feature = "cordl_class_Zenject+SubContainerCreatorByNewPrefabMethod_6")]
unsafe impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TParam5: quest_hook::libil2cpp::Type,
    TParam6: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::Zenject::SubContainerCreatorByNewPrefabMethod_6<
    TParam1,
    TParam2,
    TParam3,
    TParam4,
    TParam5,
    TParam6,
> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "SubContainerCreatorByNewPrefabMethod`6";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Zenject",
                        "SubContainerCreatorByNewPrefabMethod`6",
                    )
                    .unwrap()
                    .make_generic::<
                        (TParam1, TParam2, TParam3, TParam4, TParam5, TParam6),
                    >()
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
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabMethod_6")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TParam5: quest_hook::libil2cpp::Type,
    TParam6: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Zenject::SubContainerCreatorByNewPrefabMethod_6<
    TParam1,
    TParam2,
    TParam3,
    TParam4,
    TParam5,
    TParam6,
> {
    type Target = crate::Zenject::SubContainerCreatorByNewPrefabDynamicContext;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabMethod_6")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TParam5: quest_hook::libil2cpp::Type,
    TParam6: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Zenject::SubContainerCreatorByNewPrefabMethod_6<
    TParam1,
    TParam2,
    TParam3,
    TParam4,
    TParam5,
    TParam6,
> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewPrefabMethod_6")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TParam5: quest_hook::libil2cpp::Type,
    TParam6: quest_hook::libil2cpp::Type,
> crate::Zenject::SubContainerCreatorByNewPrefabMethod_6<
    TParam1,
    TParam2,
    TParam3,
    TParam4,
    TParam5,
    TParam6,
> {
    pub fn AddInstallers(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::GameObjectContext>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    crate::Zenject::TypeValuePair,
                                >,
                            >,
                            quest_hook::libil2cpp::Gc<crate::Zenject::GameObjectContext>,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("AddInstallers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "AddInstallers", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (args, context))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        prefabProvider: quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabProvider>,
        gameObjectBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
        installerMethod: quest_hook::libil2cpp::Gc<
            crate::System::Action_7<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                TParam1,
                TParam2,
                TParam3,
                TParam4,
                TParam5,
                TParam6,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (container, prefabProvider, gameObjectBindInfo, installerMethod),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        prefabProvider: quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabProvider>,
        gameObjectBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
        installerMethod: quest_hook::libil2cpp::Gc<
            crate::System::Action_7<
                quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                TParam1,
                TParam2,
                TParam3,
                TParam4,
                TParam5,
                TParam6,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabProvider>,
                            quest_hook::libil2cpp::Gc<
                                crate::Zenject::GameObjectCreationParameters,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Action_7<
                                    quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                                    TParam1,
                                    TParam2,
                                    TParam3,
                                    TParam4,
                                    TParam5,
                                    TParam6,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (container, prefabProvider, gameObjectBindInfo, installerMethod),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Zenject+SubContainerCreatorByNewPrefabMethod_6")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TParam2: quest_hook::libil2cpp::Type,
    TParam3: quest_hook::libil2cpp::Type,
    TParam4: quest_hook::libil2cpp::Type,
    TParam5: quest_hook::libil2cpp::Type,
    TParam6: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Zenject::SubContainerCreatorByNewPrefabMethod_6<
    TParam1,
    TParam2,
    TParam3,
    TParam4,
    TParam5,
    TParam6,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
