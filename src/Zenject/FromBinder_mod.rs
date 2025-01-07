#[cfg(feature = "Zenject+FromBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct FromBinder {
    __cordl_parent: crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
    pub _BindContainer_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Zenject::DiContainer,
    >,
    pub _BindStatement_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Zenject::BindStatement,
    >,
}
#[cfg(feature = "Zenject+FromBinder")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::FromBinder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "FromBinder";
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
#[cfg(feature = "Zenject+FromBinder")]
impl std::ops::Deref for crate::Zenject::FromBinder {
    type Target = crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FromBinder")]
impl std::ops::DerefMut for crate::Zenject::FromBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FromBinder")]
impl crate::Zenject::FromBinder {
    pub fn FromComponentInChildren(
        &mut self,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromComponentInChildren", (includeInactive))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentInHierarchy(
        &mut self,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromComponentInHierarchy", (includeInactive))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentInNewPrefabResource_GameObjectCreationParameters1(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        gameObjectInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object
            .invoke("FromComponentInNewPrefabResource", (resourcePath, gameObjectInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentInNewPrefabResource_Il2CppString0(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromComponentInNewPrefabResource", (resourcePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentInNewPrefab_GameObjectCreationParameters1(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        gameObjectInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromComponentInNewPrefab", (prefab, gameObjectInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentInNewPrefab_Object0(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromComponentInNewPrefab", (prefab))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentInParents(
        &mut self,
        excludeSelf: bool,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object
            .invoke("FromComponentInParents", (excludeSelf, includeInactive))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentOnRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromComponentOnRoot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentOn_Func_2_1(
        &mut self,
        gameObjectGetter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromComponentOn", (gameObjectGetter))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentOn_GameObject0(
        &mut self,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromComponentOn", (gameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentSibling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromComponentSibling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentsInChildrenBase(
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object
            .invoke(
                "FromComponentsInChildrenBase",
                (excludeSelf, predicate, includeInactive),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentsInHierarchyBase(
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object
            .invoke("FromComponentsInHierarchyBase", (predicate, includeInactive))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentsInNewPrefabResource_GameObjectCreationParameters1(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        gameObjectInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object
            .invoke(
                "FromComponentsInNewPrefabResource",
                (resourcePath, gameObjectInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentsInNewPrefabResource_Il2CppString0(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromComponentsInNewPrefabResource", (resourcePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentsInNewPrefab_GameObjectCreationParameters1(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        gameObjectInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object
            .invoke("FromComponentsInNewPrefab", (prefab, gameObjectInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentsInNewPrefab_Object0(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromComponentsInNewPrefab", (prefab))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentsInParents(
        &mut self,
        excludeSelf: bool,
        includeInactive: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object
            .invoke("FromComponentsInParents", (excludeSelf, includeInactive))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentsOnRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromComponentsOnRoot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentsOn_Func_2_1(
        &mut self,
        gameObjectGetter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromComponentsOn", (gameObjectGetter))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentsOn_GameObject0(
        &mut self,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromComponentsOn", (gameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromComponentsSibling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromComponentsSibling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FromIFactoryBase<TContract>(
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromIFactoryBase", (factoryBindGenerator))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromInstanceBase(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromInstanceBase", (instance))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromMethodBase<TConcrete>(
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromMethodBase", (method))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromMethodMultipleBase<TConcrete>(
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromMethodMultipleBase", (method))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromMethodMultipleUntyped(
        &mut self,
        method: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IEnumerable_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    >,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromMethodMultipleUntyped", (method))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromMethodUntyped(
        &mut self,
        method: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromMethodUntyped", (method))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromNew(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromNew", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FromNewComponentOnNewPrefabResource_GameObjectCreationParameters1(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        gameObjectInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object
            .invoke(
                "FromNewComponentOnNewPrefabResource",
                (resourcePath, gameObjectInfo),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FromNewComponentOnNewPrefabResource_Il2CppString0(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object
            .invoke("FromNewComponentOnNewPrefabResource", (resourcePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromNewComponentOnNewPrefab_GameObjectCreationParameters1(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        gameObjectInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object
            .invoke("FromNewComponentOnNewPrefab", (prefab, gameObjectInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromNewComponentOnNewPrefab_Object0(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromNewComponentOnNewPrefab", (prefab))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromNewComponentOnRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromNewComponentOnRoot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FromNewComponentOn_Func_2_1(
        &mut self,
        gameObjectGetter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
                quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromNewComponentOn", (gameObjectGetter))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromNewComponentOn_GameObject0(
        &mut self,
        gameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromNewComponentOn", (gameObject))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromNewComponentSibling(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromNewComponentSibling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FromNewScriptableObject(
        &mut self,
        resource: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromNewScriptableObject", (resource))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromNewScriptableObjectResource(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromNewScriptableObjectResource", (resourcePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromResolveAll_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromResolveAll", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FromResolveAll_Il2CppObject1(
        &mut self,
        subIdentifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromResolveAll", (subIdentifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromResolveAll_Il2CppObject_InjectSources2(
        &mut self,
        subIdentifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        source: crate::Zenject::InjectSources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromResolveAll", (subIdentifier, source))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromResolveGetterBase<TObj, TResult>(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TResult>>,
        source: crate::Zenject::InjectSources,
        matchMultiple: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    >
    where
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object
            .invoke(
                "FromResolveGetterBase",
                (identifier, method, source, matchMultiple),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn FromResolveInternal(
        &mut self,
        subIdentifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        matchAll: bool,
        source: crate::Zenject::InjectSources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object
            .invoke("FromResolveInternal", (subIdentifier, matchAll, source))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromResolve_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromResolve", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FromResolve_Il2CppObject1(
        &mut self,
        subIdentifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromResolve", (subIdentifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromResolve_Il2CppObject_InjectSources2(
        &mut self,
        subIdentifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        source: crate::Zenject::InjectSources,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromResolve", (subIdentifier, source))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromResource(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromResource", (resourcePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromResources(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromResources", (resourcePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromScriptableObject(
        &mut self,
        resource: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromScriptableObject", (resource))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromScriptableObjectInternal(
        &mut self,
        resource: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
        createNew: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object
            .invoke("FromScriptableObjectInternal", (resource, createNew))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromScriptableObjectResource(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object.invoke("FromScriptableObjectResource", (resourcePath))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromScriptableObjectResourceInternal(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        createNew: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ScopeConcreteIdArgConditionCopyNonLazyBinder,
        > = __cordl_object
            .invoke("FromScriptableObjectResourceInternal", (resourcePath, createNew))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromSubContainerResolveAll_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::SubContainerBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::SubContainerBinder> = __cordl_object
            .invoke("FromSubContainerResolveAll", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FromSubContainerResolveAll_Il2CppObject1(
        &mut self,
        subIdentifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::SubContainerBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::SubContainerBinder> = __cordl_object
            .invoke("FromSubContainerResolveAll", (subIdentifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromSubContainerResolveInternal(
        &mut self,
        subIdentifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        resolveAll: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::SubContainerBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::SubContainerBinder> = __cordl_object
            .invoke("FromSubContainerResolveInternal", (subIdentifier, resolveAll))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromSubContainerResolve_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::SubContainerBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::SubContainerBinder> = __cordl_object
            .invoke("FromSubContainerResolve", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FromSubContainerResolve_Il2CppObject1(
        &mut self,
        subIdentifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::SubContainerBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::SubContainerBinder> = __cordl_object
            .invoke("FromSubContainerResolve", (subIdentifier))?;
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
    pub fn _FromNewComponentSibling_b__37_0(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider> = __cordl_object
            .invoke("<FromNewComponentSibling>b__37_0", (container, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        bindContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        bindStatement: quest_hook::libil2cpp::Gc<crate::Zenject::BindStatement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindContainer, bindInfo, bindStatement))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AllParentTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        > = __cordl_object.invoke("get_AllParentTypes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BindContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer> = __cordl_object
            .invoke("get_BindContainer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BindStatement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::BindStatement>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::BindStatement> = __cordl_object
            .invoke("get_BindStatement", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ConcreteTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        > = __cordl_object.invoke("get_ConcreteTypes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_BindContainer(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BindContainer", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_BindStatement(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Zenject::BindStatement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BindStatement", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SubFinalizer(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Zenject::IBindingFinalizer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SubFinalizer", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+FromBinder")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::FromBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
