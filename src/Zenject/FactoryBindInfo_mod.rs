#[cfg(feature = "Zenject+FactoryBindInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct FactoryBindInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _FactoryType_k__BackingField: *mut crate::System::Type,
    pub _ProviderFunc_k__BackingField: *mut crate::System::Func_2<
        *mut crate::Zenject::DiContainer,
        *mut crate::Zenject::IProvider,
    >,
    pub _Arguments_k__BackingField: *mut crate::System::Collections::Generic::List_1<
        crate::Zenject::TypeValuePair,
    >,
}
#[cfg(feature = "Zenject+FactoryBindInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FactoryBindInfo => "Zenject"
    ."FactoryBindInfo"
);
#[cfg(feature = "Zenject+FactoryBindInfo")]
impl std::ops::Deref for crate::Zenject::FactoryBindInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryBindInfo")]
impl std::ops::DerefMut for crate::Zenject::FactoryBindInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryBindInfo")]
impl crate::Zenject::FactoryBindInfo {
    pub fn New(
        factoryType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (factoryType))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        factoryType: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (factoryType))?;
        Ok(__cordl_ret)
    }
    pub fn get_Arguments(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        > = __cordl_object.invoke("get_Arguments", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_FactoryType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_FactoryType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ProviderFunc(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Func_2<
            *mut crate::Zenject::DiContainer,
            *mut crate::Zenject::IProvider,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Func_2<
            *mut crate::Zenject::DiContainer,
            *mut crate::Zenject::IProvider,
        > = __cordl_object.invoke("get_ProviderFunc", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Arguments(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Arguments", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_FactoryType(
        &mut self,
        value: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FactoryType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_ProviderFunc(
        &mut self,
        value: *mut crate::System::Func_2<
            *mut crate::Zenject::DiContainer,
            *mut crate::Zenject::IProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ProviderFunc", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+FactoryBindInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::FactoryBindInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
