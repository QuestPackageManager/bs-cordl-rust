#[cfg(feature = "Zenject+PlaceholderFactoryBase_1")]
#[repr(C)]
#[derive(Debug)]
pub struct PlaceholderFactoryBase_1<TValue: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _provider: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
    pub _injectContext: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "Zenject+PlaceholderFactoryBase_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::PlaceholderFactoryBase_1 < TValue > =>
    "Zenject"."PlaceholderFactoryBase`1" < TValue >
);
#[cfg(feature = "Zenject+PlaceholderFactoryBase_1")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::PlaceholderFactoryBase_1<TValue> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PlaceholderFactoryBase_1")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::PlaceholderFactoryBase_1<TValue> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+PlaceholderFactoryBase_1")]
impl<
    TValue: quest_hook::libil2cpp::Type,
> crate::Zenject::PlaceholderFactoryBase_1<TValue> {
    pub fn Construct(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::Zenject::IProvider>,
        injectContext: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Construct", (provider, injectContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInternal(
        &mut self,
        extraArgs: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object.invoke("CreateInternal", (extraArgs))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Validate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Validate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn __zenCreateInjectTypeInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__zenCreateInjectTypeInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn __zenInjectMethod0(
        P_0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        P_1: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__zenInjectMethod0", (P_0, P_1))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParamTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        >,
    >
    where
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Type>,
            >,
        > = __cordl_object.invoke("get_ParamTypes", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+PlaceholderFactoryBase_1")]
impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::PlaceholderFactoryBase_1<TValue> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+PlaceholderFactoryBase_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsRef<crate::Zenject::IPlaceholderFactory>
for crate::Zenject::PlaceholderFactoryBase_1<TValue> {
    fn as_ref(&self) -> &crate::Zenject::IPlaceholderFactory {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+PlaceholderFactoryBase_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsMut<crate::Zenject::IPlaceholderFactory>
for crate::Zenject::PlaceholderFactoryBase_1<TValue> {
    fn as_mut(&mut self) -> &mut crate::Zenject::IPlaceholderFactory {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+PlaceholderFactoryBase_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsRef<crate::Zenject::IValidatable>
for crate::Zenject::PlaceholderFactoryBase_1<TValue> {
    fn as_ref(&self) -> &crate::Zenject::IValidatable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+PlaceholderFactoryBase_1")]
impl<TValue: quest_hook::libil2cpp::Type> AsMut<crate::Zenject::IValidatable>
for crate::Zenject::PlaceholderFactoryBase_1<TValue> {
    fn as_mut(&mut self) -> &mut crate::Zenject::IValidatable {
        unsafe { std::mem::transmute(self) }
    }
}
