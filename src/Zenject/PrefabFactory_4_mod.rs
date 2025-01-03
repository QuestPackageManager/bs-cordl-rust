#[cfg(feature = "Zenject+PrefabFactory_4")]
#[repr(C)]
#[derive(Debug)]
pub struct PrefabFactory_4<
    P1: quest_hook::libil2cpp::Type,
    P2: quest_hook::libil2cpp::Type,
    P3: quest_hook::libil2cpp::Type,
    T: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _container: *mut crate::Zenject::DiContainer,
    __cordl_phantom_P1: std::marker::PhantomData<P1>,
    __cordl_phantom_P2: std::marker::PhantomData<P2>,
    __cordl_phantom_P3: std::marker::PhantomData<P3>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Zenject+PrefabFactory_4")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::PrefabFactory_4 < P1, P2, P3, T > =>
    "Zenject"."PrefabFactory`4" < P1, P2, P3, T >
);
#[cfg(feature = "Zenject+PrefabFactory_4")]
impl<
    P1: quest_hook::libil2cpp::Type,
    P2: quest_hook::libil2cpp::Type,
    P3: quest_hook::libil2cpp::Type,
    T: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::Zenject::PrefabFactory_4<P1, P2, P3, T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PrefabFactory_4")]
impl<
    P1: quest_hook::libil2cpp::Type,
    P2: quest_hook::libil2cpp::Type,
    P3: quest_hook::libil2cpp::Type,
    T: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::Zenject::PrefabFactory_4<P1, P2, P3, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PrefabFactory_4")]
impl<
    P1: quest_hook::libil2cpp::Type,
    P2: quest_hook::libil2cpp::Type,
    P3: quest_hook::libil2cpp::Type,
    T: quest_hook::libil2cpp::Type,
> crate::Zenject::PrefabFactory_4<P1, P2, P3, T> {
    pub fn Create(
        &mut self,
        prefab: quest_hook::libil2cpp::Gc<crate::UnityEngine::Object>,
        param: P1,
        param2: P2,
        param3: P3,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        P1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: T = __cordl_object
            .invoke("Create", (prefab, param, param2, param3))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        P1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn __zenCreate(
        P_0: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        P1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("__zenCreate", (P_0))?;
        Ok(__cordl_ret.into())
    }
    pub fn __zenCreateInjectTypeInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    >
    where
        P1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__zenCreateInjectTypeInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn __zenFieldSetter0(
        P_0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        P_1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        P1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__zenFieldSetter0", (P_0, P_1))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        P1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Container(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    >
    where
        P1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        P3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer> = __cordl_object
            .invoke("get_Container", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+PrefabFactory_4")]
impl<
    P1: quest_hook::libil2cpp::Type,
    P2: quest_hook::libil2cpp::Type,
    P3: quest_hook::libil2cpp::Type,
    T: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType for crate::Zenject::PrefabFactory_4<P1, P2, P3, T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+PrefabFactory_4")]
impl<
    P1: quest_hook::libil2cpp::Type,
    P2: quest_hook::libil2cpp::Type,
    P3: quest_hook::libil2cpp::Type,
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::Zenject::IFactory> for crate::Zenject::PrefabFactory_4<P1, P2, P3, T> {
    fn as_ref(&self) -> &crate::Zenject::IFactory {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+PrefabFactory_4")]
impl<
    P1: quest_hook::libil2cpp::Type,
    P2: quest_hook::libil2cpp::Type,
    P3: quest_hook::libil2cpp::Type,
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::Zenject::IFactory> for crate::Zenject::PrefabFactory_4<P1, P2, P3, T> {
    fn as_mut(&mut self) -> &mut crate::Zenject::IFactory {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+PrefabFactory_4")]
impl<
    P1: quest_hook::libil2cpp::Type,
    P2: quest_hook::libil2cpp::Type,
    P3: quest_hook::libil2cpp::Type,
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::Zenject::IFactory_5<*mut crate::UnityEngine::Object, P1, P2, P3, T>>
for crate::Zenject::PrefabFactory_4<P1, P2, P3, T> {
    fn as_ref(
        &self,
    ) -> &crate::Zenject::IFactory_5<*mut crate::UnityEngine::Object, P1, P2, P3, T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+PrefabFactory_4")]
impl<
    P1: quest_hook::libil2cpp::Type,
    P2: quest_hook::libil2cpp::Type,
    P3: quest_hook::libil2cpp::Type,
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::Zenject::IFactory_5<*mut crate::UnityEngine::Object, P1, P2, P3, T>>
for crate::Zenject::PrefabFactory_4<P1, P2, P3, T> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Zenject::IFactory_5<
        *mut crate::UnityEngine::Object,
        P1,
        P2,
        P3,
        T,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
