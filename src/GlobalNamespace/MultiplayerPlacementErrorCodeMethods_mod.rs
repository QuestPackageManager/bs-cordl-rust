#[cfg(feature = "MultiplayerPlacementErrorCodeMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerPlacementErrorCodeMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "MultiplayerPlacementErrorCodeMethods")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerPlacementErrorCodeMethods {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerPlacementErrorCodeMethods";
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
#[cfg(feature = "MultiplayerPlacementErrorCodeMethods")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerPlacementErrorCodeMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerPlacementErrorCodeMethods")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerPlacementErrorCodeMethods {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerPlacementErrorCodeMethods")]
impl crate::GlobalNamespace::MultiplayerPlacementErrorCodeMethods {
    pub fn ToConnectionFailedReason(
        errorCode: crate::GlobalNamespace::MultiplayerPlacementErrorCode,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::ConnectionFailedReason> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::MultiplayerPlacementErrorCode),
                        crate::GlobalNamespace::ConnectionFailedReason,
                        1usize,
                    >("ToConnectionFailedReason")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToConnectionFailedReason", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::ConnectionFailedReason = unsafe {
            method.invoke_unchecked((), (errorCode))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerPlacementErrorCodeMethods")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerPlacementErrorCodeMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
