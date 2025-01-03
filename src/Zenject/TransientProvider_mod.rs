#[cfg(feature = "Zenject+TransientProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct TransientProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _concreteType: *mut crate::System::Type,
    pub _extraArguments: *mut crate::System::Collections::Generic::List_1<
        crate::Zenject::TypeValuePair,
    >,
    pub _concreteIdentifier: *mut quest_hook::libil2cpp::Il2CppObject,
    pub _instantiateCallback: *mut crate::System::Action_2<
        *mut crate::Zenject::InjectContext,
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
}
#[cfg(feature = "Zenject+TransientProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::TransientProvider => "Zenject"
    ."TransientProvider"
);
#[cfg(feature = "Zenject+TransientProvider")]
impl std::ops::Deref for crate::Zenject::TransientProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+TransientProvider")]
impl std::ops::DerefMut for crate::Zenject::TransientProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+TransientProvider")]
impl crate::Zenject::TransientProvider {
    pub fn GetAllInstancesWithInjectSplit(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
        injectAction: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Action>,
        buffer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
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
        Ok(__cordl_ret.into())
    }
    pub fn GetInstanceType(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("GetInstanceType", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeToCreate(
        &mut self,
        contractType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("GetTypeToCreate", (contractType))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        concreteType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        extraArguments: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::Zenject::TypeValuePair,
            >,
        >,
        bindingContext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        concreteIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        instantiateCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::Zenject::InjectContext,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    concreteType,
                    container,
                    extraArguments,
                    bindingContext,
                    concreteIdentifier,
                    instantiateCallback,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        concreteType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        extraArguments: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::Zenject::TypeValuePair,
            >,
        >,
        bindingContext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        concreteIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        instantiateCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::Zenject::InjectContext,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    concreteType,
                    container,
                    extraArguments,
                    bindingContext,
                    concreteIdentifier,
                    instantiateCallback,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCached(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCached", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TypeVariesBasedOnMemberType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_TypeVariesBasedOnMemberType", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+TransientProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::TransientProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+TransientProvider")]
impl AsRef<crate::Zenject::IProvider> for crate::Zenject::TransientProvider {
    fn as_ref(&self) -> &crate::Zenject::IProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+TransientProvider")]
impl AsMut<crate::Zenject::IProvider> for crate::Zenject::TransientProvider {
    fn as_mut(&mut self) -> &mut crate::Zenject::IProvider {
        unsafe { std::mem::transmute(self) }
    }
}
