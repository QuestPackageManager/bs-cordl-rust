#[cfg(feature = "Zenject+SubContainerCreatorByNewGameObjectDynamicContext")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerCreatorByNewGameObjectDynamicContext {
    __cordl_parent: crate::Zenject::SubContainerCreatorDynamicContext,
    pub _gameObjectBindInfo: quest_hook::libil2cpp::Gc<
        crate::Zenject::GameObjectCreationParameters,
    >,
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewGameObjectDynamicContext")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Zenject::SubContainerCreatorByNewGameObjectDynamicContext {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "SubContainerCreatorByNewGameObjectDynamicContext";
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
#[cfg(feature = "Zenject+SubContainerCreatorByNewGameObjectDynamicContext")]
impl std::ops::Deref
for crate::Zenject::SubContainerCreatorByNewGameObjectDynamicContext {
    type Target = crate::Zenject::SubContainerCreatorDynamicContext;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewGameObjectDynamicContext")]
impl std::ops::DerefMut
for crate::Zenject::SubContainerCreatorByNewGameObjectDynamicContext {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewGameObjectDynamicContext")]
impl crate::Zenject::SubContainerCreatorByNewGameObjectDynamicContext {
    pub fn CreateGameObject(
        &mut self,
        shouldMakeActive: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<bool>),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
                        1usize,
                    >("CreateGameObject")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "CreateGameObject", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject> = unsafe {
            method.invoke_unchecked(self, (shouldMakeActive))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        gameObjectBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container, gameObjectBindInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        gameObjectBindInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::GameObjectCreationParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
                            quest_hook::libil2cpp::Gc<
                                crate::Zenject::GameObjectCreationParameters,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (container, gameObjectBindInfo))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByNewGameObjectDynamicContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::SubContainerCreatorByNewGameObjectDynamicContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
