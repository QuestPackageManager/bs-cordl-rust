#[cfg(feature = "Zenject+ConcreteBinderGeneric_1")]
#[repr(C)]
#[derive(Debug)]
pub struct ConcreteBinderGeneric_1<TContract: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::Zenject::FromBinderGeneric_1<TContract>,
    __cordl_phantom_TContract: std::marker::PhantomData<TContract>,
}
#[cfg(feature = "Zenject+ConcreteBinderGeneric_1")]
unsafe impl<TContract: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::Zenject::ConcreteBinderGeneric_1<TContract> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "ConcreteBinderGeneric`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Zenject",
                        "ConcreteBinderGeneric`1",
                    )
                    .unwrap()
                    .make_generic::<(TContract)>()
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
#[cfg(feature = "Zenject+ConcreteBinderGeneric_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::ConcreteBinderGeneric_1<TContract> {
    type Target = crate::Zenject::FromBinderGeneric_1<TContract>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConcreteBinderGeneric_1")]
impl<TContract: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::ConcreteBinderGeneric_1<TContract> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConcreteBinderGeneric_1")]
impl<
    TContract: quest_hook::libil2cpp::Type,
> crate::Zenject::ConcreteBinderGeneric_1<TContract> {
    pub fn New(
        bindContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        bindStatement: quest_hook::libil2cpp::Gc<crate::Zenject::BindStatement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindContainer, bindInfo, bindStatement))?;
        Ok(__cordl_object.into())
    }
    pub fn ToSelf(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::FromBinderGeneric_1<TContract>>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::FromBinderGeneric_1<TContract>,
                >,
                0usize,
            >("ToSelf")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToSelf", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FromBinderGeneric_1<TContract>,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn To_0<TConcrete>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::FromBinderGeneric_1<TConcrete>>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::FromBinderGeneric_1<TConcrete>,
                >,
                0usize,
            >("To")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "To", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FromBinderGeneric_1<TConcrete>,
        > = unsafe { method.invoke_unchecked(self, ()) };
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
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                    self, "To", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FromBinderNonGeneric,
        > = unsafe { method.invoke_unchecked(self, (generator)) };
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
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                    self, "To", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FromBinderNonGeneric,
        > = unsafe { method.invoke_unchecked(self, (concreteTypes)) };
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
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                    self, "To", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::FromBinderNonGeneric,
        > = unsafe { method.invoke_unchecked(self, (concreteTypes)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ToSelf_b__1_0(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                    self, "<ToSelf>b__1_0", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider> = unsafe {
            method.invoke_unchecked(self, (container, _cordl_type))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _To_b__5_0(
        &mut self,
        concreteType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                bool,
                1usize,
            >("<To>b__5_0")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "<To>b__5_0", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (concreteType)) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        bindContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        bindStatement: quest_hook::libil2cpp::Gc<crate::Zenject::BindStatement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bindContainer, bindInfo, bindStatement))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+ConcreteBinderGeneric_1")]
impl<TContract: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::ConcreteBinderGeneric_1<TContract> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
