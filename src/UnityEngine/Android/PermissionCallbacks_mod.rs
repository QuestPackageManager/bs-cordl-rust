#[cfg(feature = "UnityEngine+Android+PermissionCallbacks")]
#[repr(C)]
#[derive(Debug)]
pub struct PermissionCallbacks {
    __cordl_parent: crate::UnityEngine::AndroidJavaProxy,
    pub PermissionGranted: *mut crate::System::Action_1<*mut crate::System::String>,
    pub PermissionDenied: *mut crate::System::Action_1<*mut crate::System::String>,
    pub PermissionDeniedAndDontAskAgain: *mut crate::System::Action_1<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "UnityEngine+Android+PermissionCallbacks")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Android::PermissionCallbacks =>
    "UnityEngine.Android"."PermissionCallbacks"
);
#[cfg(feature = "UnityEngine+Android+PermissionCallbacks")]
impl std::ops::Deref for crate::UnityEngine::Android::PermissionCallbacks {
    type Target = crate::UnityEngine::AndroidJavaProxy;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+PermissionCallbacks")]
impl std::ops::DerefMut for crate::UnityEngine::Android::PermissionCallbacks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Android+PermissionCallbacks")]
impl crate::UnityEngine::Android::PermissionCallbacks {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn add_PermissionDenied(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_PermissionDenied", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_PermissionDeniedAndDontAskAgain(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_PermissionDeniedAndDontAskAgain", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_PermissionGranted(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_PermissionGranted", (value))?;
        Ok(__cordl_ret)
    }
    pub fn onPermissionDenied(
        &mut self,
        permissionName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("onPermissionDenied", (permissionName))?;
        Ok(__cordl_ret)
    }
    pub fn onPermissionDeniedAndDontAskAgain(
        &mut self,
        permissionName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("onPermissionDeniedAndDontAskAgain", (permissionName))?;
        Ok(__cordl_ret)
    }
    pub fn onPermissionGranted(
        &mut self,
        permissionName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("onPermissionGranted", (permissionName))?;
        Ok(__cordl_ret)
    }
    pub fn remove_PermissionDenied(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_PermissionDenied", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_PermissionDeniedAndDontAskAgain(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_PermissionDeniedAndDontAskAgain", (value))?;
        Ok(__cordl_ret)
    }
    pub fn remove_PermissionGranted(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_PermissionGranted", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+Android+PermissionCallbacks")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Android::PermissionCallbacks {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}