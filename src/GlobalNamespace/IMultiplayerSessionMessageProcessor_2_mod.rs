#[cfg(feature = "cordl_class_IMultiplayerSessionMessageProcessor_2")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[repr(C)]
pub struct IMultiplayerSessionMessageProcessor_2<
    TMessageType: quest_hook::libil2cpp::Type,
    TConnectedPlayer: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TMessageType: std::marker::PhantomData<TMessageType>,
    __cordl_phantom_TConnectedPlayer: std::marker::PhantomData<TConnectedPlayer>,
}
#[cfg(feature = "cordl_class_IMultiplayerSessionMessageProcessor_2")]
unsafe impl<TMessageType: quest_hook::libil2cpp::Type, TConnectedPlayer: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::IMultiplayerSessionMessageProcessor_2<
        TMessageType,
        TConnectedPlayer,
    >
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IMultiplayerSessionMessageProcessor`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("", "IMultiplayerSessionMessageProcessor`2")
                .unwrap()
                .make_generic::<(TMessageType, TConnectedPlayer)>()
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
#[cfg(feature = "IMultiplayerSessionMessageProcessor_2")]
impl<TMessageType: quest_hook::libil2cpp::Type, TConnectedPlayer: quest_hook::libil2cpp::Type>
    std::ops::Deref
    for crate::GlobalNamespace::IMultiplayerSessionMessageProcessor_2<
        TMessageType,
        TConnectedPlayer,
    >
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IMultiplayerSessionMessageProcessor_2")]
impl<TMessageType: quest_hook::libil2cpp::Type, TConnectedPlayer: quest_hook::libil2cpp::Type>
    std::ops::DerefMut
    for crate::GlobalNamespace::IMultiplayerSessionMessageProcessor_2<
        TMessageType,
        TConnectedPlayer,
    >
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IMultiplayerSessionMessageProcessor_2")]
impl<TMessageType: quest_hook::libil2cpp::Type, TConnectedPlayer: quest_hook::libil2cpp::Type>
    crate::GlobalNamespace::IMultiplayerSessionMessageProcessor_2<TMessageType, TConnectedPlayer>
{
    pub fn RegisterCallback<T>(
        &mut self,
        serializerType: TMessageType,
        callback: quest_hook::libil2cpp::Gc<crate::System::Action_2<T, TConnectedPlayer>>,
        constructor: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TMessageType: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TConnectedPlayer: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        TMessageType,
                        quest_hook::libil2cpp::Gc<crate::System::Action_2<T, TConnectedPlayer>>,
                        quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
                    ), quest_hook::libil2cpp::Void, 3usize>("RegisterCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RegisterCallback",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (serializerType, callback, constructor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RegisterSerializer(
        &mut self,
        serializerType: TMessageType,
        subSerializer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::INetworkPacketSubSerializer_1<TConnectedPlayer>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TMessageType: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TConnectedPlayer: quest_hook::libil2cpp::Type
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
                        TMessageType,
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::INetworkPacketSubSerializer_1<TConnectedPlayer>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("RegisterSerializer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RegisterSerializer",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (serializerType, subSerializer))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterCallback<T>(
        &mut self,
        serializerType: TMessageType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TMessageType: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TConnectedPlayer: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(TMessageType), quest_hook::libil2cpp::Void, 1usize>(
                        "UnregisterCallback",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnregisterCallback",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (serializerType))? };
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterSerializer(
        &mut self,
        serializerType: TMessageType,
        subSerializer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::INetworkPacketSubSerializer_1<TConnectedPlayer>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TMessageType: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TConnectedPlayer: quest_hook::libil2cpp::Type
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
                        TMessageType,
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::INetworkPacketSubSerializer_1<TConnectedPlayer>,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "UnregisterSerializer"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "UnregisterSerializer",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (serializerType, subSerializer))? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_IMultiplayerSessionMessageProcessor_2")]
impl<TMessageType: quest_hook::libil2cpp::Type, TConnectedPlayer: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::IMultiplayerSessionMessageProcessor_2<
        TMessageType,
        TConnectedPlayer,
    >
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
