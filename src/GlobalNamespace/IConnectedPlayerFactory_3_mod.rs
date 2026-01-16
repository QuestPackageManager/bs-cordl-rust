#[cfg(feature = "cordl_class_IConnectedPlayerFactory_3")]
#[derive(Debug)]
#[repr(C)]
pub struct IConnectedPlayerFactory_3<
    TConnectedPlayer: quest_hook::libil2cpp::Type,
    TConnectedPlayerImpl: quest_hook::libil2cpp::Type,
    TGameSpecificIdentityData: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TConnectedPlayer: std::marker::PhantomData<TConnectedPlayer>,
    __cordl_phantom_TConnectedPlayerImpl: std::marker::PhantomData<TConnectedPlayerImpl>,
    __cordl_phantom_TGameSpecificIdentityData: std::marker::PhantomData<TGameSpecificIdentityData>,
}
#[cfg(feature = "cordl_class_IConnectedPlayerFactory_3")]
unsafe impl<
        TConnectedPlayer: quest_hook::libil2cpp::Type,
        TConnectedPlayerImpl: quest_hook::libil2cpp::Type,
        TGameSpecificIdentityData: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::IConnectedPlayerFactory_3<
        TConnectedPlayer,
        TConnectedPlayerImpl,
        TGameSpecificIdentityData,
    >
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IConnectedPlayerFactory`3";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("", "IConnectedPlayerFactory`3")
                .unwrap()
                .make_generic::<(
                    TConnectedPlayer,
                    TConnectedPlayerImpl,
                    TGameSpecificIdentityData,
                )>()
                .unwrap()
                .unwrap()
        })
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "IConnectedPlayerFactory_3")]
impl<
        TConnectedPlayer: quest_hook::libil2cpp::Type,
        TConnectedPlayerImpl: quest_hook::libil2cpp::Type,
        TGameSpecificIdentityData: quest_hook::libil2cpp::Type,
    > std::ops::Deref
    for crate::GlobalNamespace::IConnectedPlayerFactory_3<
        TConnectedPlayer,
        TConnectedPlayerImpl,
        TGameSpecificIdentityData,
    >
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IConnectedPlayerFactory_3")]
impl<
        TConnectedPlayer: quest_hook::libil2cpp::Type,
        TConnectedPlayerImpl: quest_hook::libil2cpp::Type,
        TGameSpecificIdentityData: quest_hook::libil2cpp::Type,
    > std::ops::DerefMut
    for crate::GlobalNamespace::IConnectedPlayerFactory_3<
        TConnectedPlayer,
        TConnectedPlayerImpl,
        TGameSpecificIdentityData,
    >
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IConnectedPlayerFactory_3")]
impl<
        TConnectedPlayer: quest_hook::libil2cpp::Type,
        TConnectedPlayerImpl: quest_hook::libil2cpp::Type,
        TGameSpecificIdentityData: quest_hook::libil2cpp::Type,
    >
    crate::GlobalNamespace::IConnectedPlayerFactory_3<
        TConnectedPlayer,
        TConnectedPlayerImpl,
        TGameSpecificIdentityData,
    >
{
    pub fn CreateDirectlyConnectedPlayer(
        &mut self,
        manager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_3<
                TConnectedPlayer,
                TConnectedPlayerImpl,
                TGameSpecificIdentityData,
            >,
        >,
        connectionId: u8,
        connection: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
    ) -> quest_hook::libil2cpp::Result<TConnectedPlayerImpl>
    where
        TConnectedPlayer: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TConnectedPlayerImpl: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TGameSpecificIdentityData: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::ConnectedPlayerManager_3<
                                TConnectedPlayer,
                                TConnectedPlayerImpl,
                                TGameSpecificIdentityData,
                            >,
                        >,
                        u8,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
                    ), TConnectedPlayerImpl, 3usize>(
                        "CreateDirectlyConnectedPlayer"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateDirectlyConnectedPlayer",
                            3usize
                        )
                    })
            });
        let __cordl_ret: TConnectedPlayerImpl = unsafe {
            cordl_method_info.invoke_unchecked(self, (manager, connectionId, connection))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateLocalPlayer(
        &mut self,
        manager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_3<
                TConnectedPlayer,
                TConnectedPlayerImpl,
                TGameSpecificIdentityData,
            >,
        >,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isConnectionOwner: bool,
        publicEncryptionKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        random: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        compatibilityVersion: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<TConnectedPlayerImpl>
    where
        TConnectedPlayer: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TConnectedPlayerImpl: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TGameSpecificIdentityData: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::ConnectedPlayerManager_3<
                                TConnectedPlayer,
                                TConnectedPlayerImpl,
                                TGameSpecificIdentityData,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        bool,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), TConnectedPlayerImpl, 7usize>("CreateLocalPlayer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateLocalPlayer",
                            7usize
                        )
                    })
            });
        let __cordl_ret: TConnectedPlayerImpl = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    manager,
                    userId,
                    userName,
                    isConnectionOwner,
                    publicEncryptionKey,
                    random,
                    compatibilityVersion,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRemoteConnectedPlayer(
        &mut self,
        manager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_3<
                TConnectedPlayer,
                TConnectedPlayerImpl,
                TGameSpecificIdentityData,
            >,
        >,
        connectionId: u8,
        packet: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerConnectedPacket>,
        parent: TConnectedPlayerImpl,
    ) -> quest_hook::libil2cpp::Result<TConnectedPlayerImpl>
    where
        TConnectedPlayer: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TConnectedPlayerImpl: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TGameSpecificIdentityData: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::ConnectedPlayerManager_3<
                                TConnectedPlayer,
                                TConnectedPlayerImpl,
                                TGameSpecificIdentityData,
                            >,
                        >,
                        u8,
                        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerConnectedPacket>,
                        TConnectedPlayerImpl,
                    ), TConnectedPlayerImpl, 4usize>(
                        "CreateRemoteConnectedPlayer"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateRemoteConnectedPlayer",
                            4usize
                        )
                    })
            });
        let __cordl_ret: TConnectedPlayerImpl = unsafe {
            cordl_method_info.invoke_unchecked(self, (manager, connectionId, packet, parent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_IConnectedPlayerFactory_3")]
impl<
        TConnectedPlayer: quest_hook::libil2cpp::Type,
        TConnectedPlayerImpl: quest_hook::libil2cpp::Type,
        TGameSpecificIdentityData: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::IConnectedPlayerFactory_3<
        TConnectedPlayer,
        TConnectedPlayerImpl,
        TGameSpecificIdentityData,
    >
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
