#[cfg(feature = "UnityEngine+Experimental+Rendering+BuiltinRuntimeReflectionSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct BuiltinRuntimeReflectionSystem {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+BuiltinRuntimeReflectionSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Experimental::Rendering::BuiltinRuntimeReflectionSystem =>
    "UnityEngine.Experimental.Rendering"."BuiltinRuntimeReflectionSystem"
);
#[cfg(feature = "UnityEngine+Experimental+Rendering+BuiltinRuntimeReflectionSystem")]
impl std::ops::Deref
for crate::UnityEngine::Experimental::Rendering::BuiltinRuntimeReflectionSystem {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+BuiltinRuntimeReflectionSystem")]
impl std::ops::DerefMut
for crate::UnityEngine::Experimental::Rendering::BuiltinRuntimeReflectionSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+BuiltinRuntimeReflectionSystem")]
impl crate::UnityEngine::Experimental::Rendering::BuiltinRuntimeReflectionSystem {
    pub fn BuiltinUpdate() -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BuiltinUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_BuiltinRuntimeReflectionSystem_New() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Experimental::Rendering::BuiltinRuntimeReflectionSystem,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Experimental::Rendering::BuiltinRuntimeReflectionSystem,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_BuiltinRuntimeReflectionSystem_New", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn TickRealtimeProbes(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TickRealtimeProbes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+BuiltinRuntimeReflectionSystem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Experimental::Rendering::BuiltinRuntimeReflectionSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+BuiltinRuntimeReflectionSystem")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::UnityEngine::Experimental::Rendering::BuiltinRuntimeReflectionSystem {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+BuiltinRuntimeReflectionSystem")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IDisposable>>
for crate::UnityEngine::Experimental::Rendering::BuiltinRuntimeReflectionSystem {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IDisposable> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+BuiltinRuntimeReflectionSystem")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem,
    >,
> for crate::UnityEngine::Experimental::Rendering::BuiltinRuntimeReflectionSystem {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Experimental+Rendering+BuiltinRuntimeReflectionSystem")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem,
    >,
> for crate::UnityEngine::Experimental::Rendering::BuiltinRuntimeReflectionSystem {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Experimental::Rendering::IScriptableRuntimeReflectionSystem,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
