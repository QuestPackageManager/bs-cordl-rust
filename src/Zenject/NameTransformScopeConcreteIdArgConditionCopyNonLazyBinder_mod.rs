#[cfg(feature = "Zenject+NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder {
    __cordl_parent: crate::Zenject::TransformScopeConcreteIdArgConditionCopyNonLazyBinder,
}
#[cfg(feature = "Zenject+NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder => "Zenject"
    ."NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder"
);
#[cfg(feature = "Zenject+NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder")]
impl std::ops::Deref
for crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder {
    type Target = crate::Zenject::TransformScopeConcreteIdArgConditionCopyNonLazyBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder")]
impl std::ops::DerefMut
for crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder")]
impl crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder {
    pub fn New(
        bindInfo: *mut crate::Zenject::BindInfo,
        gameObjectInfo: *mut crate::Zenject::GameObjectCreationParameters,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo, gameObjectInfo))?;
        Ok(__cordl_object)
    }
    pub fn WithGameObjectName(
        &mut self,
        gameObjectName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::TransformScopeConcreteIdArgConditionCopyNonLazyBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::TransformScopeConcreteIdArgConditionCopyNonLazyBinder = __cordl_object
            .invoke("WithGameObjectName", (gameObjectName))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        bindInfo: *mut crate::Zenject::BindInfo,
        gameObjectInfo: *mut crate::Zenject::GameObjectCreationParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindInfo, gameObjectInfo))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::NameTransformScopeConcreteIdArgConditionCopyNonLazyBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
