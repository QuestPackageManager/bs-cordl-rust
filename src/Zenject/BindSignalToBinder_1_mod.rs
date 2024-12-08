#[cfg(feature = "Zenject+BindSignalToBinder_1")]
#[repr(C)]
#[derive(Debug)]
pub struct BindSignalToBinder_1<TSignal: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Object,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _bindStatement: *mut crate::Zenject::BindStatement,
    pub _signalBindInfo: *mut crate::Zenject::SignalBindingBindInfo,
    __cordl_phantom_TSignal: std::marker::PhantomData<TSignal>,
}
#[cfg(feature = "Zenject+BindSignalToBinder_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::BindSignalToBinder_1 < TSignal > =>
    "Zenject"."BindSignalToBinder`1" < TSignal >
);
#[cfg(feature = "Zenject+BindSignalToBinder_1")]
impl<TSignal: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Zenject::BindSignalToBinder_1<TSignal> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+BindSignalToBinder_1")]
impl<TSignal: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Zenject::BindSignalToBinder_1<TSignal> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+BindSignalToBinder_1")]
impl<
    TSignal: quest_hook::libil2cpp::Type,
> crate::Zenject::BindSignalToBinder_1<TSignal> {
    #[cfg(feature = "Zenject+BindSignalToBinder_1+__c__DisplayClass7_0")]
    pub type __c__DisplayClass7_0 = crate::Zenject::BindSignalToBinder_1___c__DisplayClass7_0<
        TSignal,
    >;
    #[cfg(feature = "Zenject+BindSignalToBinder_1+__c__DisplayClass9_0_1")]
    pub type __c__DisplayClass9_0_1<TObject: quest_hook::libil2cpp::Type> = crate::Zenject::BindSignalToBinder_1___c__DisplayClass9_0_1<
        TSignal,
        TObject,
    >;
    #[cfg(feature = "Zenject+BindSignalToBinder_1+__c__DisplayClass6_0")]
    pub type __c__DisplayClass6_0 = crate::Zenject::BindSignalToBinder_1___c__DisplayClass6_0<
        TSignal,
    >;
    #[cfg(feature = "Zenject+BindSignalToBinder_1+__c__DisplayClass8_0_1")]
    pub type __c__DisplayClass8_0_1<TObject: quest_hook::libil2cpp::Type> = crate::Zenject::BindSignalToBinder_1___c__DisplayClass8_0_1<
        TSignal,
        TObject,
    >;
    #[cfg(feature = "Zenject+BindSignalToBinder_1+__c__DisplayClass9_1_1")]
    pub type __c__DisplayClass9_1_1<TObject: quest_hook::libil2cpp::Type> = crate::Zenject::BindSignalToBinder_1___c__DisplayClass9_1_1<
        TSignal,
        TObject,
    >;
    #[cfg(feature = "Zenject+BindSignalToBinder_1+__c__DisplayClass8_1_1")]
    pub type __c__DisplayClass8_1_1<TObject: quest_hook::libil2cpp::Type> = crate::Zenject::BindSignalToBinder_1___c__DisplayClass8_1_1<
        TSignal,
        TObject,
    >;
    pub fn _ctor(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
        signalBindInfo: *mut crate::Zenject::SignalBindingBindInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (container, signalBindInfo))?;
        Ok(__cordl_ret)
    }
    pub fn get_SignalBindInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::SignalBindingBindInfo>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::SignalBindingBindInfo = __cordl_object
            .invoke("get_SignalBindInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToMethod_Action_1_0(
        &mut self,
        callback: *mut crate::System::Action_1<TSignal>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::SignalCopyBinder>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::SignalCopyBinder = __cordl_object
            .invoke("ToMethod", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn ToMethod_Action1(
        &mut self,
        callback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::SignalCopyBinder>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::SignalCopyBinder = __cordl_object
            .invoke("ToMethod", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn ToMethod_Action_2_2<TObject>(
        &mut self,
        handler: *mut crate::System::Action_2<TObject, TSignal>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::BindSignalFromBinder_2<TObject, TSignal>,
    >
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::BindSignalFromBinder_2<TObject, TSignal> = __cordl_object
            .invoke("ToMethod", (handler))?;
        Ok(__cordl_ret)
    }
    pub fn ToMethod_Func_2_3<TObject>(
        &mut self,
        handlerGetter: *mut crate::System::Func_2<TObject, *mut crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::BindSignalFromBinder_2<TObject, TSignal>,
    >
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::BindSignalFromBinder_2<TObject, TSignal> = __cordl_object
            .invoke("ToMethod", (handlerGetter))?;
        Ok(__cordl_ret)
    }
    pub fn ToMethod_Func_2_4<TObject>(
        &mut self,
        handlerGetter: *mut crate::System::Func_2<
            TObject,
            *mut crate::System::Action_1<TSignal>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::BindSignalFromBinder_2<TObject, TSignal>,
    >
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TObject: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::BindSignalFromBinder_2<TObject, TSignal> = __cordl_object
            .invoke("ToMethod", (handlerGetter))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        container: *mut crate::Zenject::DiContainer,
        signalBindInfo: *mut crate::Zenject::SignalBindingBindInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (container, signalBindInfo))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+BindSignalToBinder_1")]
impl<TSignal: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Zenject::BindSignalToBinder_1<TSignal> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
