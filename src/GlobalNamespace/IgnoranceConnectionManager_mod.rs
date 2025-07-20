#[cfg(feature = "IgnoranceConnectionManager")]
#[repr(C)]
#[derive(Debug)]
pub struct IgnoranceConnectionManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub onInitializedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub onConnectedEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub onDisconnectedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<crate::GlobalNamespace::DisconnectedReason>,
    >,
    pub onConnectionFailedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<crate::GlobalNamespace::ConnectionFailedReason>,
    >,
    pub onConnectionConnectedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
        >,
    >,
    pub onConnectionDisconnectedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
            crate::GlobalNamespace::DisconnectedReason,
        >,
    >,
    pub onReceivedDataEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_3<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
            quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
            crate::BGNet::Core::DeliveryMethod,
        >,
    >,
    pub _userId_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _userName_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _channels: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::IgnoranceCore::IgnoranceChannelTypes>,
    >,
    pub _state: crate::GlobalNamespace::IgnoranceConnectionManager_ConnectionState,
    pub _client: quest_hook::libil2cpp::Gc<crate::IgnoranceCore::IgnoranceClient>,
    pub _server: quest_hook::libil2cpp::Gc<crate::IgnoranceCore::IgnoranceServer>,
    pub _connectionRequestHandler: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IConnectionRequestHandler,
    >,
    pub _pendingConnections: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<u32>,
    >,
    pub _connections: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection,
            >,
        >,
    >,
    pub _incomingDataWriter: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::Utils::NetDataWriter,
    >,
    pub _incomingDataReader: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::Utils::NetDataReader,
    >,
    pub _connectionRequestWriter: quest_hook::libil2cpp::Gc<
        crate::LiteNetLib::Utils::NetDataWriter,
    >,
    pub _backgroundSentryDisconnectCts: quest_hook::libil2cpp::Gc<
        crate::System::Threading::CancellationTokenSource,
    >,
    pub _sentryDisconnected: bool,
    pub _lastPollUpdateTime: i64,
    pub _timeProvider: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider>,
    pub _taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
    pub _serverUserId_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _serverUserName_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "IgnoranceConnectionManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IgnoranceConnectionManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IgnoranceConnectionManager";
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
#[cfg(feature = "IgnoranceConnectionManager")]
impl std::ops::Deref for crate::GlobalNamespace::IgnoranceConnectionManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceConnectionManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::IgnoranceConnectionManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceConnectionManager")]
impl crate::GlobalNamespace::IgnoranceConnectionManager {
    pub const kBackgroundDisconnectTimeoutMs: i64 = 120000i64;
    pub const kIgnoranceConnectHeader: &'static str = "IgnCon";
    pub const kMaxClientShutdownTimeMs: i32 = 30i32;
    pub const kMaxServerShutdownTimeMs: i32 = 300i32;
    #[cfg(feature = "IgnoranceConnectionManager+ConnectToServerParams")]
    pub type ConnectToServerParams = crate::GlobalNamespace::IgnoranceConnectionManager_ConnectToServerParams;
    #[cfg(feature = "IgnoranceConnectionManager+ConnectionState")]
    pub type ConnectionState = crate::GlobalNamespace::IgnoranceConnectionManager_ConnectionState;
    #[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnection")]
    pub type IgnoranceConnection = crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection;
    #[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnectionParamsBase")]
    pub type IgnoranceConnectionParamsBase = crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase;
    #[cfg(feature = "IgnoranceConnectionManager+StartServerParams")]
    pub type StartServerParams = crate::GlobalNamespace::IgnoranceConnectionManager_StartServerParams;
    pub fn BackgroundDisconnectSentry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                0usize,
            >("BackgroundDisconnectSentry")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(),
                    "BackgroundDisconnectSentry", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckSentryState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("CheckSentryState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "CheckSentryState", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Disconnect(
        &mut self,
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::DisconnectedReason),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Disconnect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "Disconnect", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (disconnectedReason))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DisconnectPeer(
        &mut self,
        peerId: u32,
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (u32, crate::GlobalNamespace::DisconnectedReason),
                quest_hook::libil2cpp::Void,
                2usize,
            >("DisconnectPeer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "DisconnectPeer", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (peerId, disconnectedReason))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DisposeAsync(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
                0usize,
            >("DisposeAsync")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "DisposeAsync", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetConnection(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32),
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
                1usize,
            >("GetConnection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "GetConnection", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnection,
        > = unsafe { method.invoke_unchecked(self, (index))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetConnectionMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                0usize,
            >("GetConnectionMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "GetConnectionMessage",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::LiteNetLib::Utils::NetDataWriter,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn HandleConnectionEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleConnectionEvents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "HandleConnectionEvents",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleDisconnectionEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleDisconnectionEvents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "HandleDisconnectionEvents",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleIncomingEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("HandleIncomingEvents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "HandleIncomingEvents",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Init<T>(
        &mut self,
        initParams: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectionInitParams_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IConnectionInitParams_1<T>,
                >),
                bool,
                1usize,
            >("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "Init", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (initParams))? };
        Ok(__cordl_ret.into())
    }
    pub fn Log(
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Log")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "Log", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (msg))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LogError(
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("LogError")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "LogError", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (msg))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_ITimeProvider_ITaskUtility1(
        timeProvider: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider>,
        taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (timeProvider, taskUtility))?;
        Ok(__cordl_object.into())
    }
    pub fn PollUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("PollUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "PollUpdate", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Send(
        &mut self,
        peerId: u32,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        deliveryMethod: crate::BGNet::Core::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                    crate::BGNet::Core::DeliveryMethod,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("Send")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "Send", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (peerId, writer, deliveryMethod))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SendToAll_IConnection1(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        deliveryMethod: crate::BGNet::Core::DeliveryMethod,
        excludingConnection: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnection,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                    crate::BGNet::Core::DeliveryMethod,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SendToAll")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "SendToAll", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer, deliveryMethod, excludingConnection))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SendToAll_NetDataWriter_DeliveryMethod0(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        deliveryMethod: crate::BGNet::Core::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                    crate::BGNet::Core::DeliveryMethod,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SendToAll")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "SendToAll", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer, deliveryMethod))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn StartBackgroundSentry(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("StartBackgroundSentry")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "StartBackgroundSentry",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToChannel(
        deliveryMethod: crate::BGNet::Core::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::BGNet::Core::DeliveryMethod),
                u8,
                1usize,
            >("ToChannel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "ToChannel", 1usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (deliveryMethod))? };
        Ok(__cordl_ret.into())
    }
    pub fn ToDeliveryMethod(
        channel: u8,
    ) -> quest_hook::libil2cpp::Result<crate::BGNet::Core::DeliveryMethod> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u8),
                crate::BGNet::Core::DeliveryMethod,
                1usize,
            >("ToDeliveryMethod")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "ToDeliveryMethod", 1usize
                )
            });
        let __cordl_ret: crate::BGNet::Core::DeliveryMethod = unsafe {
            method.invoke_unchecked((), (channel))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetConnection(
        &mut self,
        peerId: u32,
        connection: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    u32,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection,
                        >,
                    >,
                ),
                bool,
                2usize,
            >("TryGetConnection")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "TryGetConnection", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (peerId, connection))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParseConnectionMessage(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
        userId: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        userName: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        isConnectionOwner: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                    quest_hook::libil2cpp::ByRefMut<bool>,
                ),
                bool,
                4usize,
            >("TryParseConnectionMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "TryParseConnectionMessage",
                    4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (reader, userId, userName, isConnectionOwner))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WaitForClientShutdown(
        client: quest_hook::libil2cpp::Gc<crate::IgnoranceCore::IgnoranceClient>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::IgnoranceCore::IgnoranceClient>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("WaitForClientShutdown")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "WaitForClientShutdown",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (client))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WaitForServerShutdown(
        server: quest_hook::libil2cpp::Gc<crate::IgnoranceCore::IgnoranceServer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::IgnoranceCore::IgnoranceServer>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("WaitForServerShutdown")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "WaitForServerShutdown",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (server))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ITimeProvider_ITaskUtility1(
        &mut self,
        timeProvider: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider>,
        taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider>,
                    quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (timeProvider, taskUtility))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_onConnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_onConnectedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "add_onConnectedEvent",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_onConnectionConnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_onConnectionConnectedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(),
                    "add_onConnectionConnectedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_onConnectionDisconnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
                crate::GlobalNamespace::DisconnectedReason,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
                        crate::GlobalNamespace::DisconnectedReason,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_onConnectionDisconnectedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(),
                    "add_onConnectionDisconnectedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_onConnectionFailedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::ConnectionFailedReason>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        crate::GlobalNamespace::ConnectionFailedReason,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_onConnectionFailedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(),
                    "add_onConnectionFailedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_onDisconnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::DisconnectedReason>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<crate::GlobalNamespace::DisconnectedReason>,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_onDisconnectedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "add_onDisconnectedEvent",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_onInitializedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_onInitializedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "add_onInitializedEvent",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_onReceivedDataEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
                quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                crate::BGNet::Core::DeliveryMethod,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_3<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
                        quest_hook::libil2cpp::Gc<
                            crate::LiteNetLib::Utils::NetDataReader,
                        >,
                        crate::BGNet::Core::DeliveryMethod,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_onReceivedDataEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "add_onReceivedDataEvent",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_connectionCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_connectionCount")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "get_connectionCount",
                    0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isConnected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isConnected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "get_isConnected", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isConnecting(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isConnecting")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "get_isConnecting", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isConnectionOwner(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isConnectionOwner")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "get_isConnectionOwner",
                    0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isDisconnecting(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isDisconnecting")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "get_isDisconnecting",
                    0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isDisposed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isDisposed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "get_isDisposed", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_serverUserId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_serverUserId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "get_serverUserId", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_serverUserName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_serverUserName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "get_serverUserName", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_userId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_userId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "get_userId", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_userName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_userName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "get_userName", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn remove_onConnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_onConnectedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "remove_onConnectedEvent",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_onConnectionConnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_onConnectionConnectedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(),
                    "remove_onConnectionConnectedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_onConnectionDisconnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
                crate::GlobalNamespace::DisconnectedReason,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_2<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
                        crate::GlobalNamespace::DisconnectedReason,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_onConnectionDisconnectedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(),
                    "remove_onConnectionDisconnectedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_onConnectionFailedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::ConnectionFailedReason>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        crate::GlobalNamespace::ConnectionFailedReason,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_onConnectionFailedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(),
                    "remove_onConnectionFailedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_onDisconnectedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::GlobalNamespace::DisconnectedReason>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<crate::GlobalNamespace::DisconnectedReason>,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_onDisconnectedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(),
                    "remove_onDisconnectedEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_onInitializedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_onInitializedEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "remove_onInitializedEvent",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_onReceivedDataEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_3<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
                quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
                crate::BGNet::Core::DeliveryMethod,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_3<
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
                        quest_hook::libil2cpp::Gc<
                            crate::LiteNetLib::Utils::NetDataReader,
                        >,
                        crate::BGNet::Core::DeliveryMethod,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_onReceivedDataEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(),
                    "remove_onReceivedDataEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_serverUserId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_serverUserId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "set_serverUserId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_serverUserName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_serverUserName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "set_serverUserName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_userId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_userId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "set_userId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_userName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_userName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager as
                    quest_hook::libil2cpp::Type > ::class(), "set_userName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IgnoranceConnectionManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IgnoranceConnectionManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IgnoranceConnectionManager")]
