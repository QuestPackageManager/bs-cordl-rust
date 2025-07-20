#[cfg(feature = "NoInvitePlatformHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct NoInvitePlatformHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "NoInvitePlatformHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::NoInvitePlatformHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "NoInvitePlatformHandler";
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
#[cfg(feature = "NoInvitePlatformHandler")]
impl std::ops::Deref for crate::GlobalNamespace::NoInvitePlatformHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "NoInvitePlatformHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::NoInvitePlatformHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "NoInvitePlatformHandler")]
impl crate::GlobalNamespace::NoInvitePlatformHandler {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OpenInvitePanel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::NoInvitePlatformHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OpenInvitePanel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::NoInvitePlatformHandler as
                    quest_hook::libil2cpp::Type > ::class(), "OpenInvitePanel", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::NoInvitePlatformHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::NoInvitePlatformHandler as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_isSupported(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::NoInvitePlatformHandler as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isSupported")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::NoInvitePlatformHandler as
                    quest_hook::libil2cpp::Type > ::class(), "get_isSupported", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "NoInvitePlatformHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::NoInvitePlatformHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "NoInvitePlatformHandler")]
impl AsRef<crate::GlobalNamespace::IInvitePlatformHandler>
for crate::GlobalNamespace::NoInvitePlatformHandler {
    fn as_ref(&self) -> &crate::GlobalNamespace::IInvitePlatformHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "NoInvitePlatformHandler")]
impl AsMut<crate::GlobalNamespace::IInvitePlatformHandler>
for crate::GlobalNamespace::NoInvitePlatformHandler {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IInvitePlatformHandler {
        unsafe { std::mem::transmute(self) }
    }
}
