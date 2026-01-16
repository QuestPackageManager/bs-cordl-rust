#[cfg(feature = "cordl_class_ConnectedPlayerManager")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_ConnectedPlayerManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ConnectedPlayerManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ConnectedPlayerManager";
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
#[cfg(feature = "ConnectedPlayerManager")]
impl std::ops::Deref for crate::GlobalNamespace::ConnectedPlayerManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::ConnectedPlayerManager {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectedPlayerManager")]
impl crate::GlobalNamespace::ConnectedPlayerManager {
    pub const invalidSortIndex: i32 = -1i32;
    pub const kAllConnectionsId: u8 = 127u8;
    pub const kLocalConnectionId: u8 = 0u8;
    pub const kMaxUnreliableMessageLength: i32 = 412i32;
    pub const kPingUpdateFrequencyMs: i64 = 2000i64;
    pub const kTimeSensitiveAllowedReceiveWindowMs: i64 = 30i64;
    #[cfg(feature = "ConnectedPlayerManager+InternalMessageType")]
    pub type InternalMessageType = crate::GlobalNamespace::ConnectedPlayerManager_InternalMessageType;
    #[cfg(feature = "ConnectedPlayerManager+MessageType")]
    pub type MessageType = crate::GlobalNamespace::ConnectedPlayerManager_MessageType;
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
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_ConnectedPlayerManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConnectedPlayerManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_ConnectedPlayerManager+InternalMessageType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectedPlayerManager_InternalMessageType {
    #[default]
    GameSpecificMessageType = 12u8,
    KickPlayer = 8u8,
    MultiplayerSession = 7u8,
    Party = 6u8,
    Ping = 10u8,
    PlayerConnected = 1u8,
    PlayerDisconnected = 4u8,
    PlayerIdentity = 2u8,
    PlayerLatencyUpdate = 3u8,
    PlayerSortOrderUpdate = 5u8,
    PlayerStateUpdate = 9u8,
    Pong = 11u8,
    SyncTime = 0u8,
}
#[cfg(feature = "cordl_class_ConnectedPlayerManager+InternalMessageType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ConnectedPlayerManager_InternalMessageType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ConnectedPlayerManager/InternalMessageType";
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
#[cfg(feature = "cordl_class_ConnectedPlayerManager+InternalMessageType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::ConnectedPlayerManager_InternalMessageType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_ConnectedPlayerManager+InternalMessageType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::ConnectedPlayerManager_InternalMessageType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_ConnectedPlayerManager+InternalMessageType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::ConnectedPlayerManager_InternalMessageType {
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
#[cfg(feature = "cordl_class_ConnectedPlayerManager+InternalMessageType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::ConnectedPlayerManager_InternalMessageType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_ConnectedPlayerManager+MessageType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectedPlayerManager_MessageType {
    #[default]
    GameSpecificMessageType = 12u8,
    MultiplayerSession = 7u8,
    Party = 6u8,
    PlayerIdentity = 2u8,
}
#[cfg(feature = "cordl_class_ConnectedPlayerManager+MessageType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ConnectedPlayerManager_MessageType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ConnectedPlayerManager/MessageType";
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
#[cfg(feature = "cordl_class_ConnectedPlayerManager+MessageType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::ConnectedPlayerManager_MessageType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_ConnectedPlayerManager+MessageType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::ConnectedPlayerManager_MessageType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_ConnectedPlayerManager+MessageType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::ConnectedPlayerManager_MessageType {
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
#[cfg(feature = "cordl_class_ConnectedPlayerManager+MessageType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::ConnectedPlayerManager_MessageType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
