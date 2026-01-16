#[cfg(feature = "cordl_class_IMultiplayerSessionManager_4")]
#[repr(C)]
#[derive(Debug)]
pub struct IMultiplayerSessionManager_4<
    TConnectedPlayerManager: quest_hook::libil2cpp::Type,
    TConnectedPlayer: quest_hook::libil2cpp::Type,
    TConnectedPlayerImpl: quest_hook::libil2cpp::Type,
    TGameSpecificIdentityData: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TConnectedPlayerManager: std::marker::PhantomData<
        TConnectedPlayerManager,
    >,
    __cordl_phantom_TConnectedPlayer: std::marker::PhantomData<TConnectedPlayer>,
    __cordl_phantom_TConnectedPlayerImpl: std::marker::PhantomData<TConnectedPlayerImpl>,
    __cordl_phantom_TGameSpecificIdentityData: std::marker::PhantomData<
        TGameSpecificIdentityData,
    >,
}
#[cfg(feature = "cordl_class_IMultiplayerSessionManager_4")]
unsafe impl<
    TConnectedPlayerManager: quest_hook::libil2cpp::Type,
    TConnectedPlayer: quest_hook::libil2cpp::Type,
    TConnectedPlayerImpl: quest_hook::libil2cpp::Type,
    TGameSpecificIdentityData: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IMultiplayerSessionManager_4<
    TConnectedPlayerManager,
    TConnectedPlayer,
    TConnectedPlayerImpl,
    TGameSpecificIdentityData,
> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IMultiplayerSessionManager`4";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "",
                        "IMultiplayerSessionManager`4",
                    )
                    .unwrap()
                    .make_generic::<
                        (
                            TConnectedPlayerManager,
                            TConnectedPlayer,
                            TConnectedPlayerImpl,
                            TGameSpecificIdentityData,
                        ),
                    >()
                    .unwrap()
                    .unwrap()
            })
    }
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
#[cfg(feature = "IMultiplayerSessionManager_4")]
impl<
    TConnectedPlayerManager: quest_hook::libil2cpp::Type,
    TConnectedPlayer: quest_hook::libil2cpp::Type,
    TConnectedPlayerImpl: quest_hook::libil2cpp::Type,
    TGameSpecificIdentityData: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::GlobalNamespace::IMultiplayerSessionManager_4<
    TConnectedPlayerManager,
    TConnectedPlayer,
    TConnectedPlayerImpl,
    TGameSpecificIdentityData,
> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IMultiplayerSessionManager_4")]
impl<
    TConnectedPlayerManager: quest_hook::libil2cpp::Type,
    TConnectedPlayer: quest_hook::libil2cpp::Type,
    TConnectedPlayerImpl: quest_hook::libil2cpp::Type,
    TGameSpecificIdentityData: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::GlobalNamespace::IMultiplayerSessionManager_4<
    TConnectedPlayerManager,
    TConnectedPlayer,
    TConnectedPlayerImpl,
    TGameSpecificIdentityData,
> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IMultiplayerSessionManager_4")]
impl<
    TConnectedPlayerManager: quest_hook::libil2cpp::Type,
    TConnectedPlayer: quest_hook::libil2cpp::Type,
    TConnectedPlayerImpl: quest_hook::libil2cpp::Type,
    TGameSpecificIdentityData: quest_hook::libil2cpp::Type,
> crate::GlobalNamespace::IMultiplayerSessionManager_4<
    TConnectedPlayerManager,
    TConnectedPlayer,
    TConnectedPlayerImpl,
    TGameSpecificIdentityData,
> {
    pub fn StartSession(
        &mut self,
        connectedPlayerManager: TConnectedPlayerManager,
        multiplayerSessionInitializer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager_1_IMultiplayerSessionInitializer<
                TConnectedPlayer,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TConnectedPlayerManager: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TConnectedPlayer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TConnectedPlayerImpl: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TGameSpecificIdentityData: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            TConnectedPlayerManager,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IMultiplayerSessionManager_1_IMultiplayerSessionInitializer<
                                    TConnectedPlayer,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("StartSession")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StartSession", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (connectedPlayerManager, multiplayerSessionInitializer),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_IMultiplayerSessionManager_4")]
impl<
    TConnectedPlayerManager: quest_hook::libil2cpp::Type,
    TConnectedPlayer: quest_hook::libil2cpp::Type,
    TConnectedPlayerImpl: quest_hook::libil2cpp::Type,
    TGameSpecificIdentityData: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IMultiplayerSessionManager_4<
    TConnectedPlayerManager,
    TConnectedPlayer,
    TConnectedPlayerImpl,
    TGameSpecificIdentityData,
> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IMultiplayerSessionManager_4")]
impl<
    TConnectedPlayerManager: quest_hook::libil2cpp::Type,
    TConnectedPlayer: quest_hook::libil2cpp::Type,
    TConnectedPlayerImpl: quest_hook::libil2cpp::Type,
    TGameSpecificIdentityData: quest_hook::libil2cpp::Type,
> AsRef<crate::GlobalNamespace::IMultiplayerSessionManager_1<TConnectedPlayer>>
for crate::GlobalNamespace::IMultiplayerSessionManager_4<
    TConnectedPlayerManager,
    TConnectedPlayer,
    TConnectedPlayerImpl,
    TGameSpecificIdentityData,
> {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IMultiplayerSessionManager_1<TConnectedPlayer> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IMultiplayerSessionManager_4")]
impl<
    TConnectedPlayerManager: quest_hook::libil2cpp::Type,
    TConnectedPlayer: quest_hook::libil2cpp::Type,
    TConnectedPlayerImpl: quest_hook::libil2cpp::Type,
    TGameSpecificIdentityData: quest_hook::libil2cpp::Type,
> AsMut<crate::GlobalNamespace::IMultiplayerSessionManager_1<TConnectedPlayer>>
for crate::GlobalNamespace::IMultiplayerSessionManager_4<
    TConnectedPlayerManager,
    TConnectedPlayer,
    TConnectedPlayerImpl,
    TGameSpecificIdentityData,
> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IMultiplayerSessionManager_1<TConnectedPlayer> {
        unsafe { std::mem::transmute(self) }
    }
}
