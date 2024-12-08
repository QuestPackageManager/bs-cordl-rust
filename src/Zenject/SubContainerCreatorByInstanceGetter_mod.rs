#[cfg(feature = "Zenject+SubContainerCreatorByInstanceGetter")]
#[repr(C)]
#[derive(Debug)]
pub struct SubContainerCreatorByInstanceGetter {
    __cordl_parent: crate::System::Object,
    pub _subcontainerGetter: *mut crate::System::Func_2<
        *mut crate::Zenject::InjectContext,
        *mut crate::Zenject::DiContainer,
    >,
}
#[cfg(feature = "Zenject+SubContainerCreatorByInstanceGetter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SubContainerCreatorByInstanceGetter =>
    "Zenject"."SubContainerCreatorByInstanceGetter"
);
#[cfg(feature = "Zenject+SubContainerCreatorByInstanceGetter")]
impl std::ops::Deref for crate::Zenject::SubContainerCreatorByInstanceGetter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByInstanceGetter")]
impl std::ops::DerefMut for crate::Zenject::SubContainerCreatorByInstanceGetter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByInstanceGetter")]
impl crate::Zenject::SubContainerCreatorByInstanceGetter {
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
        subcontainerGetter: *mut crate::System::Func_2<
            *mut crate::Zenject::InjectContext,
            *mut crate::Zenject::DiContainer,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (subcontainerGetter))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        subcontainerGetter: *mut crate::System::Func_2<
            *mut crate::Zenject::InjectContext,
            *mut crate::Zenject::DiContainer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (subcontainerGetter))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+SubContainerCreatorByInstanceGetter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::SubContainerCreatorByInstanceGetter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
