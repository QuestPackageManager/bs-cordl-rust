#[cfg(feature = "UnityEngine+CullingGroup")]
#[repr(C)]
#[derive(Debug)]
pub struct CullingGroup {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Ptr: crate::System::IntPtr,
    pub m_OnStateChanged: *mut crate::UnityEngine::CullingGroup_StateChanged,
}
#[cfg(feature = "UnityEngine+CullingGroup")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::CullingGroup => "UnityEngine"
    ."CullingGroup"
);
#[cfg(feature = "UnityEngine+CullingGroup")]
impl std::ops::Deref for crate::UnityEngine::CullingGroup {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+CullingGroup")]
impl std::ops::DerefMut for crate::UnityEngine::CullingGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+CullingGroup")]
impl crate::UnityEngine::CullingGroup {
    #[cfg(feature = "UnityEngine+CullingGroup+StateChanged")]
    pub type StateChanged = crate::UnityEngine::CullingGroup_StateChanged;
}
#[cfg(feature = "UnityEngine+CullingGroup")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::CullingGroup {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+CullingGroup+StateChanged")]
#[repr(C)]
#[derive(Debug)]
pub struct CullingGroup_StateChanged {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+CullingGroup+StateChanged")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::CullingGroup_StateChanged =>
    "UnityEngine"."CullingGroup/StateChanged"
);
#[cfg(feature = "UnityEngine+CullingGroup+StateChanged")]
impl std::ops::Deref for crate::UnityEngine::CullingGroup_StateChanged {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+CullingGroup+StateChanged")]
impl std::ops::DerefMut for crate::UnityEngine::CullingGroup_StateChanged {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+CullingGroup+StateChanged")]
impl crate::UnityEngine::CullingGroup_StateChanged {
    pub fn Invoke(
        &mut self,
        sphere: crate::UnityEngine::CullingGroupEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (sphere))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut quest_hook::libil2cpp::Il2CppObject,
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
        object: *mut quest_hook::libil2cpp::Il2CppObject,
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
#[cfg(feature = "UnityEngine+CullingGroup+StateChanged")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::CullingGroup_StateChanged {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
