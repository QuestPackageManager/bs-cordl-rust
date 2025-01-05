#[cfg(feature = "Zenject+MonoInstaller_1")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoInstaller_1<TDerived: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::Zenject::MonoInstaller,
    __cordl_phantom_TDerived: std::marker::PhantomData<TDerived>,
}
#[cfg(feature = "Zenject+MonoInstaller_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::MonoInstaller_1 < TDerived > =>
    "Zenject"."MonoInstaller`1" < TDerived >
);
#[cfg(feature = "Zenject+MonoInstaller_1")]
impl<TDerived: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::MonoInstaller_1<TDerived> {
    type Target = crate::Zenject::MonoInstaller;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+MonoInstaller_1")]
impl<TDerived: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::MonoInstaller_1<TDerived> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+MonoInstaller_1")]
impl<TDerived: quest_hook::libil2cpp::Type> crate::Zenject::MonoInstaller_1<TDerived> {
    pub fn InstallFromResource_DiContainer0(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<TDerived>
    where
        TDerived: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TDerived = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstallFromResource", (container))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstallFromResource_DiContainer_Il2CppArray2(
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        extraArgs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<TDerived>
    where
        TDerived: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TDerived = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstallFromResource", (container, extraArgs))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstallFromResource_Il2CppString_DiContainer1(
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<TDerived>
    where
        TDerived: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TDerived = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstallFromResource", (resourcePath, container))?;
        Ok(__cordl_ret.into())
    }
    pub fn InstallFromResource_Il2CppString_DiContainer_Il2CppArray3(
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        extraArgs: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<TDerived>
    where
        TDerived: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: TDerived = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InstallFromResource", (resourcePath, container, extraArgs))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TDerived: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn __zenCreateInjectTypeInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    >
    where
        TDerived: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__zenCreateInjectTypeInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TDerived: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+MonoInstaller_1")]
impl<TDerived: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::MonoInstaller_1<TDerived> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
