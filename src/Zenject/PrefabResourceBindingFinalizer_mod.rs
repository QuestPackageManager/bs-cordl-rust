#[cfg(feature = "cordl_class_Zenject+PrefabResourceBindingFinalizer")]
#[repr(C)]
#[derive(Debug)]
pub struct PrefabResourceBindingFinalizer {
    __cordl_parent: crate::Zenject::ProviderBindingFinalizer,
    pub _gameObjectBindInfo:
        quest_hook::libil2cpp::Gc<crate::Zenject::GameObjectCreationParameters>,
    pub _resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _providerFactory: quest_hook::libil2cpp::Gc<
        crate::System::Func_3<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
            quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabInstantiator>,
            quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
        >,
    >,
}
#[cfg(feature = "cordl_class_Zenject+PrefabResourceBindingFinalizer")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::PrefabResourceBindingFinalizer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "PrefabResourceBindingFinalizer";
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
#[cfg(feature = "Zenject+PrefabResourceBindingFinalizer")]
impl std::ops::Deref for crate::Zenject::PrefabResourceBindingFinalizer {
    type Target = crate::Zenject::ProviderBindingFinalizer;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PrefabResourceBindingFinalizer")]
impl std::ops::DerefMut for crate::Zenject::PrefabResourceBindingFinalizer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PrefabResourceBindingFinalizer")]
impl crate::Zenject::PrefabResourceBindingFinalizer {
    pub fn FinalizeBindingConcrete(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        concreteTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                quest_hook::libil2cpp::Gc<crate::System::Type>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "FinalizeBindingConcrete"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FinalizeBindingConcrete",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (container, concreteTypes))? };
        Ok(__cordl_ret.into())
    }
    pub fn FinalizeBindingSelf(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("FinalizeBindingSelf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "FinalizeBindingSelf", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (container))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        gameObjectBindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::GameObjectCreationParameters>,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        providerFactory: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabInstantiator>,
                quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (bindInfo, gameObjectBindInfo, resourcePath, providerFactory),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn OnFinalizeBinding(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnFinalizeBinding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnFinalizeBinding", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (container))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        gameObjectBindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::GameObjectCreationParameters>,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        providerFactory: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
                quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabInstantiator>,
                quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
                        quest_hook::libil2cpp::Gc<crate::Zenject::GameObjectCreationParameters>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Func_3<
                                quest_hook::libil2cpp::Gc<crate::System::Type>,
                                quest_hook::libil2cpp::Gc<crate::Zenject::IPrefabInstantiator>,
                                quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
                            >,
                        >,
                    ), quest_hook::libil2cpp::Void, 4usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (bindInfo, gameObjectBindInfo, resourcePath, providerFactory),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Zenject+PrefabResourceBindingFinalizer")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::PrefabResourceBindingFinalizer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
