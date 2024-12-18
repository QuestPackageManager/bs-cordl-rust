#[cfg(feature = "UnityEngine+AudioSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+AudioSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AudioSettings => "UnityEngine"
    ."AudioSettings"
);
#[cfg(feature = "UnityEngine+AudioSettings")]
impl std::ops::Deref for crate::UnityEngine::AudioSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AudioSettings")]
impl std::ops::DerefMut for crate::UnityEngine::AudioSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AudioSettings")]
impl crate::UnityEngine::AudioSettings {
    #[cfg(feature = "UnityEngine+AudioSettings+AudioConfigurationChangeHandler")]
    pub type AudioConfigurationChangeHandler = crate::UnityEngine::AudioSettings_AudioConfigurationChangeHandler;
    #[cfg(feature = "UnityEngine+AudioSettings+Mobile")]
    pub type Mobile = crate::UnityEngine::AudioSettings_Mobile;
}
#[cfg(feature = "UnityEngine+AudioSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AudioSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+AudioSettings+AudioConfigurationChangeHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioSettings_AudioConfigurationChangeHandler {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+AudioSettings+AudioConfigurationChangeHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AudioSettings_AudioConfigurationChangeHandler => "UnityEngine"
    ."AudioSettings/AudioConfigurationChangeHandler"
);
#[cfg(feature = "UnityEngine+AudioSettings+AudioConfigurationChangeHandler")]
impl std::ops::Deref
for crate::UnityEngine::AudioSettings_AudioConfigurationChangeHandler {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AudioSettings+AudioConfigurationChangeHandler")]
impl std::ops::DerefMut
for crate::UnityEngine::AudioSettings_AudioConfigurationChangeHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AudioSettings+AudioConfigurationChangeHandler")]
impl crate::UnityEngine::AudioSettings_AudioConfigurationChangeHandler {
    pub fn Invoke(
        &mut self,
        deviceWasChanged: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (deviceWasChanged))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AudioSettings+AudioConfigurationChangeHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AudioSettings_AudioConfigurationChangeHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+AudioSettings+Mobile")]
#[repr(C)]
#[derive(Debug)]
pub struct AudioSettings_Mobile {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+AudioSettings+Mobile")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AudioSettings_Mobile =>
    "UnityEngine"."AudioSettings/Mobile"
);
#[cfg(feature = "UnityEngine+AudioSettings+Mobile")]
impl std::ops::Deref for crate::UnityEngine::AudioSettings_Mobile {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AudioSettings+Mobile")]
impl std::ops::DerefMut for crate::UnityEngine::AudioSettings_Mobile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AudioSettings+Mobile")]
impl crate::UnityEngine::AudioSettings_Mobile {}
#[cfg(feature = "UnityEngine+AudioSettings+Mobile")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AudioSettings_Mobile {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
