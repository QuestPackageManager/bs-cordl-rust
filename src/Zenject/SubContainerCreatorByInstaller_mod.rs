#[cfg(feature = "Zenject+SubContainerCreatorByInstaller")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerCreatorByInstaller {
    __cordl_parent: crate::System::Object,
    pub _installerType: *mut crate::System::Type,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _extraArgs: *mut crate::System::Collections::Generic::List_1<
        crate::Zenject::TypeValuePair,
    >,
    pub _containerBindInfo: *mut crate::Zenject::SubContainerCreatorBindInfo,
}
#[cfg(feature = "Zenject+SubContainerCreatorByInstaller")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SubContainerCreatorByInstaller =>
    "Zenject"."SubContainerCreatorByInstaller"
);
#[cfg(feature = "Zenject+SubContainerCreatorByInstaller")]
impl std::ops::Deref for crate::Zenject::SubContainerCreatorByInstaller {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByInstaller")]
impl std::ops::DerefMut for crate::Zenject::SubContainerCreatorByInstaller {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByInstaller")]
impl crate::Zenject::SubContainerCreatorByInstaller {
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
    pub fn New_DiContainer_SubContainerCreatorBindInfo_Type1(
        container: *mut crate::Zenject::DiContainer,
        containerBindInfo: *mut crate::Zenject::SubContainerCreatorBindInfo,
        installerType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container, containerBindInfo, installerType))?;
        Ok(__cordl_object)
    }
    pub fn New_IEnumerable_1_0(
        container: *mut crate::Zenject::DiContainer,
        containerBindInfo: *mut crate::Zenject::SubContainerCreatorBindInfo,
        installerType: *mut crate::System::Type,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::Zenject::TypeValuePair,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (container, containerBindInfo, installerType, extraArgs),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor_DiContainer_SubContainerCreatorBindInfo_Type1(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
        containerBindInfo: *mut crate::Zenject::SubContainerCreatorBindInfo,
        installerType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (container, containerBindInfo, installerType))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IEnumerable_1_0(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
        containerBindInfo: *mut crate::Zenject::SubContainerCreatorBindInfo,
        installerType: *mut crate::System::Type,
        extraArgs: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::Zenject::TypeValuePair,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (container, containerBindInfo, installerType, extraArgs))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByInstaller")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::SubContainerCreatorByInstaller {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
