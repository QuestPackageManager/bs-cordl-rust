#[cfg(feature = "StandardScoreSyncStateNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct StandardScoreSyncStateNetSerializable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _state: crate::GlobalNamespace::StandardScoreSyncState,
    pub _id_k__BackingField: crate::GlobalNamespace::SyncStateId,
    pub _time_k__BackingField: i64,
}
#[cfg(feature = "StandardScoreSyncStateNetSerializable")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::StandardScoreSyncStateNetSerializable {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "StandardScoreSyncStateNetSerializable";
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
#[cfg(feature = "StandardScoreSyncStateNetSerializable")]
impl std::ops::Deref for crate::GlobalNamespace::StandardScoreSyncStateNetSerializable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StandardScoreSyncStateNetSerializable")]
impl std::ops::DerefMut
for crate::GlobalNamespace::StandardScoreSyncStateNetSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StandardScoreSyncStateNetSerializable")]
impl crate::GlobalNamespace::StandardScoreSyncStateNetSerializable {
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Deserialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Deserialize", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (reader))
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
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Release")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Release", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Serialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Serialize", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::SyncStateId> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::GlobalNamespace::SyncStateId, 0usize>("get_id")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_id", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::SyncStateId = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_pool() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PacketPool_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
                        >,
                    >,
                >,
                0usize,
            >("get_pool")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_pool", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::StandardScoreSyncStateNetSerializable,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_state(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::StandardScoreSyncState> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::GlobalNamespace::StandardScoreSyncState,
                0usize,
            >("get_state")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_state", 0usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::StandardScoreSyncState = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_time(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i64, 0usize>("get_time")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_time", 0usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn set_id(
        &mut self,
        value: crate::GlobalNamespace::SyncStateId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::SyncStateId),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_id")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_id", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_state(
        &mut self,
        value: crate::GlobalNamespace::StandardScoreSyncState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::StandardScoreSyncState),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_state")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_state", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_time(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i64), quest_hook::libil2cpp::Void, 1usize>("set_time")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_time", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StandardScoreSyncStateNetSerializable")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::StandardScoreSyncStateNetSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "StandardScoreSyncStateNetSerializable")]
impl AsRef<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::StandardScoreSyncStateNetSerializable {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "StandardScoreSyncStateNetSerializable")]
impl AsMut<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::StandardScoreSyncStateNetSerializable {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "StandardScoreSyncStateNetSerializable")]
impl AsRef<
    crate::GlobalNamespace::ISyncStateSerializable_1<
        crate::GlobalNamespace::StandardScoreSyncState,
    >,
> for crate::GlobalNamespace::StandardScoreSyncStateNetSerializable {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::ISyncStateSerializable_1<
        crate::GlobalNamespace::StandardScoreSyncState,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "StandardScoreSyncStateNetSerializable")]
impl AsMut<
    crate::GlobalNamespace::ISyncStateSerializable_1<
        crate::GlobalNamespace::StandardScoreSyncState,
    >,
> for crate::GlobalNamespace::StandardScoreSyncStateNetSerializable {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::ISyncStateSerializable_1<
        crate::GlobalNamespace::StandardScoreSyncState,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "StandardScoreSyncStateNetSerializable")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::StandardScoreSyncStateNetSerializable {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "StandardScoreSyncStateNetSerializable")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::StandardScoreSyncStateNetSerializable {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
