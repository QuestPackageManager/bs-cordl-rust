#[cfg(feature = "Zenject+SignalBus")]
#[repr(C)]
#[derive(Debug)]
pub struct SignalBus {
    __cordl_parent: crate::System::Object,
    pub _subscriptionPool: *mut crate::Zenject::SignalSubscription_Pool,
    pub _localDeclarationMap: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::Zenject::BindingId,
        *mut crate::Zenject::SignalDeclaration,
    >,
    pub _parentBus: *mut crate::Zenject::SignalBus,
    pub _subscriptionMap: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::Zenject::SignalSubscriptionId,
        *mut crate::Zenject::SignalSubscription,
    >,
    pub _settings: *mut crate::Zenject::ZenjectSettings_SignalSettings,
    pub _signalDeclarationFactory: *mut crate::Zenject::SignalDeclaration_Factory,
    pub _container: *mut crate::Zenject::DiContainer,
}
#[cfg(feature = "Zenject+SignalBus")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SignalBus => "Zenject"."SignalBus"
);
#[cfg(feature = "Zenject+SignalBus")]
impl std::ops::Deref for crate::Zenject::SignalBus {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalBus")]
impl std::ops::DerefMut for crate::Zenject::SignalBus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalBus")]
impl crate::Zenject::SignalBus {
    #[cfg(feature = "Zenject+SignalBus+__c")]
    pub type __c = crate::Zenject::SignalBus___c;
    #[cfg(feature = "Zenject+SignalBus+__c__DisplayClass25_0_1")]
    pub type __c__DisplayClass25_0_1<TSignal: quest_hook::libil2cpp::Type> = crate::Zenject::SignalBus___c__DisplayClass25_0_1<
        TSignal,
    >;
    #[cfg(feature = "Zenject+SignalBus+__c__DisplayClass27_0_1")]
    pub type __c__DisplayClass27_0_1<TSignal: quest_hook::libil2cpp::Type> = crate::Zenject::SignalBus___c__DisplayClass27_0_1<
        TSignal,
    >;
    pub fn TryFireId_TSignal0<TSignal>(
        &mut self,
        identifier: *mut crate::System::Object,
        signal: TSignal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryFireId", (identifier, signal))?;
        Ok(__cordl_ret)
    }
    pub fn TryFireId_Object1<TSignal>(
        &mut self,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryFireId", (identifier))?;
        Ok(__cordl_ret)
    }
    pub fn TryFireId_Object2(
        &mut self,
        identifier: *mut crate::System::Object,
        signal: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryFireId", (identifier, signal))?;
        Ok(__cordl_ret)
    }
    pub fn UnsubscribeInternal_Type_Object_Object__cordl_bool0(
        &mut self,
        signalType: *mut crate::System::Type,
        identifier: *mut crate::System::Object,
        token: *mut crate::System::Object,
        throwIfMissing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "UnsubscribeInternal",
                (signalType, identifier, token, throwIfMissing),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UnsubscribeInternal_BindingId_Object__cordl_bool1(
        &mut self,
        signalId: crate::Zenject::BindingId,
        token: *mut crate::System::Object,
        throwIfMissing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnsubscribeInternal", (signalId, token, throwIfMissing))?;
        Ok(__cordl_ret)
    }
    pub fn UnsubscribeInternal_SignalSubscriptionId__cordl_bool2(
        &mut self,
        id: crate::Zenject::SignalSubscriptionId,
        throwIfMissing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnsubscribeInternal", (id, throwIfMissing))?;
        Ok(__cordl_ret)
    }
    pub fn Subscribe_Action0<TSignal>(
        &mut self,
        callback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Subscribe", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn Subscribe_Action_1_1<TSignal>(
        &mut self,
        callback: *mut crate::System::Action_1<TSignal>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Subscribe", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn Subscribe_Type_Action_1_2(
        &mut self,
        signalType: *mut crate::System::Type,
        callback: *mut crate::System::Action_1<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Subscribe", (signalType, callback))?;
        Ok(__cordl_ret)
    }
    pub fn FireId_TSignal0<TSignal>(
        &mut self,
        identifier: *mut crate::System::Object,
        signal: TSignal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FireId", (identifier, signal))?;
        Ok(__cordl_ret)
    }
    pub fn FireId_Object1<TSignal>(
        &mut self,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FireId", (identifier))?;
        Ok(__cordl_ret)
    }
    pub fn FireId_Object2(
        &mut self,
        identifier: *mut crate::System::Object,
        signal: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FireId", (identifier, signal))?;
        Ok(__cordl_ret)
    }
    pub fn get_ParentBus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::SignalBus> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::SignalBus = __cordl_object
            .invoke("get_ParentBus", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryUnsubscribeId_Object_Action0<TSignal>(
        &mut self,
        identifier: *mut crate::System::Object,
        callback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryUnsubscribeId", (identifier, callback))?;
        Ok(__cordl_ret)
    }
    pub fn TryUnsubscribeId_Type_Object_Action1(
        &mut self,
        signalType: *mut crate::System::Type,
        identifier: *mut crate::System::Object,
        callback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryUnsubscribeId", (signalType, identifier, callback))?;
        Ok(__cordl_ret)
    }
    pub fn TryUnsubscribeId_Type_Object_Action_1_2(
        &mut self,
        signalType: *mut crate::System::Type,
        identifier: *mut crate::System::Object,
        callback: *mut crate::System::Action_1<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryUnsubscribeId", (signalType, identifier, callback))?;
        Ok(__cordl_ret)
    }
    pub fn TryUnsubscribeId_Object_Action_1_3<TSignal>(
        &mut self,
        identifier: *mut crate::System::Object,
        callback: *mut crate::System::Action_1<TSignal>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryUnsubscribeId", (identifier, callback))?;
        Ok(__cordl_ret)
    }
    pub fn Fire_TSignal0<TSignal>(
        &mut self,
        signal: TSignal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Fire", (signal))?;
        Ok(__cordl_ret)
    }
    pub fn Fire_1<TSignal>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Fire", ())?;
        Ok(__cordl_ret)
    }
    pub fn Fire_Object2(
        &mut self,
        signal: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Fire", (signal))?;
        Ok(__cordl_ret)
    }
    pub fn SubscribeInternal_Type_Object_Object_Action_1_0(
        &mut self,
        signalType: *mut crate::System::Type,
        identifier: *mut crate::System::Object,
        token: *mut crate::System::Object,
        callback: *mut crate::System::Action_1<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubscribeInternal", (signalType, identifier, token, callback))?;
        Ok(__cordl_ret)
    }
    pub fn SubscribeInternal_BindingId_Object_Action_1_1(
        &mut self,
        signalId: crate::Zenject::BindingId,
        token: *mut crate::System::Object,
        callback: *mut crate::System::Action_1<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubscribeInternal", (signalId, token, callback))?;
        Ok(__cordl_ret)
    }
    pub fn SubscribeInternal_SignalSubscriptionId_Action_1_2(
        &mut self,
        id: crate::Zenject::SignalSubscriptionId,
        callback: *mut crate::System::Action_1<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubscribeInternal", (id, callback))?;
        Ok(__cordl_ret)
    }
    pub fn LateDispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateDispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn DeclareSignal_Object_Nullable_1_0<T>(
        &mut self,
        identifier: *mut crate::System::Object,
        missingHandlerResponse: crate::System::Nullable_1<
            crate::Zenject::SignalMissingHandlerResponses,
        >,
        forceAsync: crate::System::Nullable_1<bool>,
        asyncTickPriority: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DeclareSignal",
                (identifier, missingHandlerResponse, forceAsync, asyncTickPriority),
            )?;
        Ok(__cordl_ret)
    }
    pub fn DeclareSignal_Type_Object_Nullable_1_1(
        &mut self,
        signalType: *mut crate::System::Type,
        identifier: *mut crate::System::Object,
        missingHandlerResponse: crate::System::Nullable_1<
            crate::Zenject::SignalMissingHandlerResponses,
        >,
        forceAsync: crate::System::Nullable_1<bool>,
        asyncTickPriority: crate::System::Nullable_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "DeclareSignal",
                (
                    signalType,
                    identifier,
                    missingHandlerResponse,
                    forceAsync,
                    asyncTickPriority,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn TryUnsubscribe_Action0<TSignal>(
        &mut self,
        callback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryUnsubscribe", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn TryUnsubscribe_Type_Action1(
        &mut self,
        signalType: *mut crate::System::Type,
        callback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryUnsubscribe", (signalType, callback))?;
        Ok(__cordl_ret)
    }
    pub fn TryUnsubscribe_Type_Action_1_2(
        &mut self,
        signalType: *mut crate::System::Type,
        callback: *mut crate::System::Action_1<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryUnsubscribe", (signalType, callback))?;
        Ok(__cordl_ret)
    }
    pub fn TryUnsubscribe_Action_1_3<TSignal>(
        &mut self,
        callback: *mut crate::System::Action_1<TSignal>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryUnsubscribe", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        signalDeclarations: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::SignalDeclaration,
        >,
        parentBus: *mut crate::Zenject::SignalBus,
        zenjectSettings: *mut crate::Zenject::ZenjectSettings,
        subscriptionPool: *mut crate::Zenject::SignalSubscription_Pool,
        signalDeclarationFactory: *mut crate::Zenject::SignalDeclaration_Factory,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    signalDeclarations,
                    parentBus,
                    zenjectSettings,
                    subscriptionPool,
                    signalDeclarationFactory,
                    container,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn SubscribeId_Object_Action0<TSignal>(
        &mut self,
        identifier: *mut crate::System::Object,
        callback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubscribeId", (identifier, callback))?;
        Ok(__cordl_ret)
    }
    pub fn SubscribeId_Object_Action_1_1<TSignal>(
        &mut self,
        identifier: *mut crate::System::Object,
        callback: *mut crate::System::Action_1<TSignal>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubscribeId", (identifier, callback))?;
        Ok(__cordl_ret)
    }
    pub fn SubscribeId_Type_Object_Action_1_2(
        &mut self,
        signalType: *mut crate::System::Type,
        identifier: *mut crate::System::Object,
        callback: *mut crate::System::Action_1<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubscribeId", (signalType, identifier, callback))?;
        Ok(__cordl_ret)
    }
    pub fn get_NumSubscribers(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_NumSubscribers", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnsubscribeId_Object_Action0<TSignal>(
        &mut self,
        identifier: *mut crate::System::Object,
        callback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnsubscribeId", (identifier, callback))?;
        Ok(__cordl_ret)
    }
    pub fn UnsubscribeId_Type_Object_Action1(
        &mut self,
        signalType: *mut crate::System::Type,
        identifier: *mut crate::System::Object,
        callback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnsubscribeId", (signalType, identifier, callback))?;
        Ok(__cordl_ret)
    }
    pub fn UnsubscribeId_Type_Object_Action_1_2(
        &mut self,
        signalType: *mut crate::System::Type,
        identifier: *mut crate::System::Object,
        callback: *mut crate::System::Action_1<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnsubscribeId", (signalType, identifier, callback))?;
        Ok(__cordl_ret)
    }
    pub fn UnsubscribeId_Object_Action_1_3<TSignal>(
        &mut self,
        identifier: *mut crate::System::Object,
        callback: *mut crate::System::Action_1<TSignal>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnsubscribeId", (identifier, callback))?;
        Ok(__cordl_ret)
    }
    pub fn TryFire_TSignal0<TSignal>(
        &mut self,
        signal: TSignal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryFire", (signal))?;
        Ok(__cordl_ret)
    }
    pub fn TryFire_1<TSignal>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryFire", ())?;
        Ok(__cordl_ret)
    }
    pub fn TryFire_Object2(
        &mut self,
        signal: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryFire", (signal))?;
        Ok(__cordl_ret)
    }
    pub fn Unsubscribe_Action0<TSignal>(
        &mut self,
        callback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unsubscribe", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn Unsubscribe_Type_Action1(
        &mut self,
        signalType: *mut crate::System::Type,
        callback: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unsubscribe", (signalType, callback))?;
        Ok(__cordl_ret)
    }
    pub fn Unsubscribe_Type_Action_1_2(
        &mut self,
        signalType: *mut crate::System::Type,
        callback: *mut crate::System::Action_1<*mut crate::System::Object>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unsubscribe", (signalType, callback))?;
        Ok(__cordl_ret)
    }
    pub fn Unsubscribe_Action_1_3<TSignal>(
        &mut self,
        callback: *mut crate::System::Action_1<TSignal>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSignal: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unsubscribe", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn GetDeclaration_Type_Object__cordl_bool0(
        &mut self,
        signalType: *mut crate::System::Type,
        identifier: *mut crate::System::Object,
        requireDeclaration: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::SignalDeclaration> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::SignalDeclaration = __cordl_object
            .invoke("GetDeclaration", (signalType, identifier, requireDeclaration))?;
        Ok(__cordl_ret)
    }
    pub fn GetDeclaration_BindingId__cordl_bool1(
        &mut self,
        signalId: crate::Zenject::BindingId,
        requireDeclaration: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::SignalDeclaration> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::SignalDeclaration = __cordl_object
            .invoke("GetDeclaration", (signalId, requireDeclaration))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        signalDeclarations: *mut crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::SignalDeclaration,
        >,
        parentBus: *mut crate::Zenject::SignalBus,
        zenjectSettings: *mut crate::Zenject::ZenjectSettings,
        subscriptionPool: *mut crate::Zenject::SignalSubscription_Pool,
        signalDeclarationFactory: *mut crate::Zenject::SignalDeclaration_Factory,
        container: *mut crate::Zenject::DiContainer,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    signalDeclarations,
                    parentBus,
                    zenjectSettings,
                    subscriptionPool,
                    signalDeclarationFactory,
                    container,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+SignalBus")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::SignalBus {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
