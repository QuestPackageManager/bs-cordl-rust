#[cfg(feature = "Zenject+BindSignalFromBinder_2")]
#[repr(C)]
#[derive(Debug)]
pub struct BindSignalFromBinder_2<
    TObject: quest_hook::libil2cpp::Type,
    TSignal: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Object,
    pub _bindStatement: *mut crate::Zenject::BindStatement,
    pub _methodGetter: *mut crate::System::Func_2<
        TObject,
        *mut crate::System::Action_1<TSignal>,
    >,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _signalBindInfo: *mut crate::Zenject::SignalBindingBindInfo,
    __cordl_phantom_TObject: std::marker::PhantomData<TObject>,
    __cordl_phantom_TSignal: std::marker::PhantomData<TSignal>,
}
#[cfg(feature = "Zenject+BindSignalFromBinder_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::BindSignalFromBinder_2 < TObject,
    TSignal > => "Zenject"."BindSignalFromBinder`2" < TObject, TSignal >
);
#[cfg(feature = "Zenject+BindSignalFromBinder_2")]
impl<
    TObject: quest_hook::libil2cpp::Type,
    TSignal: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::Zenject::BindSignalFromBinder_2<TObject, TSignal> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+BindSignalFromBinder_2")]
impl<
    TObject: quest_hook::libil2cpp::Type,
    TSignal: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::Zenject::BindSignalFromBinder_2<TObject, TSignal> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+BindSignalFromBinder_2")]
impl<
    TObject: quest_hook::libil2cpp::Type,
    TSignal: quest_hook::libil2cpp::Type,
> crate::Zenject::BindSignalFromBinder_2<TObject, TSignal> {
    #[cfg(feature = "Zenject+BindSignalFromBinder_2+__c")]
    pub type __c = crate::Zenject::BindSignalFromBinder_2___c<TObject, TSignal>;
    #[cfg(feature = "Zenject+BindSignalFromBinder_2+__c__DisplayClass8_0")]
    pub type __c__DisplayClass8_0 = crate::Zenject::BindSignalFromBinder_2___c__DisplayClass8_0<
        TObject,
        TSignal,
    >;
    pub fn From(
        &mut self,
        objectBindCallback: *mut crate::System::Action_1<
            *mut crate::Zenject::ConcreteBinderGeneric_1<TObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::SignalCopyBinder>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::SignalCopyBinder = __cordl_object
            .invoke("From", (objectBindCallback))?;
        Ok(__cordl_ret)
    }
    pub fn FromNew(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::SignalCopyBinder>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::SignalCopyBinder = __cordl_object
            .invoke("FromNew", ())?;
        Ok(__cordl_ret)
    }
    pub fn FromResolve(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::SignalCopyBinder>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::SignalCopyBinder = __cordl_object
            .invoke("FromResolve", ())?;
        Ok(__cordl_ret)
    }
    pub fn FromResolveAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::SignalCopyBinder>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::SignalCopyBinder = __cordl_object
            .invoke("FromResolveAll", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        signalBindInfo: *mut crate::Zenject::SignalBindingBindInfo,
        bindStatement: *mut crate::Zenject::BindStatement,
        methodGetter: *mut crate::System::Func_2<
            TObject,
            *mut crate::System::Action_1<TSignal>,
        >,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (signalBindInfo, bindStatement, methodGetter, container),
            )?;
        Ok(__cordl_object)
    }
    pub fn _From_b__8_0(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Action_1<*mut crate::System::Object>,
    >
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Action_1<*mut crate::System::Object> = __cordl_object
            .invoke("<From>b__8_0", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        signalBindInfo: *mut crate::Zenject::SignalBindingBindInfo,
        bindStatement: *mut crate::Zenject::BindStatement,
        methodGetter: *mut crate::System::Func_2<
            TObject,
            *mut crate::System::Action_1<TSignal>,
        >,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signalBindInfo, bindStatement, methodGetter, container))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+BindSignalFromBinder_2")]
impl<
    TObject: quest_hook::libil2cpp::Type,
    TSignal: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Zenject::BindSignalFromBinder_2<TObject, TSignal> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
