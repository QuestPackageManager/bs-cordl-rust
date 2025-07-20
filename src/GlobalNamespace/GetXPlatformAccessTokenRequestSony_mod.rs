#[cfg(feature = "GetXPlatformAccessTokenRequestSony")]
#[repr(C)]
#[derive(Debug)]
pub struct GetXPlatformAccessTokenRequestSony {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub platformToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub platformEnvironment: crate::GlobalNamespace::PlatformEnvironment,
}
#[cfg(feature = "GetXPlatformAccessTokenRequestSony")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::GetXPlatformAccessTokenRequestSony {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "GetXPlatformAccessTokenRequestSony";
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
#[cfg(feature = "GetXPlatformAccessTokenRequestSony")]
impl std::ops::Deref for crate::GlobalNamespace::GetXPlatformAccessTokenRequestSony {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GetXPlatformAccessTokenRequestSony")]
impl std::ops::DerefMut for crate::GlobalNamespace::GetXPlatformAccessTokenRequestSony {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GetXPlatformAccessTokenRequestSony")]
impl crate::GlobalNamespace::GetXPlatformAccessTokenRequestSony {
    pub fn New(
        platformToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        platformEnvironment: crate::GlobalNamespace::PlatformEnvironment,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (platformToken, platformEnvironment))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        platformToken: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        platformEnvironment: crate::GlobalNamespace::PlatformEnvironment,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::GlobalNamespace::PlatformEnvironment,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (platformToken, platformEnvironment))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "GetXPlatformAccessTokenRequestSony")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GetXPlatformAccessTokenRequestSony {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
