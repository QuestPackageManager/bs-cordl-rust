#[cfg(feature = "RpcHandler_1")]
#[repr(C)]
#[derive(Debug)]
pub struct RpcHandler_1<TType: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Object,
    pub _multiplayerSessionManager: *mut IMultiplayerSessionManager,
    pub _messageType: crate::GlobalNamespace::MultiplayerSessionManager_MessageType,
    pub _rpcSerializer: *mut NetworkPacketSerializer_2<TType, *mut IConnectedPlayer>,
    __cordl_phantom_TType: std::marker::PhantomData<TType>,
}
#[cfg(feature = "RpcHandler_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for RpcHandler_1 < TType > => ""."RpcHandler`1" < TType >
);
#[cfg(feature = "RpcHandler_1")]
impl<TType: quest_hook::libil2cpp::Type> std::ops::Deref for RpcHandler_1<TType> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RpcHandler_1")]
impl<TType: quest_hook::libil2cpp::Type> std::ops::DerefMut for RpcHandler_1<TType> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RpcHandler_1")]
impl<TType: quest_hook::libil2cpp::Type> RpcHandler_1<TType> {
    #[cfg(feature = "RpcHandler_1+__c__DisplayClass10_0_1")]
    pub type __c__DisplayClass10_0_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::RpcHandler_1___c__DisplayClass10_0_1<
        TType,
        T,
    >;
    #[cfg(feature = "RpcHandler_1+__c__DisplayClass17_0_3")]
    pub type __c__DisplayClass17_0_3<
        T: quest_hook::libil2cpp::Type,
        T0: quest_hook::libil2cpp::Type,
        T1: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::RpcHandler_1___c__DisplayClass17_0_3<TType, T, T0, T1>;
    #[cfg(feature = "RpcHandler_1+__c__DisplayClass12_0_3")]
    pub type __c__DisplayClass12_0_3<
        T: quest_hook::libil2cpp::Type,
        T0: quest_hook::libil2cpp::Type,
        T1: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::RpcHandler_1___c__DisplayClass12_0_3<TType, T, T0, T1>;
    #[cfg(feature = "RpcHandler_1+__c__DisplayClass15_0_1")]
    pub type __c__DisplayClass15_0_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::RpcHandler_1___c__DisplayClass15_0_1<
        TType,
        T,
    >;
    #[cfg(feature = "RpcHandler_1+__c__DisplayClass18_0_4")]
    pub type __c__DisplayClass18_0_4<
        T: quest_hook::libil2cpp::Type,
        T0: quest_hook::libil2cpp::Type,
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::RpcHandler_1___c__DisplayClass18_0_4<
        TType,
        T,
        T0,
        T1,
        T2,
    >;
    #[cfg(feature = "RpcHandler_1+__c__DisplayClass11_0_2")]
    pub type __c__DisplayClass11_0_2<
        T: quest_hook::libil2cpp::Type,
        T0: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::RpcHandler_1___c__DisplayClass11_0_2<TType, T, T0>;
    #[cfg(feature = "RpcHandler_1+__c__DisplayClass13_0_4")]
    pub type __c__DisplayClass13_0_4<
        T: quest_hook::libil2cpp::Type,
        T0: quest_hook::libil2cpp::Type,
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::RpcHandler_1___c__DisplayClass13_0_4<
        TType,
        T,
        T0,
        T1,
        T2,
    >;
    #[cfg(feature = "RpcHandler_1+__c__DisplayClass20_0_1")]
    pub type __c__DisplayClass20_0_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::RpcHandler_1___c__DisplayClass20_0_1<
        TType,
        T,
    >;
    #[cfg(feature = "RpcHandler_1+__c__DisplayClass19_0_5")]
    pub type __c__DisplayClass19_0_5<
        T: quest_hook::libil2cpp::Type,
        T0: quest_hook::libil2cpp::Type,
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::RpcHandler_1___c__DisplayClass19_0_5<
        TType,
        T,
        T0,
        T1,
        T2,
        T3,
    >;
    #[cfg(feature = "RpcHandler_1+__c__DisplayClass14_0_5")]
    pub type __c__DisplayClass14_0_5<
        T: quest_hook::libil2cpp::Type,
        T0: quest_hook::libil2cpp::Type,
        T1: quest_hook::libil2cpp::Type,
        T2: quest_hook::libil2cpp::Type,
        T3: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::RpcHandler_1___c__DisplayClass14_0_5<
        TType,
        T,
        T0,
        T1,
        T2,
        T3,
    >;
    #[cfg(feature = "RpcHandler_1+__c__DisplayClass16_0_2")]
    pub type __c__DisplayClass16_0_2<
        T: quest_hook::libil2cpp::Type,
        T0: quest_hook::libil2cpp::Type,
    > = crate::GlobalNamespace::RpcHandler_1___c__DisplayClass16_0_2<TType, T, T0>;
    pub fn _ctor(
        &mut self,
        multiplayerSessionManager: *mut IMultiplayerSessionManager,
        messageType: crate::GlobalNamespace::MultiplayerSessionManager_MessageType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (multiplayerSessionManager, messageType))?;
        Ok(__cordl_ret)
    }
    pub fn Destroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Destroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn EnqueueRpc_0<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnqueueRpc", ())?;
        Ok(__cordl_ret)
    }
    pub fn EnqueueRpc_T0_1<T, T0>(
        &mut self,
        value0: T0,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnqueueRpc", (value0))?;
        Ok(__cordl_ret)
    }
    pub fn EnqueueRpc_T0_T1_2<T, T0, T1>(
        &mut self,
        value0: T0,
        value1: T1,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnqueueRpc", (value0, value1))?;
        Ok(__cordl_ret)
    }
    pub fn EnqueueRpc_T0_T1_T2_3<T, T0, T1, T2>(
        &mut self,
        value0: T0,
        value1: T1,
        value2: T2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnqueueRpc", (value0, value1, value2))?;
        Ok(__cordl_ret)
    }
    pub fn EnqueueRpc_T0_T1_T2_T3_4<T, T0, T1, T2, T3>(
        &mut self,
        value0: T0,
        value1: T1,
        value2: T2,
        value3: T3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EnqueueRpc", (value0, value1, value2, value3))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallbackWithTime_Action_2_0<T>(
        &mut self,
        _cordl_type: TType,
        callback: *mut crate::System::Action_2<*mut crate::System::String, i64>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallbackWithTime", (_cordl_type, callback))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallbackWithTime_Action_3_1<T, T0>(
        &mut self,
        _cordl_type: TType,
        callback: *mut crate::System::Action_3<*mut crate::System::String, i64, T0>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallbackWithTime", (_cordl_type, callback))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallbackWithTime_Action_4_2<T, T0, T1>(
        &mut self,
        _cordl_type: TType,
        callback: *mut crate::System::Action_4<*mut crate::System::String, i64, T0, T1>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallbackWithTime", (_cordl_type, callback))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallbackWithTime_Action_5_3<T, T0, T1, T2>(
        &mut self,
        _cordl_type: TType,
        callback: *mut crate::System::Action_5<
            *mut crate::System::String,
            i64,
            T0,
            T1,
            T2,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallbackWithTime", (_cordl_type, callback))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallbackWithTime_Action_6_4<T, T0, T1, T2, T3>(
        &mut self,
        _cordl_type: TType,
        callback: *mut crate::System::Action_6<
            *mut crate::System::String,
            i64,
            T0,
            T1,
            T2,
            T3,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallbackWithTime", (_cordl_type, callback))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallback_Action_1_0<T>(
        &mut self,
        _cordl_type: TType,
        callback: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallback", (_cordl_type, callback))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallback_Action_2_1<T, T0>(
        &mut self,
        _cordl_type: TType,
        callback: *mut crate::System::Action_2<*mut crate::System::String, T0>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallback", (_cordl_type, callback))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallback_Action_3_2<T, T0, T1>(
        &mut self,
        _cordl_type: TType,
        callback: *mut crate::System::Action_3<*mut crate::System::String, T0, T1>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallback", (_cordl_type, callback))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallback_Action_4_3<T, T0, T1, T2>(
        &mut self,
        _cordl_type: TType,
        callback: *mut crate::System::Action_4<*mut crate::System::String, T0, T1, T2>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallback", (_cordl_type, callback))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallback_Action_5_4<T, T0, T1, T2, T3>(
        &mut self,
        _cordl_type: TType,
        callback: *mut crate::System::Action_5<
            *mut crate::System::String,
            T0,
            T1,
            T2,
            T3,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T0: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallback", (_cordl_type, callback))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterCallback_Action_2_5<T>(
        &mut self,
        _cordl_type: TType,
        callback: *mut crate::System::Action_2<*mut IConnectedPlayer, T>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TType: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterCallback", (_cordl_type, callback))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        multiplayerSessionManager: *mut IMultiplayerSessionManager,
        messageType: crate::GlobalNamespace::MultiplayerSessionManager_MessageType,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (multiplayerSessionManager, messageType))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "RpcHandler_1")]
impl<TType: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for RpcHandler_1<TType> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
