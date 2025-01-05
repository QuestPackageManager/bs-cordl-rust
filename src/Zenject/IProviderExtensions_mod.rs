#[cfg(feature = "Zenject+IProviderExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct IProviderExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Zenject+IProviderExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::IProviderExtensions => "Zenject"
    ."IProviderExtensions"
);
#[cfg(feature = "Zenject+IProviderExtensions")]
impl std::ops::Deref for crate::Zenject::IProviderExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IProviderExtensions")]
impl std::ops::DerefMut for crate::Zenject::IProviderExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IProviderExtensions")]
impl crate::Zenject::IProviderExtensions {
    pub fn GetAllInstancesWithInjectSplit(
        creator: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
        injectAction: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Action>,
        >,
        buffer: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetAllInstancesWithInjectSplit",
                (creator, context, injectAction, buffer),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllInstances_Gc1(
        creator: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
        args: quest_hook::libil2cpp::Gc<crate::Zenject::TypeValuePair>,
        buffer: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAllInstances", (creator, context, args, buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllInstances_Gc_Gc_Gc0(
        creator: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
        buffer: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAllInstances", (creator, context, buffer))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstance_Gc1(
        creator: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
        args: quest_hook::libil2cpp::Gc<crate::Zenject::TypeValuePair>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInstance", (creator, context, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstance_Gc_Gc0(
        creator: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInstance", (creator, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInstance_Gc1(
        creator: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
        args: quest_hook::libil2cpp::Gc<crate::Zenject::TypeValuePair>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetInstance", (creator, context, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInstance_Gc_Gc0(
        creator: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetInstance", (creator, context))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+IProviderExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::IProviderExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
