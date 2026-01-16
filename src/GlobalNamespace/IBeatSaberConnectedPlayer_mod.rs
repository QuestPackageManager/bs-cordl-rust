#[cfg(feature = "cordl_class_IBeatSaberConnectedPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct IBeatSaberConnectedPlayer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_IBeatSaberConnectedPlayer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::IBeatSaberConnectedPlayer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IBeatSaberConnectedPlayer";
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
#[cfg(feature = "IBeatSaberConnectedPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::IBeatSaberConnectedPlayer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IBeatSaberConnectedPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::IBeatSaberConnectedPlayer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IBeatSaberConnectedPlayer")]
impl crate::GlobalNamespace::IBeatSaberConnectedPlayer {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_multiplayerAvatarsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::MultiplayerAvatarsData> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::MultiplayerAvatarsData,
                        0usize,
                    >("get_multiplayerAvatarsData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_multiplayerAvatarsData", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::MultiplayerAvatarsData = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_IBeatSaberConnectedPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IBeatSaberConnectedPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IBeatSaberConnectedPlayer")]
impl AsRef<crate::GlobalNamespace::IConnectedPlayer>
for crate::GlobalNamespace::IBeatSaberConnectedPlayer {
    fn as_ref(&self) -> &crate::GlobalNamespace::IConnectedPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IBeatSaberConnectedPlayer")]
impl AsMut<crate::GlobalNamespace::IConnectedPlayer>
for crate::GlobalNamespace::IBeatSaberConnectedPlayer {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IConnectedPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
