#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_SessionManager {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_SessionManager =>
    "HoudiniEngineUnity"."HEU_SessionManager"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_SessionManager {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_SessionManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager")]
impl crate::HoudiniEngineUnity::HEU_SessionManager {
    #[cfg(
        feature = "HoudiniEngineUnity+HEU_SessionManager+CreateSessionFromTypeDelegate"
    )]
    pub type CreateSessionFromTypeDelegate = crate::HoudiniEngineUnity::HEU_SessionManager_CreateSessionFromTypeDelegate;
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_SessionManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager+CreateSessionFromTypeDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_SessionManager_CreateSessionFromTypeDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager+CreateSessionFromTypeDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_SessionManager_CreateSessionFromTypeDelegate =>
    "HoudiniEngineUnity"."HEU_SessionManager/CreateSessionFromTypeDelegate"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager+CreateSessionFromTypeDelegate")]
impl std::ops::Deref
for crate::HoudiniEngineUnity::HEU_SessionManager_CreateSessionFromTypeDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager+CreateSessionFromTypeDelegate")]
impl std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_SessionManager_CreateSessionFromTypeDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager+CreateSessionFromTypeDelegate")]
impl crate::HoudiniEngineUnity::HEU_SessionManager_CreateSessionFromTypeDelegate {
    pub fn BeginInvoke(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (_cordl_type, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_SessionBase> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_SessionBase = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::HoudiniEngineUnity::HEU_SessionBase> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::HoudiniEngineUnity::HEU_SessionBase = __cordl_object
            .invoke("Invoke", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_SessionManager+CreateSessionFromTypeDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_SessionManager_CreateSessionFromTypeDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
