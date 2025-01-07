#[cfg(feature = "UnityEngine+ResourceManagement+Diagnostics+DiagnosticEventCollector")]
#[repr(C)]
#[derive(Debug)]
pub struct DiagnosticEventCollector {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
}
#[cfg(feature = "UnityEngine+ResourceManagement+Diagnostics+DiagnosticEventCollector")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEventCollector {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.Diagnostics";
    const CLASS_NAME: &'static str = "DiagnosticEventCollector";
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
#[cfg(feature = "UnityEngine+ResourceManagement+Diagnostics+DiagnosticEventCollector")]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEventCollector {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Diagnostics+DiagnosticEventCollector")]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEventCollector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Diagnostics+DiagnosticEventCollector")]
impl crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEventCollector {
    pub fn FindOrCreateGlobalInstance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEventCollector,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEventCollector,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindOrCreateGlobalInstance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PostEvent(
        &mut self,
        diagnosticEvent: crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PostEvent", (diagnosticEvent))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterEventHandler(
        handler: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent,
            >,
        >,
        _cordl_register: bool,
        create: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RegisterEventHandler", (handler, _cordl_register, create))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterEventHandler(
        &mut self,
        handler: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterEventHandler", (handler))?;
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
    pub fn get_PlayerConnectionGuid() -> quest_hook::libil2cpp::Result<
        crate::System::Guid,
    > {
        let __cordl_ret: crate::System::Guid = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_PlayerConnectionGuid", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ResourceManagement+Diagnostics+DiagnosticEventCollector")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEventCollector {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
