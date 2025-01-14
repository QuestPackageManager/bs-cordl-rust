#[cfg(feature = "Oculus+Platform+Models+NetSyncSessionsChangedNotification")]
#[repr(C)]
#[derive(Debug)]
pub struct NetSyncSessionsChangedNotification {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ConnectionId: i64,
    pub Sessions: quest_hook::libil2cpp::Gc<
        crate::Oculus::Platform::Models::NetSyncSessionList,
    >,
}
#[cfg(feature = "Oculus+Platform+Models+NetSyncSessionsChangedNotification")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Oculus::Platform::Models::NetSyncSessionsChangedNotification {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform.Models";
    const CLASS_NAME: &'static str = "NetSyncSessionsChangedNotification";
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
#[cfg(feature = "Oculus+Platform+Models+NetSyncSessionsChangedNotification")]
impl std::ops::Deref
for crate::Oculus::Platform::Models::NetSyncSessionsChangedNotification {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+NetSyncSessionsChangedNotification")]
impl std::ops::DerefMut
for crate::Oculus::Platform::Models::NetSyncSessionsChangedNotification {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Models+NetSyncSessionsChangedNotification")]
impl crate::Oculus::Platform::Models::NetSyncSessionsChangedNotification {
    pub fn New(
        o: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (o))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        o: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (o))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Models+NetSyncSessionsChangedNotification")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::Models::NetSyncSessionsChangedNotification {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
