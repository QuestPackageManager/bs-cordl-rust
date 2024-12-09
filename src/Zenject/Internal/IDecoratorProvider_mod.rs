#[cfg(feature = "Zenject+Internal+IDecoratorProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IDecoratorProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+Internal+IDecoratorProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::Internal::IDecoratorProvider =>
    "Zenject.Internal"."IDecoratorProvider"
);
#[cfg(feature = "Zenject+Internal+IDecoratorProvider")]
impl std::ops::Deref for crate::Zenject::Internal::IDecoratorProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+IDecoratorProvider")]
impl std::ops::DerefMut for crate::Zenject::Internal::IDecoratorProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+IDecoratorProvider")]
impl crate::Zenject::Internal::IDecoratorProvider {
    pub fn GetAllInstances(
        &mut self,
        provider: *mut crate::Zenject::IProvider,
        context: *mut crate::Zenject::InjectContext,
        buffer: *mut crate::System::Collections::Generic::List_1<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetAllInstances", (provider, context, buffer))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Zenject+Internal+IDecoratorProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::Internal::IDecoratorProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
