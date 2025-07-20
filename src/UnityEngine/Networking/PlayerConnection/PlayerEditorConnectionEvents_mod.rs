#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerEditorConnectionEvents {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub messageTypeSubscribers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageTypeSubscribers,
            >,
        >,
    >,
    pub connectionEvent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_ConnectionChangeEvent,
    >,
    pub disconnectionEvent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_ConnectionChangeEvent,
    >,
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Networking.PlayerConnection";
    const CLASS_NAME: &'static str = "PlayerEditorConnectionEvents";
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
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents")]
impl std::ops::Deref
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents")]
impl std::ops::DerefMut
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents")]
impl crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents {
    #[cfg(
        feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+ConnectionChangeEvent"
    )]
    pub type ConnectionChangeEvent = crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_ConnectionChangeEvent;
    #[cfg(
        feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageEvent"
    )]
    pub type MessageEvent = crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageEvent;
    #[cfg(
        feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageTypeSubscribers"
    )]
    pub type MessageTypeSubscribers = crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageTypeSubscribers;
    pub fn AddAndCreate(
        &mut self,
        messageId: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::System::Guid),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Events::UnityEvent_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
                                >,
                            >,
                        >,
                        1usize,
                    >("AddAndCreate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "AddAndCreate", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
                >,
            >,
        > = unsafe { method.invoke_unchecked(self, (messageId))? };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeMessageIdSubscribers(
        &mut self,
        messageId: crate::System::Guid,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        playerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::System::Guid,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("InvokeMessageIdSubscribers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "InvokeMessageIdSubscribers", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (messageId, data, playerId))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UnregisterManagedCallback(
        &mut self,
        messageId: crate::System::Guid,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityAction_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            crate::System::Guid,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Events::UnityAction_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("UnregisterManagedCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "UnregisterManagedCallback", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (messageId, callback))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+ConnectionChangeEvent"
)]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerEditorConnectionEvents_ConnectionChangeEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<i32>,
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+ConnectionChangeEvent"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_ConnectionChangeEvent {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Networking.PlayerConnection";
    const CLASS_NAME: &'static str = "PlayerEditorConnectionEvents/ConnectionChangeEvent";
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
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+ConnectionChangeEvent"
)]
impl std::ops::Deref
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_ConnectionChangeEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<i32>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+ConnectionChangeEvent"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_ConnectionChangeEvent {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+ConnectionChangeEvent"
)]
impl crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_ConnectionChangeEvent {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+ConnectionChangeEvent"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_ConnectionChangeEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageEvent"
)]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerEditorConnectionEvents_MessageEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
        >,
    >,
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageEvent"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageEvent {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Networking.PlayerConnection";
    const CLASS_NAME: &'static str = "PlayerEditorConnectionEvents/MessageEvent";
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
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageEvent"
)]
impl std::ops::Deref
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
        >,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageEvent"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageEvent {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageEvent"
)]
impl crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageEvent {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageEvent"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageTypeSubscribers"
)]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerEditorConnectionEvents_MessageTypeSubscribers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_messageTypeId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub subscriberCount: i32,
    pub messageCallback: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageEvent,
    >,
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageTypeSubscribers"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageTypeSubscribers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Networking.PlayerConnection";
    const CLASS_NAME: &'static str = "PlayerEditorConnectionEvents/MessageTypeSubscribers";
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
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageTypeSubscribers"
)]
impl std::ops::Deref
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageTypeSubscribers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageTypeSubscribers"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageTypeSubscribers {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageTypeSubscribers"
)]
impl crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageTypeSubscribers {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_MessageTypeId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<(), crate::System::Guid, 0usize>("get_MessageTypeId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "get_MessageTypeId", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Guid = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_MessageTypeId(
        &mut self,
        value: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (crate::System::Guid),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_MessageTypeId")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "set_MessageTypeId", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageTypeSubscribers"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageTypeSubscribers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
