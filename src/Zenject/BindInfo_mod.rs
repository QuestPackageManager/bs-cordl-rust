#[cfg(feature = "Zenject+BindInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct BindInfo {
    __cordl_parent: crate::System::Object,
    pub MarkAsCreationBinding: bool,
    pub MarkAsUniqueSingleton: bool,
    pub ConcreteIdentifier: *mut crate::System::Object,
    pub SaveProvider: bool,
    pub OnlyBindIfNotBound: bool,
    pub RequireExplicitScope: bool,
    pub Identifier: *mut crate::System::Object,
    pub ContractTypes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Type,
    >,
    pub BindingInheritanceMethod: crate::Zenject::BindingInheritanceMethods,
    pub InvalidBindResponse: crate::Zenject::InvalidBindResponses,
    pub NonLazy: bool,
    pub Condition: *mut crate::Zenject::BindingCondition,
    pub ToChoice: crate::Zenject::ToChoices,
    pub ContextInfo: *mut crate::System::String,
    pub ToTypes: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Type,
    >,
    pub Scope: crate::Zenject::ScopeTypes,
    pub Arguments: *mut crate::System::Collections::Generic::List_1<
        crate::Zenject::TypeValuePair,
    >,
    pub InstantiatedCallback: *mut crate::System::Action_2<
        *mut crate::Zenject::InjectContext,
        *mut crate::System::Object,
    >,
}
#[cfg(feature = "Zenject+BindInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::BindInfo => "Zenject"."BindInfo"
);
#[cfg(feature = "Zenject+BindInfo")]
impl std::ops::Deref for crate::Zenject::BindInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+BindInfo")]
impl std::ops::DerefMut for crate::Zenject::BindInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+BindInfo")]
impl crate::Zenject::BindInfo {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetContextInfo(
        &mut self,
        contextInfo: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetContextInfo", (contextInfo))?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+BindInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::BindInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
