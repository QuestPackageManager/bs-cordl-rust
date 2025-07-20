#[cfg(feature = "MultiplayerConnectedPlayerObservable")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerConnectedPlayerObservable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _connectedPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IConnectedPlayer,
    >,
}
#[cfg(feature = "MultiplayerConnectedPlayerObservable")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerConnectedPlayerObservable {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerConnectedPlayerObservable";
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
#[cfg(feature = "MultiplayerConnectedPlayerObservable")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerConnectedPlayerObservable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerObservable")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerConnectedPlayerObservable {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerObservable")]
impl crate::GlobalNamespace::MultiplayerConnectedPlayerObservable {
    pub fn New(
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (connectedPlayer))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::IConnectedPlayer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (connectedPlayer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_isFailed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_isFailed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_isFailed", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_offsetSyncTime(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i64, 0usize>("get_offsetSyncTime")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_offsetSyncTime", 0usize
                        )
                    })
            });
        let __cordl_ret: i64 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerObservable")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerConnectedPlayerObservable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerObservable")]
impl AsRef<crate::GlobalNamespace::IMultiplayerObservable>
for crate::GlobalNamespace::MultiplayerConnectedPlayerObservable {
    fn as_ref(&self) -> &crate::GlobalNamespace::IMultiplayerObservable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MultiplayerConnectedPlayerObservable")]
impl AsMut<crate::GlobalNamespace::IMultiplayerObservable>
for crate::GlobalNamespace::MultiplayerConnectedPlayerObservable {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IMultiplayerObservable {
        unsafe { std::mem::transmute(self) }
    }
}
