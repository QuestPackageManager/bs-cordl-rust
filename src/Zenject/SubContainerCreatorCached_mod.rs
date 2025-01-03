#[cfg(feature = "Zenject+SubContainerCreatorCached")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerCreatorCached {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _subCreator: quest_hook::libil2cpp::Gc<crate::Zenject::ISubContainerCreator>,
    pub _isLookingUp: bool,
    pub _subContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
}
#[cfg(feature = "Zenject+SubContainerCreatorCached")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SubContainerCreatorCached => "Zenject"
    ."SubContainerCreatorCached"
);
#[cfg(feature = "Zenject+SubContainerCreatorCached")]
impl std::ops::Deref for crate::Zenject::SubContainerCreatorCached {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorCached")]
impl std::ops::DerefMut for crate::Zenject::SubContainerCreatorCached {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorCached")]
impl crate::Zenject::SubContainerCreatorCached {
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
        subCreator: quest_hook::libil2cpp::Gc<crate::Zenject::ISubContainerCreator>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (subCreator))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        subCreator: quest_hook::libil2cpp::Gc<crate::Zenject::ISubContainerCreator>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (subCreator))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorCached")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::SubContainerCreatorCached {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorCached")]
impl AsRef<crate::Zenject::ISubContainerCreator>
for crate::Zenject::SubContainerCreatorCached {
    fn as_ref(&self) -> &crate::Zenject::ISubContainerCreator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorCached")]
impl AsMut<crate::Zenject::ISubContainerCreator>
for crate::Zenject::SubContainerCreatorCached {
    fn as_mut(&mut self) -> &mut crate::Zenject::ISubContainerCreator {
        unsafe { std::mem::transmute(self) }
    }
}
