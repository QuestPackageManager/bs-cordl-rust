#[cfg(feature = "MockPlayerInstance")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlayerInstance {
    __cordl_parent: crate::System::Object,
    pub _id: *mut crate::System::String,
    pub _userId: *mut crate::System::String,
    pub _userName: *mut crate::System::String,
    pub _timeProvider: *mut crate::BGNet::Core::ITimeProvider,
    pub _taskUtility: *mut crate::BGNet::Core::ITaskUtility,
    pub _cancellationTokenSource: *mut crate::System::Threading::CancellationTokenSource,
    pub _multiplayerSessionManager: *mut MultiplayerSessionManager,
    pub _connectedPlayerManager: *mut ConnectedPlayerManager,
    pub _fsm: *mut MockPlayerFiniteStateMachine,
}
#[cfg(feature = "MockPlayerInstance")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MockPlayerInstance => ""."MockPlayerInstance"
);
#[cfg(feature = "MockPlayerInstance")]
impl std::ops::Deref for MockPlayerInstance {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerInstance")]
impl std::ops::DerefMut for MockPlayerInstance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlayerInstance")]
impl MockPlayerInstance {
    #[cfg(feature = "MockPlayerInstance+_Stop_d__21")]
    pub type _Stop_d__21 = crate::GlobalNamespace::MockPlayerInstance__Stop_d__21;
    #[cfg(feature = "MockPlayerInstance+_DisposeAsync_d__22")]
    pub type _DisposeAsync_d__22 = crate::GlobalNamespace::MockPlayerInstance__DisposeAsync_d__22;
    #[cfg(feature = "MockPlayerInstance+_RunAsync_d__18")]
    pub type _RunAsync_d__18 = crate::GlobalNamespace::MockPlayerInstance__RunAsync_d__18;
    #[cfg(feature = "MockPlayerInstance+__c__DisplayClass17_0_1")]
    pub type __c__DisplayClass17_0_1<T: quest_hook::libil2cpp::Type> = crate::GlobalNamespace::MockPlayerInstance___c__DisplayClass17_0_1<
        T,
    >;
    pub fn ConnectToServer<T>(
        &mut self,
        connectionInitParams: *mut IConnectionInitParams_1<T>,
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
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        timeProvider: *mut crate::BGNet::Core::ITimeProvider,
        taskUtility: *mut crate::BGNet::Core::ITaskUtility,
        beatmapDataProvider: *mut IMockBeatmapDataProvider,
        connectionManager: *mut IConnectionManager,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (timeProvider, taskUtility, beatmapDataProvider, connectionManager),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_userName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_userName", ())?;
        Ok(__cordl_ret)
    }
    pub fn Stop(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Stop", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn DispatchAsync(
        &mut self,
        action: *mut crate::System::Func_1<*mut crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("DispatchAsync", (action))?;
        Ok(__cordl_ret)
    }
    pub fn RunAsync(
        &mut self,
        runner: *mut IStandaloneThreadRunner,
        token: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("RunAsync", (runner, token))?;
        Ok(__cordl_ret)
    }
    pub fn get_id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_id", ())?;
        Ok(__cordl_ret)
    }
    pub fn __ctor_b__15_1(
        &mut self,
        r: DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<.ctor>b__15_1", (r))?;
        Ok(__cordl_ret)
    }
    pub fn get_userId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_userId", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispatch(
        &mut self,
        action: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispatch", (action))?;
        Ok(__cordl_ret)
    }
    pub fn _DisposeAsync_b__22_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("<DisposeAsync>b__22_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn __ctor_b__15_0(
        &mut self,
        r: ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<.ctor>b__15_0", (r))?;
        Ok(__cordl_ret)
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
        Ok(__cordl_ret)
    }
    pub fn DisposeAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("DisposeAsync", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        timeProvider: *mut crate::BGNet::Core::ITimeProvider,
        taskUtility: *mut crate::BGNet::Core::ITaskUtility,
        beatmapDataProvider: *mut IMockBeatmapDataProvider,
        connectionManager: *mut IConnectionManager,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (timeProvider, taskUtility, beatmapDataProvider, connectionManager),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "MockPlayerInstance")]
impl quest_hook::libil2cpp::ObjectType for MockPlayerInstance {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
