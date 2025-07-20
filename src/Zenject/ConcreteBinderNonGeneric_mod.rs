#[cfg(feature = "Zenject+ConcreteBinderNonGeneric")]
#[repr(C)]
#[derive(Debug)]
pub struct ConcreteBinderNonGeneric {
    __cordl_parent: crate::Zenject::FromBinderNonGeneric,
}
#[cfg(feature = "Zenject+ConcreteBinderNonGeneric")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::ConcreteBinderNonGeneric {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "ConcreteBinderNonGeneric";
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
#[cfg(feature = "Zenject+ConcreteBinderNonGeneric")]
impl std::ops::Deref for crate::Zenject::ConcreteBinderNonGeneric {
    type Target = crate::Zenject::FromBinderNonGeneric;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConcreteBinderNonGeneric")]
impl std::ops::DerefMut for crate::Zenject::ConcreteBinderNonGeneric {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConcreteBinderNonGeneric")]
impl crate::Zenject::ConcreteBinderNonGeneric {
    pub fn New(
        bindContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        bindStatement: quest_hook::libil2cpp::Gc<crate::Zenject::BindStatement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindContainer, bindInfo, bindStatement))?;
        Ok(__cordl_object.into())
    }
    pub fn ToSelf(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::FromBinderNonGeneric>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::Zenject::FromBinderNonGeneric>,
                        0usize,
                    >("ToSelf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "ToSelf",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FromBinderNonGeneric,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn To_0<TConcrete>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::FromBinderNonGeneric>,
    >
    where
        TConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::Zenject::FromBinderNonGeneric>,
                        0usize,
                    >("To")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "To",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FromBinderNonGeneric,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn To_Action_1_3(
        &mut self,
        generator: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::ConventionSelectTypesBinder>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::FromBinderNonGeneric>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::Zenject::ConventionSelectTypesBinder,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<crate::Zenject::FromBinderNonGeneric>,
                        1usize,
                    >("To")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "To",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FromBinderNonGeneric,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (generator))? };
        Ok(__cordl_ret.into())
    }
    pub fn To_IEnumerable_1_2(
        &mut self,
        concreteTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::FromBinderNonGeneric>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<crate::System::Type>,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<crate::Zenject::FromBinderNonGeneric>,
                        1usize,
                    >("To")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "To",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FromBinderNonGeneric,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (concreteTypes))? };
        Ok(__cordl_ret.into())
    }
    pub fn To_Il2CppArray1(
        &mut self,
        concreteTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::FromBinderNonGeneric>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::System::Type>,
                            >,
                        >),
                        quest_hook::libil2cpp::Gc<crate::Zenject::FromBinderNonGeneric>,
                        1usize,
                    >("To")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "To",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FromBinderNonGeneric,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (concreteTypes))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ToSelf_b__1_0(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            quest_hook::libil2cpp::Gc<crate::System::Type>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
                        2usize,
                    >("<ToSelf>b__1_0")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "<ToSelf>b__1_0", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider> = unsafe {
            cordl_method_info.invoke_unchecked(self, (container, _cordl_type))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        bindContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        bindStatement: quest_hook::libil2cpp::Gc<crate::Zenject::BindStatement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
                            quest_hook::libil2cpp::Gc<crate::Zenject::BindStatement>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (bindContainer, bindInfo, bindStatement))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+ConcreteBinderNonGeneric")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ConcreteBinderNonGeneric {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
