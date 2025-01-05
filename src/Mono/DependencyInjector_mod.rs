#[cfg(feature = "Mono+DependencyInjector")]
#[repr(C)]
#[derive(Debug)]
pub struct DependencyInjector {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+DependencyInjector")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::DependencyInjector => "Mono"
    ."DependencyInjector"
);
#[cfg(feature = "Mono+DependencyInjector")]
impl std::ops::Deref for crate::Mono::DependencyInjector {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+DependencyInjector")]
impl std::ops::DerefMut for crate::Mono::DependencyInjector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+DependencyInjector")]
impl crate::Mono::DependencyInjector {
    pub fn ReflectionLoad() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::ISystemDependencyProvider>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::ISystemDependencyProvider,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ReflectionLoad", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Register(
        provider: quest_hook::libil2cpp::Gc<crate::Mono::ISystemDependencyProvider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Register", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SystemProvider() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Mono::ISystemDependencyProvider>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Mono::ISystemDependencyProvider,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_SystemProvider", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+DependencyInjector")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::DependencyInjector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
