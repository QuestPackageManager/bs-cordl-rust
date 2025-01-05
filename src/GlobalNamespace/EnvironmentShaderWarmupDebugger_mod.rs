#[cfg(feature = "EnvironmentShaderWarmupDebugger")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentShaderWarmupDebugger {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _renderersThatWereNotRendered: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Renderer>,
        >,
    >,
    pub _materialsThatWereNotRendered: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        >,
    >,
    pub _reusableStringBuilder: quest_hook::libil2cpp::Gc<
        crate::System::Text::StringBuilder,
    >,
}
#[cfg(feature = "EnvironmentShaderWarmupDebugger")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::EnvironmentShaderWarmupDebugger
    => ""."EnvironmentShaderWarmupDebugger"
);
#[cfg(feature = "EnvironmentShaderWarmupDebugger")]
impl std::ops::Deref for crate::GlobalNamespace::EnvironmentShaderWarmupDebugger {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentShaderWarmupDebugger")]
impl std::ops::DerefMut for crate::GlobalNamespace::EnvironmentShaderWarmupDebugger {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentShaderWarmupDebugger")]
impl crate::GlobalNamespace::EnvironmentShaderWarmupDebugger {
    pub fn GetGameObjectPath(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetGameObjectPath", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LateTick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateTick", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "EnvironmentShaderWarmupDebugger")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EnvironmentShaderWarmupDebugger {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EnvironmentShaderWarmupDebugger")]
impl AsRef<crate::Zenject::IInitializable>
for crate::GlobalNamespace::EnvironmentShaderWarmupDebugger {
    fn as_ref(&self) -> &crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "EnvironmentShaderWarmupDebugger")]
impl AsMut<crate::Zenject::IInitializable>
for crate::GlobalNamespace::EnvironmentShaderWarmupDebugger {
    fn as_mut(&mut self) -> &mut crate::Zenject::IInitializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "EnvironmentShaderWarmupDebugger")]
impl AsRef<crate::Zenject::ILateTickable>
for crate::GlobalNamespace::EnvironmentShaderWarmupDebugger {
    fn as_ref(&self) -> &crate::Zenject::ILateTickable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "EnvironmentShaderWarmupDebugger")]
impl AsMut<crate::Zenject::ILateTickable>
for crate::GlobalNamespace::EnvironmentShaderWarmupDebugger {
    fn as_mut(&mut self) -> &mut crate::Zenject::ILateTickable {
        unsafe { std::mem::transmute(self) }
    }
}
