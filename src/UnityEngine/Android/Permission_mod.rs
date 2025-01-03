#[cfg(feature = "UnityEngine+Android+Permission")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Permission {}
#[cfg(feature = "UnityEngine+Android+Permission")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Android::Permission =>
    "UnityEngine.Android"."Permission"
);
#[cfg(feature = "UnityEngine+Android+Permission")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Android::Permission {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Android+Permission")]
impl crate::UnityEngine::Android::Permission {
    pub fn GetUnityPermissions() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::AndroidJavaObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::AndroidJavaObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetUnityPermissions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasUserAuthorizedPermission(
        permission: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasUserAuthorizedPermission", (permission))?;
        Ok(__cordl_ret.into())
    }
    pub fn RequestUserPermission(
        permission: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callbacks: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Android::PermissionCallbacks,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RequestUserPermission", (permission, callbacks))?;
        Ok(__cordl_ret.into())
    }
    pub fn RequestUserPermissions(
        permissions: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        callbacks: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Android::PermissionCallbacks,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RequestUserPermissions", (permissions, callbacks))?;
        Ok(__cordl_ret.into())
    }
}
