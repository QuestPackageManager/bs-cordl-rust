#[cfg(
    feature = "UnityEngine+ResourceManagement+Diagnostics+DiagnosticEventCollectorSingleton"
)]
#[repr(C)]
#[derive(Debug)]
pub struct DiagnosticEventCollectorSingleton {
    __cordl_parent: crate::UnityEngine::ResourceManagement::Util::ComponentSingleton_1<
        *mut crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEventCollectorSingleton,
    >,
    pub m_CreatedEvents: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent,
    >,
    pub m_UnhandledEvents: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent,
    >,
    pub s_EventHandlers: *mut crate::GlobalNamespace::DelegateList_1<
        crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent,
    >,
    pub m_lastTickSent: f32,
    pub m_lastFrame: i32,
    pub fpsAvg: f32,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Diagnostics+DiagnosticEventCollectorSingleton"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEventCollectorSingleton =>
    "UnityEngine.ResourceManagement.Diagnostics"."DiagnosticEventCollectorSingleton"
);
#[cfg(
    feature = "UnityEngine+ResourceManagement+Diagnostics+DiagnosticEventCollectorSingleton"
)]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEventCollectorSingleton {
    type Target = crate::UnityEngine::ResourceManagement::Util::ComponentSingleton_1<
        *mut crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEventCollectorSingleton,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Diagnostics+DiagnosticEventCollectorSingleton"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEventCollectorSingleton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Diagnostics+DiagnosticEventCollectorSingleton"
)]
impl crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEventCollectorSingleton {
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGameObjectName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetGameObjectName", ())?;
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
    pub fn RegisterEventHandler_Action_1_1(
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
            .invoke("RegisterEventHandler", (handler))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterEventHandler__cordl_bool__cordl_bool0(
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
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
#[cfg(
    feature = "UnityEngine+ResourceManagement+Diagnostics+DiagnosticEventCollectorSingleton"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEventCollectorSingleton {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
