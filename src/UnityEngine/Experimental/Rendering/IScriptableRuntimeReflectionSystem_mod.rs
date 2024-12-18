#[cfg(feature = "UnityEngine+Experimental+Rendering+IScriptableRuntimeReflectionSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct IScriptableRuntimeReflectionSystem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+IScriptableRuntimeReflectionSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem =>
    "UnityEngine.Experimental.Rendering"."IScriptableRuntimeReflectionSystem"
);
#[cfg(feature = "UnityEngine+Experimental+Rendering+IScriptableRuntimeReflectionSystem")]
impl std::ops::Deref
for crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+IScriptableRuntimeReflectionSystem")]
impl std::ops::DerefMut
for crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+IScriptableRuntimeReflectionSystem")]
impl crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem {
    pub fn TickRealtimeProbes(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TickRealtimeProbes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+IScriptableRuntimeReflectionSystem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+IScriptableRuntimeReflectionSystem")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+IScriptableRuntimeReflectionSystem")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
