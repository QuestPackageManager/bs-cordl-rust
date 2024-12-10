#[cfg(feature = "Zenject+ProviderBindingFinalizer")]
#[repr(C)]
#[derive(Debug)]
pub struct ProviderBindingFinalizer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _BindInfo_k__BackingField: *mut crate::Zenject::BindInfo,
}
#[cfg(feature = "Zenject+ProviderBindingFinalizer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ProviderBindingFinalizer => "Zenject"
    ."ProviderBindingFinalizer"
);
#[cfg(feature = "Zenject+ProviderBindingFinalizer")]
impl std::ops::Deref for crate::Zenject::ProviderBindingFinalizer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ProviderBindingFinalizer")]
impl std::ops::DerefMut for crate::Zenject::ProviderBindingFinalizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ProviderBindingFinalizer")]
impl crate::Zenject::ProviderBindingFinalizer {
    #[cfg(feature = "Zenject+ProviderBindingFinalizer+__c")]
    pub type __c = crate::Zenject::ProviderBindingFinalizer___c;
    pub fn FinalizeBinding(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinalizeBinding", (container))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetScope(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Zenject::ScopeTypes> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Zenject::ScopeTypes = __cordl_object
            .invoke("GetScope", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn OnFinalizeBinding(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFinalizeBinding", (container))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterProviderForAllContracts(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        provider: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterProviderForAllContracts", (container, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterProviderPerContract(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        providerFunc: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                *mut crate::Zenject::DiContainer,
                *mut crate::System::Type,
                *mut crate::Zenject::IProvider,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterProviderPerContract", (container, providerFunc))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterProvider_IProvider0<TContract>(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        provider: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContract: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterProvider", (container, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterProvider_Type_IProvider1(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        contractType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        provider: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterProvider", (container, contractType, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterProvidersForAllContractsPerConcreteType(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        concreteTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<*mut crate::System::Type>,
        >,
        providerFunc: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                *mut crate::Zenject::DiContainer,
                *mut crate::System::Type,
                *mut crate::Zenject::IProvider,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RegisterProvidersForAllContractsPerConcreteType",
                (container, concreteTypes, providerFunc),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterProvidersPerContractAndConcreteType(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        concreteTypes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<*mut crate::System::Type>,
        >,
        providerFunc: quest_hook::libil2cpp::Gc<
            crate::System::Func_3<
                *mut crate::System::Type,
                *mut crate::System::Type,
                *mut crate::Zenject::IProvider,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "RegisterProvidersPerContractAndConcreteType",
                (container, concreteTypes, providerFunc),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateBindTypes(
        &mut self,
        concreteType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        contractType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ValidateBindTypes", (concreteType, contractType))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BindInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo> = __cordl_object
            .invoke("get_BindInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BindingInheritanceMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Zenject::BindingInheritanceMethods> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Zenject::BindingInheritanceMethods = __cordl_object
            .invoke("get_BindingInheritanceMethod", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_BindInfo(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BindInfo", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+ProviderBindingFinalizer")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ProviderBindingFinalizer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+ProviderBindingFinalizer")]
impl AsRef<crate::Zenject::IBindingFinalizer>
for crate::Zenject::ProviderBindingFinalizer {
    fn as_ref(&self) -> &crate::Zenject::IBindingFinalizer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+ProviderBindingFinalizer")]
impl AsMut<crate::Zenject::IBindingFinalizer>
for crate::Zenject::ProviderBindingFinalizer {
    fn as_mut(&mut self) -> &mut crate::Zenject::IBindingFinalizer {
        unsafe { std::mem::transmute(self) }
    }
}
