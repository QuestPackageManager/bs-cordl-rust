#[cfg(feature = "Zenject+GetterProvider_2")]
#[repr(C)]
#[derive(Debug)]
pub struct GetterProvider_2<
    TObj: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub _identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _method: quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TResult>>,
    pub _matchAll: bool,
    pub _sourceType: crate::Zenject::InjectSources,
    __cordl_phantom_TObj: std::marker::PhantomData<TObj>,
    __cordl_phantom_TResult: std::marker::PhantomData<TResult>,
}
#[cfg(feature = "Zenject+GetterProvider_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::GetterProvider_2 < TObj, TResult > =>
    "Zenject"."GetterProvider`2" < TObj, TResult >
);
#[cfg(feature = "Zenject+GetterProvider_2")]
impl<
    TObj: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::Zenject::GetterProvider_2<TObj, TResult> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+GetterProvider_2")]
impl<
    TObj: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::Zenject::GetterProvider_2<TObj, TResult> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+GetterProvider_2")]
impl<
    TObj: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> crate::Zenject::GetterProvider_2<TObj, TResult> {
    pub fn GetAllInstancesWithInjectSplit(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
        injectAction: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Action>,
        >,
        buffer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>>
    where
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("GetInstanceType", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSubContext(
        &mut self,
        parent: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    >
    where
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext> = __cordl_object
            .invoke("GetSubContext", (parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TResult>>,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        sourceType: crate::Zenject::InjectSources,
        matchAll: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (identifier, method, container, sourceType, matchAll),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: quest_hook::libil2cpp::Gc<crate::System::Func_2<TObj, TResult>>,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        sourceType: crate::Zenject::InjectSources,
        matchAll: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (identifier, method, container, sourceType, matchAll))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCached(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCached", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TypeVariesBasedOnMemberType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TObj: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TResult: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_TypeVariesBasedOnMemberType", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+GetterProvider_2")]
impl<
    TObj: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType for crate::Zenject::GetterProvider_2<TObj, TResult> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+GetterProvider_2")]
impl<
    TObj: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> AsRef<crate::Zenject::IProvider> for crate::Zenject::GetterProvider_2<TObj, TResult> {
    fn as_ref(&self) -> &crate::Zenject::IProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+GetterProvider_2")]
impl<
    TObj: quest_hook::libil2cpp::Type,
    TResult: quest_hook::libil2cpp::Type,
> AsMut<crate::Zenject::IProvider> for crate::Zenject::GetterProvider_2<TObj, TResult> {
    fn as_mut(&mut self) -> &mut crate::Zenject::IProvider {
        unsafe { std::mem::transmute(self) }
    }
}
