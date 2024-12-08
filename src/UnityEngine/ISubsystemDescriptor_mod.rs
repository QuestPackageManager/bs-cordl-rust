#[cfg(feature = "UnityEngine+ISubsystemDescriptor")]
#[repr(C)]
#[derive(Debug)]
pub struct ISubsystemDescriptor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ISubsystemDescriptor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ISubsystemDescriptor =>
    "UnityEngine"."ISubsystemDescriptor"
);
#[cfg(feature = "UnityEngine+ISubsystemDescriptor")]
impl std::ops::Deref for crate::UnityEngine::ISubsystemDescriptor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ISubsystemDescriptor")]
impl std::ops::DerefMut for crate::UnityEngine::ISubsystemDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ISubsystemDescriptor")]
impl crate::UnityEngine::ISubsystemDescriptor {
    pub fn get_id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_id", ())?;
        Ok(__cordl_ret)
    }
    pub fn Create(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::ISubsystem> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ISubsystem = __cordl_object
            .invoke("Create", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+ISubsystemDescriptor")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ISubsystemDescriptor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
