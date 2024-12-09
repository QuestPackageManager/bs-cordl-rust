#[cfg(feature = "Zenject+MonoInstaller_2")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoInstaller_2<
    TParam1: quest_hook::libil2cpp::Type,
    TDerived: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Zenject::MonoInstallerBase,
    __cordl_phantom_TParam1: std::marker::PhantomData<TParam1>,
    __cordl_phantom_TDerived: std::marker::PhantomData<TDerived>,
}
#[cfg(feature = "Zenject+MonoInstaller_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::MonoInstaller_2 < TParam1, TDerived >
    => "Zenject"."MonoInstaller`2" < TParam1, TDerived >
);
#[cfg(feature = "Zenject+MonoInstaller_2")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TDerived: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::Zenject::MonoInstaller_2<TParam1, TDerived> {
    type Target = crate::Zenject::MonoInstallerBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+MonoInstaller_2")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TDerived: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::Zenject::MonoInstaller_2<TParam1, TDerived> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+MonoInstaller_2")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TDerived: quest_hook::libil2cpp::Type,
> crate::Zenject::MonoInstaller_2<TParam1, TDerived> {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDerived: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TDerived: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+MonoInstaller_2")]
impl<
    TParam1: quest_hook::libil2cpp::Type,
    TDerived: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Zenject::MonoInstaller_2<TParam1, TDerived> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
