#[cfg(feature = "Zenject+SubContainerCreatorByMethodBase")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerCreatorByMethodBase {
    __cordl_parent: crate::System::Object,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _containerBindInfo: *mut crate::Zenject::SubContainerCreatorBindInfo,
}
#[cfg(feature = "Zenject+SubContainerCreatorByMethodBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SubContainerCreatorByMethodBase =>
    "Zenject"."SubContainerCreatorByMethodBase"
);
#[cfg(feature = "Zenject+SubContainerCreatorByMethodBase")]
impl std::ops::Deref for crate::Zenject::SubContainerCreatorByMethodBase {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByMethodBase")]
impl std::ops::DerefMut for crate::Zenject::SubContainerCreatorByMethodBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByMethodBase")]
impl crate::Zenject::SubContainerCreatorByMethodBase {
    pub fn CreateEmptySubContainer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::DiContainer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::DiContainer = __cordl_object
            .invoke("CreateEmptySubContainer", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateSubContainer(
        &mut self,
        args: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        context: *mut crate::Zenject::InjectContext,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::DiContainer> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::DiContainer = __cordl_object
            .invoke("CreateSubContainer", (args, context))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        container: *mut crate::Zenject::DiContainer,
        containerBindInfo: *mut crate::Zenject::SubContainerCreatorBindInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container, containerBindInfo))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
        containerBindInfo: *mut crate::Zenject::SubContainerCreatorBindInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (container, containerBindInfo))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByMethodBase")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::SubContainerCreatorByMethodBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}