#[cfg(feature = "Zenject+SignalBus")]
#[repr(C)]
#[derive(Debug)]
pub struct SignalBus {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _subscriptionPool: quest_hook::libil2cpp::Gc<
        crate::Zenject::SignalSubscription_Pool,
    >,
    pub _localDeclarationMap: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::Zenject::BindingId,
            quest_hook::libil2cpp::Gc<crate::Zenject::SignalDeclaration>,
        >,
    >,
    pub _parentBus: quest_hook::libil2cpp::Gc<crate::Zenject::SignalBus>,
    pub _subscriptionMap: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            crate::Zenject::SignalSubscriptionId,
            quest_hook::libil2cpp::Gc<crate::Zenject::SignalSubscription>,
        >,
    >,
    pub _settings: quest_hook::libil2cpp::Gc<
        crate::Zenject::ZenjectSettings_SignalSettings,
    >,
    pub _signalDeclarationFactory: quest_hook::libil2cpp::Gc<
        crate::Zenject::SignalDeclaration_Factory,
    >,
    pub _container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
}
#[cfg(feature = "Zenject+SignalBus")]
unsafe impl quest_hook::libil2cpp::Type for crate::Zenject::SignalBus {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject";
    const CLASS_NAME: &'static str = "SignalBus";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Zenject+SignalBus")]
impl std::ops::Deref for crate::Zenject::SignalBus {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn DeclareSignal_Il2CppObject_Nullable_1_0<T>(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
        Ok(__cordl_ret.into())
    }
    pub fn DeclareSignal_Type_Il2CppObject_Nullable_1_1(
        &mut self,
        signalType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
        Ok(__cordl_ret.into())
    }
    pub fn FireId_Il2CppObject1<TSignal>(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
        Ok(__cordl_ret.into())
    }
    pub fn FireId_Il2CppObject2(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        signal: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FireId", (identifier, signal))?;
        Ok(__cordl_ret.into())
    }
    pub fn FireId_TSignal0<TSignal>(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Fire_Il2CppObject2(
        &mut self,
        signal: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Fire", (signal))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn GetDeclaration_BindingId__cordl_bool1(
        &mut self,
        signalId: crate::Zenject::BindingId,
        requireDeclaration: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::SignalDeclaration>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::SignalDeclaration> = __cordl_object
            .invoke("GetDeclaration", (signalId, requireDeclaration))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDeclaration_Type_Il2CppObject__cordl_bool0(
        &mut self,
        signalType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        requireDeclaration: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::SignalDeclaration>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::SignalDeclaration> = __cordl_object
            .invoke("GetDeclaration", (signalType, identifier, requireDeclaration))?;
        Ok(__cordl_ret.into())
    }
    pub fn LateDispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateDispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        signalDeclarations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::SignalDeclaration>,
            >,
        >,
        parentBus: quest_hook::libil2cpp::Gc<crate::Zenject::SignalBus>,
        zenjectSettings: quest_hook::libil2cpp::Gc<crate::Zenject::ZenjectSettings>,
        subscriptionPool: quest_hook::libil2cpp::Gc<
            crate::Zenject::SignalSubscription_Pool,
        >,
        signalDeclarationFactory: quest_hook::libil2cpp::Gc<
            crate::Zenject::SignalDeclaration_Factory,
        >,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
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
        Ok(__cordl_object.into())
    }
    pub fn SubscribeId_Il2CppObject_Action0<TSignal>(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action>,
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
        Ok(__cordl_ret.into())
    }
    pub fn SubscribeId_Il2CppObject_Action_1_1<TSignal>(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<TSignal>>,
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
        Ok(__cordl_ret.into())
    }
    pub fn SubscribeId_Type_Il2CppObject_Action_1_2(
        &mut self,
        signalType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubscribeId", (signalType, identifier, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubscribeInternal_BindingId_Il2CppObject_Action_1_1(
        &mut self,
        signalId: crate::Zenject::BindingId,
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubscribeInternal", (signalId, token, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubscribeInternal_SignalSubscriptionId_Action_1_2(
        &mut self,
        id: crate::Zenject::SignalSubscriptionId,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubscribeInternal", (id, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn SubscribeInternal_Type_Il2CppObject_Il2CppObject_Action_1_0(
        &mut self,
        signalType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubscribeInternal", (signalType, identifier, token, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn Subscribe_Action0<TSignal>(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action>,
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
        Ok(__cordl_ret.into())
    }
    pub fn Subscribe_Action_1_1<TSignal>(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<TSignal>>,
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
        Ok(__cordl_ret.into())
    }
    pub fn Subscribe_Type_Action_1_2(
        &mut self,
        signalType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Subscribe", (signalType, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFireId_Il2CppObject1<TSignal>(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
        Ok(__cordl_ret.into())
    }
    pub fn TryFireId_Il2CppObject2(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        signal: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryFireId", (identifier, signal))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFireId_TSignal0<TSignal>(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn TryFire_Il2CppObject2(
        &mut self,
        signal: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryFire", (signal))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn TryUnsubscribeId_Il2CppObject_Action0<TSignal>(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action>,
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
        Ok(__cordl_ret.into())
    }
    pub fn TryUnsubscribeId_Il2CppObject_Action_1_3<TSignal>(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<TSignal>>,
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
        Ok(__cordl_ret.into())
    }
    pub fn TryUnsubscribeId_Type_Il2CppObject_Action1(
        &mut self,
        signalType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryUnsubscribeId", (signalType, identifier, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryUnsubscribeId_Type_Il2CppObject_Action_1_2(
        &mut self,
        signalType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryUnsubscribeId", (signalType, identifier, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryUnsubscribe_Action0<TSignal>(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action>,
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
        Ok(__cordl_ret.into())
    }
    pub fn TryUnsubscribe_Action_1_3<TSignal>(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<TSignal>>,
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
        Ok(__cordl_ret.into())
    }
    pub fn TryUnsubscribe_Type_Action1(
        &mut self,
        signalType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryUnsubscribe", (signalType, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryUnsubscribe_Type_Action_1_2(
        &mut self,
        signalType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryUnsubscribe", (signalType, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsubscribeId_Il2CppObject_Action0<TSignal>(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action>,
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
        Ok(__cordl_ret.into())
    }
    pub fn UnsubscribeId_Il2CppObject_Action_1_3<TSignal>(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<TSignal>>,
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
        Ok(__cordl_ret.into())
    }
    pub fn UnsubscribeId_Type_Il2CppObject_Action1(
        &mut self,
        signalType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnsubscribeId", (signalType, identifier, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsubscribeId_Type_Il2CppObject_Action_1_2(
        &mut self,
        signalType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnsubscribeId", (signalType, identifier, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnsubscribeInternal_BindingId_Il2CppObject__cordl_bool1(
        &mut self,
        signalId: crate::Zenject::BindingId,
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        throwIfMissing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnsubscribeInternal", (signalId, token, throwIfMissing))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn UnsubscribeInternal_Type_Il2CppObject_Il2CppObject__cordl_bool0(
        &mut self,
        signalType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        token: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
        Ok(__cordl_ret.into())
    }
    pub fn Unsubscribe_Action0<TSignal>(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action>,
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
        Ok(__cordl_ret.into())
    }
    pub fn Unsubscribe_Action_1_3<TSignal>(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_1<TSignal>>,
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
        Ok(__cordl_ret.into())
    }
    pub fn Unsubscribe_Type_Action1(
        &mut self,
        signalType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unsubscribe", (signalType, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unsubscribe_Type_Action_1_2(
        &mut self,
        signalType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        callback: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unsubscribe", (signalType, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn __zenCreate(
        P_0: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("__zenCreate", (P_0))?;
        Ok(__cordl_ret.into())
    }
    pub fn __zenCreateInjectTypeInfo() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("__zenCreateInjectTypeInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        signalDeclarations: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<crate::Zenject::SignalDeclaration>,
            >,
        >,
        parentBus: quest_hook::libil2cpp::Gc<crate::Zenject::SignalBus>,
        zenjectSettings: quest_hook::libil2cpp::Gc<crate::Zenject::ZenjectSettings>,
        subscriptionPool: quest_hook::libil2cpp::Gc<
            crate::Zenject::SignalSubscription_Pool,
        >,
        signalDeclarationFactory: quest_hook::libil2cpp::Gc<
            crate::Zenject::SignalDeclaration_Factory,
        >,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
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
        Ok(__cordl_ret.into())
    }
    pub fn get_NumSubscribers(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_NumSubscribers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ParentBus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::SignalBus>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::SignalBus> = __cordl_object
            .invoke("get_ParentBus", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Zenject+SignalBus")]
impl AsRef<crate::Zenject::ILateDisposable> for crate::Zenject::SignalBus {
    fn as_ref(&self) -> &crate::Zenject::ILateDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+SignalBus")]
impl AsMut<crate::Zenject::ILateDisposable> for crate::Zenject::SignalBus {
    fn as_mut(&mut self) -> &mut crate::Zenject::ILateDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
