#[cfg(
    feature = "UnityEngine+ResourceManagement+Diagnostics+DiagnosticEventCollectorSingleton"
)]
#[repr(C)]
#[derive(Debug)]
pub struct DiagnosticEventCollectorSingleton {
    __cordl_parent: crate::UnityEngine::ResourceManagement::Util::ComponentSingleton_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEventCollectorSingleton,
        >,
    >,
    pub m_CreatedEvents: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            i32,
            crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent,
        >,
    >,
    pub m_UnhandledEvents: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent,
        >,
    >,
    pub s_EventHandlers: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::DelegateList_1<
            crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent,
        >,
    >,
    pub m_lastTickSent: f32,
    pub m_lastFrame: i32,
    pub fpsAvg: f32,
}
#[cfg(
    feature = "UnityEngine+ResourceManagement+Diagnostics+DiagnosticEventCollectorSingleton"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEventCollectorSingleton {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ResourceManagement.Diagnostics";
    const CLASS_NAME: &'static str = "DiagnosticEventCollectorSingleton";
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
#[cfg(
    feature = "UnityEngine+ResourceManagement+Diagnostics+DiagnosticEventCollectorSingleton"
)]
impl std::ops::Deref
for crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEventCollectorSingleton {
    type Target = crate::UnityEngine::ResourceManagement::Util::ComponentSingleton_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEventCollectorSingleton,
        >,
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Awake")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Awake", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetGameObjectName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("GetGameObjectName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetGameObjectName", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ()) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent),
                quest_hook::libil2cpp::Void,
                1usize,
            >("PostEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PostEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (diagnosticEvent))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("RegisterEventHandler")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterEventHandler", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (handler))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent,
                        >,
                    >,
                    bool,
                    bool,
                ),
                bool,
                3usize,
            >("RegisterEventHandler")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RegisterEventHandler", 3usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (handler, _cordl_register, create))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        crate::UnityEngine::ResourceManagement::Diagnostics::DiagnosticEvent,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UnregisterEventHandler")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UnregisterEventHandler", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (handler))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Update")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Update", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_PlayerConnectionGuid() -> quest_hook::libil2cpp::Result<
        crate::System::Guid,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::System::Guid,
                0usize,
            >("get_PlayerConnectionGuid")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_PlayerConnectionGuid", 0usize
                )
            });
        let __cordl_ret: crate::System::Guid = unsafe {
            method.invoke_unchecked((), ())
        };
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