impl AsRef<crate::GlobalNamespace::IConnectionManager>
for crate::GlobalNamespace::IgnoranceConnectionManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::IConnectionManager {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IgnoranceConnectionManager")]
impl AsMut<crate::GlobalNamespace::IConnectionManager>
for crate::GlobalNamespace::IgnoranceConnectionManager {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IConnectionManager {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IgnoranceConnectionManager")]
impl AsRef<crate::GlobalNamespace::IPollable>
for crate::GlobalNamespace::IgnoranceConnectionManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPollable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IgnoranceConnectionManager")]
impl AsMut<crate::GlobalNamespace::IPollable>
for crate::GlobalNamespace::IgnoranceConnectionManager {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPollable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IgnoranceConnectionManager")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::IgnoranceConnectionManager {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IgnoranceConnectionManager")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::IgnoranceConnectionManager {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+ConnectToServerParams")]
#[repr(C)]
#[derive(Debug)]
pub struct IgnoranceConnectionManager_ConnectToServerParams {
    __cordl_parent: crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase,
    pub hostName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub port: i32,
    pub serverUserId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub serverUserName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub validateCertificate: bool,
    pub rootCertificatePath: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub rootCertificate: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "IgnoranceConnectionManager+ConnectToServerParams")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IgnoranceConnectionManager_ConnectToServerParams {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IgnoranceConnectionManager/ConnectToServerParams";
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
#[cfg(feature = "IgnoranceConnectionManager+ConnectToServerParams")]
impl std::ops::Deref
for crate::GlobalNamespace::IgnoranceConnectionManager_ConnectToServerParams {
    type Target = crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+ConnectToServerParams")]
impl std::ops::DerefMut
for crate::GlobalNamespace::IgnoranceConnectionManager_ConnectToServerParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+ConnectToServerParams")]
impl crate::GlobalNamespace::IgnoranceConnectionManager_ConnectToServerParams {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager_ConnectToServerParams as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::IgnoranceConnectionManager_ConnectToServerParams
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IgnoranceConnectionManager+ConnectToServerParams")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IgnoranceConnectionManager_ConnectToServerParams {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IgnoranceConnectionManager+ConnectionState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IgnoranceConnectionManager_ConnectionState {
    #[default]
    Connected = 1i32,
    Connecting = 0i32,
    Disconnected = 3i32,
    Disconnecting = 2i32,
    Disposed = 4i32,
}
#[cfg(feature = "IgnoranceConnectionManager+ConnectionState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IgnoranceConnectionManager_ConnectionState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IgnoranceConnectionManager/ConnectionState";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "IgnoranceConnectionManager+ConnectionState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::IgnoranceConnectionManager_ConnectionState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "IgnoranceConnectionManager+ConnectionState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::IgnoranceConnectionManager_ConnectionState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "IgnoranceConnectionManager+ConnectionState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::IgnoranceConnectionManager_ConnectionState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+ConnectionState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::IgnoranceConnectionManager_ConnectionState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnection")]
#[repr(C)]
#[derive(Debug)]
pub struct IgnoranceConnectionManager_IgnoranceConnection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _connectionManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IgnoranceConnectionManager,
    >,
    pub peerId: u32,
    pub _userId_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _userName_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _isConnectionOwner_k__BackingField: bool,
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IgnoranceConnectionManager/IgnoranceConnection";
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
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnection")]
impl std::ops::Deref
for crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnection")]
impl std::ops::DerefMut
for crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnection")]
impl crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection {
    pub fn Disconnect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Disconnect")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection as
                    quest_hook::libil2cpp::Type > ::class(), "Disconnect", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        connectionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IgnoranceConnectionManager,
        >,
        peerId: u32,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isConnectionOwner: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (connectionManager, peerId, userId, userName, isConnectionOwner),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn Send(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
        deliveryMethod: crate::BGNet::Core::DeliveryMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
                    crate::BGNet::Core::DeliveryMethod,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Send")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection as
                    quest_hook::libil2cpp::Type > ::class(), "Send", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer, deliveryMethod))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        connectionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IgnoranceConnectionManager,
        >,
        peerId: u32,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isConnectionOwner: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::IgnoranceConnectionManager,
                    >,
                    u32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (connectionManager, peerId, userId, userName, isConnectionOwner),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_isConnectionOwner(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isConnectionOwner")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection as
                    quest_hook::libil2cpp::Type > ::class(), "get_isConnectionOwner",
                    0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_userId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_userId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection as
                    quest_hook::libil2cpp::Type > ::class(), "get_userId", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_userName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_userName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection as
                    quest_hook::libil2cpp::Type > ::class(), "get_userName", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_isConnectionOwner(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_isConnectionOwner")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection as
                    quest_hook::libil2cpp::Type > ::class(), "set_isConnectionOwner",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_userId(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_userId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection as
                    quest_hook::libil2cpp::Type > ::class(), "set_userId", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_userName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_userName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection as
                    quest_hook::libil2cpp::Type > ::class(), "set_userName", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnection")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnection")]
impl AsRef<crate::GlobalNamespace::IConnection>
for crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection {
    fn as_ref(&self) -> &crate::GlobalNamespace::IConnection {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnection")]
impl AsMut<crate::GlobalNamespace::IConnection>
for crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnection {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IConnection {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnectionParamsBase")]
#[repr(C)]
#[derive(Debug)]
pub struct IgnoranceConnectionManager_IgnoranceConnectionParamsBase {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub connectionRequestHandler: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IConnectionRequestHandler,
    >,
    pub useSsl: bool,
    pub userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub enableBackgroundSentry: bool,
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnectionParamsBase")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IgnoranceConnectionManager/IgnoranceConnectionParamsBase";
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
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnectionParamsBase")]
impl std::ops::Deref
for crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnectionParamsBase")]
impl std::ops::DerefMut
for crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnectionParamsBase")]
impl crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnectionParamsBase")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnectionParamsBase")]
impl AsRef<
    crate::GlobalNamespace::IConnectionInitParams_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IgnoranceConnectionManager>,
    >,
> for crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IConnectionInitParams_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IgnoranceConnectionManager>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+IgnoranceConnectionParamsBase")]
impl AsMut<
    crate::GlobalNamespace::IConnectionInitParams_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IgnoranceConnectionManager>,
    >,
> for crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IConnectionInitParams_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IgnoranceConnectionManager>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+StartServerParams")]
#[repr(C)]
#[derive(Debug)]
pub struct IgnoranceConnectionManager_StartServerParams {
    __cordl_parent: crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase,
    pub port: i32,
    pub certificatePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub certificate: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub privateKeyPath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub privateKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "IgnoranceConnectionManager+StartServerParams")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IgnoranceConnectionManager_StartServerParams {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IgnoranceConnectionManager/StartServerParams";
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
#[cfg(feature = "IgnoranceConnectionManager+StartServerParams")]
impl std::ops::Deref
for crate::GlobalNamespace::IgnoranceConnectionManager_StartServerParams {
    type Target = crate::GlobalNamespace::IgnoranceConnectionManager_IgnoranceConnectionParamsBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+StartServerParams")]
impl std::ops::DerefMut
for crate::GlobalNamespace::IgnoranceConnectionManager_StartServerParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IgnoranceConnectionManager+StartServerParams")]
impl crate::GlobalNamespace::IgnoranceConnectionManager_StartServerParams {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::IgnoranceConnectionManager_StartServerParams as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::IgnoranceConnectionManager_StartServerParams
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IgnoranceConnectionManager+StartServerParams")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IgnoranceConnectionManager_StartServerParams {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
