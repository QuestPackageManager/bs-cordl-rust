#[cfg(feature = "Zenject+FromBinderNonGeneric")]
#[repr(C)]
#[derive(Debug)]
pub struct FromBinderNonGeneric {
    __cordl_parent: crate::Zenject::FromBinder,
}
#[cfg(feature = "Zenject+FromBinderNonGeneric")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::FromBinderNonGeneric {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "FromBinderNonGeneric";
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
#[cfg(feature = "Zenject+FromBinderNonGeneric")]
impl std::ops::Deref for crate::Zenject::FromBinderNonGeneric {
    type Target = crate::Zenject::FromBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FromBinderNonGeneric")]
impl std::ops::DerefMut for crate::Zenject::FromBinderNonGeneric {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FromBinderNonGeneric")]
impl crate::Zenject::FromBinderNonGeneric {
    pub fn FromComponentsInChildren_Func_2__cordl_bool0(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
                bool,
            >,
        >,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Func_2<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
                            bool,
                        >,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
                >,
                2usize,
            >("FromComponentsInChildren")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type
                    > ::class(), "FromComponentsInChildren", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked(self, (predicate, includeInactive))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentsInChildren__cordl_bool_Func_2__cordl_bool1(
        &mut self,
        excludeSelf: bool,
        predicate: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
                bool,
            >,
        >,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    bool,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Func_2<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
                            bool,
                        >,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
                >,
                3usize,
            >("FromComponentsInChildren")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type
                    > ::class(), "FromComponentsInChildren", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe {
            method.invoke_unchecked(self, (excludeSelf, predicate, includeInactive))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentsInHierarchy(
        &mut self,
        predicate: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
                bool,
            >,
        >,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Func_2<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Component>,
                            bool,
                        >,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
                >,
                2usize,
            >("FromComponentsInHierarchy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type
                    > ::class(), "FromComponentsInHierarchy", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked(self, (predicate, includeInactive))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromFactory<TConcrete, TFactory>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TFactory: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
                >,
                0usize,
            >("FromFactory")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type
                    > ::class(), "FromFactory", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn FromIFactory<TContract>(
        &mut self,
        factoryBindGenerator: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::ConcreteBinderGeneric_1<
                        quest_hook::libil2cpp::Gc<crate::Zenject::IFactory_1<TContract>>,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Zenject::ConcreteBinderGeneric_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::Zenject::IFactory_1<TContract>,
                                >,
                            >,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
                >,
                1usize,
            >("FromIFactory")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type
                    > ::class(), "FromIFactory", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked(self, (factoryBindGenerator))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromInstance(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
                >,
                1usize,
            >("FromInstance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type
                    > ::class(), "FromInstance", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked(self, (instance))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromMethod<TConcrete>(
        &mut self,
        method: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                TConcrete,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Func_2<
                        quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                        TConcrete,
                    >,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
                >,
                1usize,
            >("FromMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type
                    > ::class(), "FromMethod", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked(self, (method))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromMethodMultiple<TConcrete>(
        &mut self,
        method: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<TConcrete>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Func_2<
                        quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<TConcrete>,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
                >,
                1usize,
            >("FromMethodMultiple")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type
                    > ::class(), "FromMethodMultiple", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked(self, (method))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromResolveAllGetter_Func_2_0<TObj, TContract>(
        &mut self,
        method: quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TContract>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TContract>>),
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
                >,
                1usize,
            >("FromResolveAllGetter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type
                    > ::class(), "FromResolveAllGetter", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked(self, (method))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromResolveAllGetter_Il2CppObject_Func_2_1<TObj, TContract>(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TContract>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TContract>>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
                >,
                2usize,
            >("FromResolveAllGetter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type
                    > ::class(), "FromResolveAllGetter", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked(self, (identifier, method))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromResolveAllGetter_Il2CppObject_Func_2_InjectSources2<TObj, TContract>(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TContract>>,
        source: crate::Zenject::InjectSources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TContract>>,
                    crate::Zenject::InjectSources,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
                >,
                3usize,
            >("FromResolveAllGetter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type
                    > ::class(), "FromResolveAllGetter", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked(self, (identifier, method, source))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromResolveGetter_Func_2_0<TObj, TContract>(
        &mut self,
        method: quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TContract>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TContract>>),
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
                >,
                1usize,
            >("FromResolveGetter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type
                    > ::class(), "FromResolveGetter", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked(self, (method))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromResolveGetter_Il2CppObject_Func_2_1<TObj, TContract>(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TContract>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TContract>>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
                >,
                2usize,
            >("FromResolveGetter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type
                    > ::class(), "FromResolveGetter", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked(self, (identifier, method))? };
        Ok(__cordl_ret.into())
    }
    pub fn FromResolveGetter_Il2CppObject_Func_2_InjectSources2<TObj, TContract>(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TContract>>,
        source: crate::Zenject::InjectSources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TContract>>,
                    crate::Zenject::InjectSources,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
                >,
                3usize,
            >("FromResolveGetter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type
                    > ::class(), "FromResolveGetter", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = unsafe { method.invoke_unchecked(self, (identifier, method, source))? };
        Ok(__cordl_ret.into())
    }
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
    pub fn _ctor(
        &mut self,
        bindContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        bindStatement: quest_hook::libil2cpp::Gc<crate::Zenject::BindStatement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type>::class()
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
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Zenject::FromBinderNonGeneric as quest_hook::libil2cpp::Type
                    > ::class(), ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bindContainer, bindInfo, bindStatement))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+FromBinderNonGeneric")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::FromBinderNonGeneric {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
