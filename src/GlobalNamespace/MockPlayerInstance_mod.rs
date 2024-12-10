#[cfg(feature = "MockPlayerInstance")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlayerInstance {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _id: *mut quest_hook::libil2cpp::Il2CppString,
    pub _userId: *mut quest_hook::libil2cpp::Il2CppString,
    pub _userName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _timeProvider: *mut crate::BGNet::Core::ITimeProvider,
    pub _taskUtility: *mut crate::BGNet::Core::ITaskUtility,
    pub _cancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
    pub _multiplayerSessionManager: *mut crate::GlobalNamespace::MultiplayerSessionManager,
    pub _connectedPlayerManager: *mut crate::GlobalNamespace::ConnectedPlayerManager,
    pub _fsm: *mut crate::GlobalNamespace::MockPlayerFiniteStateMachine,
}
#[cfg(feature = "MockPlayerInstance")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MockPlayerInstance => ""
    ."MockPlayerInstance"
);
#[cfg(feature = "MockPlayerInstance")]
impl std::ops::Deref for crate::GlobalNamespace::MockPlayerInstance {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerInstance")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockPlayerInstance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerInstance")]
impl crate::GlobalNamespace::MockPlayerInstance {
    #[cfg(feature = "MockPlayerInstance+_DisposeAsync_d__22")]
    pub type _DisposeAsync_d__22 = crate::GlobalNamespace::MockPlayerInstance__DisposeAsync_d__22;
    #[cfg(feature = "MockPlayerInstance+_RunAsync_d__18")]
    pub type _RunAsync_d__18 = crate::GlobalNamespace::MockPlayerInstance__RunAsync_d__18;
    #[cfg(feature = "MockPlayerInstance+_Stop_d__21")]
    pub type _Stop_d__21 = crate::GlobalNamespace::MockPlayerInstance__Stop_d__21;
    #[cfg(feature = "MockPlayerInstance+__c__DisplayClass17_0_1")]
    pub type __c__DisplayClass17_0_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::MockPlayerInstance___c__DisplayClass17_0_1<
        T,
    >;
    pub fn ConnectToServer<T>(
        &mut self,
        connectionInitParams: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectionInitParams_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConnectToServer", (connectionInitParams))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispatch(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispatch", (action))?;
        Ok(__cordl_ret.into())
    }
    pub fn DispatchAsync(
        &mut self,
        action: quest_hook::libil2cpp::Gc<
            crate::System::Func_1<*mut crate::System::Threading::Tasks::Task>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("DispatchAsync", (action))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DisposeAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("DisposeAsync", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        timeProvider: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider>,
        taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
        beatmapDataProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMockBeatmapDataProvider,
        >,
        connectionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (timeProvider, taskUtility, beatmapDataProvider, connectionManager),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn RunAsync(
        &mut self,
        runner: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IStandaloneThreadRunner,
        >,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("RunAsync", (runner, token))?;
        Ok(__cordl_ret.into())
    }
    pub fn Stop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Stop", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _DisposeAsync_b__22_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = __cordl_object.invoke("<DisposeAsync>b__22_0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn __ctor_b__15_0(
        &mut self,
        r: crate::GlobalNamespace::ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<.ctor>b__15_0", (r))?;
        Ok(__cordl_ret.into())
    }
    pub fn __ctor_b__15_1(
        &mut self,
        r: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<.ctor>b__15_1", (r))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        timeProvider: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider>,
        taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
        beatmapDataProvider: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMockBeatmapDataProvider,
        >,
        connectionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectionManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (timeProvider, taskUtility, beatmapDataProvider, connectionManager),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_id", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_userId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_userId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_userName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_userName", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MockPlayerInstance")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MockPlayerInstance {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
