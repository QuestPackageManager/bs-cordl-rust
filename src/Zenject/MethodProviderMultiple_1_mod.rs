#[cfg(feature = "Zenject+MethodProviderMultiple_1")]
#[repr(C)]
#[derive(Debug)]
pub struct MethodProviderMultiple_1<TReturn: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Object,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _method: *mut crate::System::Func_2<
        *mut crate::Zenject::InjectContext,
        *mut crate::System::Collections::Generic::IEnumerable_1<TReturn>,
    >,
    __cordl_phantom_TReturn: std::marker::PhantomData<TReturn>,
}
#[cfg(feature = "Zenject+MethodProviderMultiple_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::MethodProviderMultiple_1 < TReturn > =>
    "Zenject"."MethodProviderMultiple`1" < TReturn >
);
#[cfg(feature = "Zenject+MethodProviderMultiple_1")]
impl<TReturn: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::MethodProviderMultiple_1<TReturn> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+MethodProviderMultiple_1")]
impl<TReturn: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::MethodProviderMultiple_1<TReturn> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+MethodProviderMultiple_1")]
impl<
    TReturn: quest_hook::libil2cpp::Type,
> crate::Zenject::MethodProviderMultiple_1<TReturn> {
    pub fn GetAllInstancesWithInjectSplit(
        &mut self,
        context: *mut crate::Zenject::InjectContext,
        args: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        injectAction: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Action>,
        buffer: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TReturn: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        Ok(__cordl_ret)
    }
    pub fn GetInstanceType(
        &mut self,
        context: *mut crate::Zenject::InjectContext,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type>
    where
        TReturn: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetInstanceType", (context))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        method: *mut crate::System::Func_2<
            *mut crate::Zenject::InjectContext,
            *mut crate::System::Collections::Generic::IEnumerable_1<TReturn>,
        >,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        TReturn: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (method, container))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        method: *mut crate::System::Func_2<
            *mut crate::Zenject::InjectContext,
            *mut crate::System::Collections::Generic::IEnumerable_1<TReturn>,
        >,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TReturn: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (method, container))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsCached(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TReturn: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCached", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TypeVariesBasedOnMemberType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TReturn: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_TypeVariesBasedOnMemberType", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+MethodProviderMultiple_1")]
impl<TReturn: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::MethodProviderMultiple_1<TReturn> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
