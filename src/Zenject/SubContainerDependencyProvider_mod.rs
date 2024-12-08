#[cfg(feature = "Zenject+SubContainerDependencyProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerDependencyProvider {
    __cordl_parent: crate::System::Object,
    pub _subContainerCreator: *mut crate::Zenject::ISubContainerCreator,
    pub _dependencyType: *mut crate::System::Type,
    pub _identifier: *mut crate::System::Object,
    pub _resolveAll: bool,
}
#[cfg(feature = "Zenject+SubContainerDependencyProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SubContainerDependencyProvider =>
    "Zenject"."SubContainerDependencyProvider"
);
#[cfg(feature = "Zenject+SubContainerDependencyProvider")]
impl std::ops::Deref for crate::Zenject::SubContainerDependencyProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerDependencyProvider")]
impl std::ops::DerefMut for crate::Zenject::SubContainerDependencyProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerDependencyProvider")]
impl crate::Zenject::SubContainerDependencyProvider {
    pub fn _ctor(
        &mut self,
        dependencyType: *mut crate::System::Type,
        identifier: *mut crate::System::Object,
        subContainerCreator: *mut crate::Zenject::ISubContainerCreator,
        resolveAll: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (dependencyType, identifier, subContainerCreator, resolveAll),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_TypeVariesBasedOnMemberType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_TypeVariesBasedOnMemberType", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateSubContext(
        &mut self,
        parent: *mut crate::Zenject::InjectContext,
        subContainer: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::InjectContext> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::InjectContext = __cordl_object
            .invoke("CreateSubContext", (parent, subContainer))?;
        Ok(__cordl_ret)
    }
    pub fn GetAllInstancesWithInjectSplit(
        &mut self,
        context: *mut crate::Zenject::InjectContext,
        args: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        injectAction: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Action>,
        buffer: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetAllInstancesWithInjectSplit",
                (context, args, injectAction, buffer),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsCached(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCached", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetInstanceType(
        &mut self,
        context: *mut crate::Zenject::InjectContext,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetInstanceType", (context))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        dependencyType: *mut crate::System::Type,
        identifier: *mut crate::System::Object,
        subContainerCreator: *mut crate::Zenject::ISubContainerCreator,
        resolveAll: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (dependencyType, identifier, subContainerCreator, resolveAll),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+SubContainerDependencyProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::SubContainerDependencyProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
