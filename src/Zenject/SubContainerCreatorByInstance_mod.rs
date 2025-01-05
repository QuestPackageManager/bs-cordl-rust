#[cfg(feature = "Zenject+SubContainerCreatorByInstance")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerCreatorByInstance {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _subcontainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
}
#[cfg(feature = "Zenject+SubContainerCreatorByInstance")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SubContainerCreatorByInstance =>
    "Zenject"."SubContainerCreatorByInstance"
);
#[cfg(feature = "Zenject+SubContainerCreatorByInstance")]
impl std::ops::Deref for crate::Zenject::SubContainerCreatorByInstance {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByInstance")]
impl std::ops::DerefMut for crate::Zenject::SubContainerCreatorByInstance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByInstance")]
impl crate::Zenject::SubContainerCreatorByInstance {
    pub fn CreateSubContainer(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer> = __cordl_object
            .invoke("CreateSubContainer", (args, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        subcontainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (subcontainer))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        subcontainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (subcontainer))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByInstance")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::SubContainerCreatorByInstance {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByInstance")]
impl AsRef<crate::Zenject::ISubContainerCreator>
for crate::Zenject::SubContainerCreatorByInstance {
    fn as_ref(&self) -> &crate::Zenject::ISubContainerCreator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByInstance")]
impl AsMut<crate::Zenject::ISubContainerCreator>
for crate::Zenject::SubContainerCreatorByInstance {
    fn as_mut(&mut self) -> &mut crate::Zenject::ISubContainerCreator {
        unsafe { std::mem::transmute(self) }
    }
}
