#[cfg(feature = "Zenject+SubContainerCreatorDynamicContext")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerCreatorDynamicContext {
    __cordl_parent: crate::System::Object,
    pub _container: *mut crate::Zenject::DiContainer,
}
#[cfg(feature = "Zenject+SubContainerCreatorDynamicContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SubContainerCreatorDynamicContext =>
    "Zenject"."SubContainerCreatorDynamicContext"
);
#[cfg(feature = "Zenject+SubContainerCreatorDynamicContext")]
impl std::ops::Deref for crate::Zenject::SubContainerCreatorDynamicContext {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorDynamicContext")]
impl std::ops::DerefMut for crate::Zenject::SubContainerCreatorDynamicContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorDynamicContext")]
impl crate::Zenject::SubContainerCreatorDynamicContext {
    pub fn get_Container(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::DiContainer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::DiContainer = __cordl_object
            .invoke("get_Container", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddInstallers(
        &mut self,
        args: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        context: *mut crate::Zenject::GameObjectContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddInstallers", (args, context))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (container))?;
        Ok(__cordl_ret)
    }
    pub fn CreateSubContainer(
        &mut self,
        args: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        parentContext: *mut crate::Zenject::InjectContext,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::DiContainer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::DiContainer = __cordl_object
            .invoke("CreateSubContainer", (args, parentContext))?;
        Ok(__cordl_ret)
    }
    pub fn CreateGameObject(
        &mut self,
        shouldMakeActive: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::GameObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::GameObject = __cordl_object
            .invoke("CreateGameObject", (shouldMakeActive))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorDynamicContext")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::SubContainerCreatorDynamicContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
