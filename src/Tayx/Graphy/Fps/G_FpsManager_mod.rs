#[cfg(feature = "Tayx+Graphy+Fps+G_FpsManager")]
#[repr(C)]
#[derive(Debug)]
pub struct G_FpsManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub m_fpsGraphGameObject: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    pub m_nonBasicTextGameObjects: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    >,
    pub m_backgroundImages: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
        >,
    >,
    pub m_graphyManager: quest_hook::libil2cpp::Gc<crate::Tayx::Graphy::GraphyManager>,
    pub m_fpsGraph: quest_hook::libil2cpp::Gc<crate::Tayx::Graphy::Fps::G_FpsGraph>,
    pub m_fpsMonitor: quest_hook::libil2cpp::Gc<crate::Tayx::Graphy::Fps::G_FpsMonitor>,
    pub m_fpsText: quest_hook::libil2cpp::Gc<crate::Tayx::Graphy::Fps::G_FpsText>,
    pub m_rectTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::RectTransform>,
    pub m_childrenGameObjects: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        >,
    >,
    pub m_previousModuleState: crate::Tayx::Graphy::GraphyManager_ModuleState,
    pub m_currentModuleState: crate::Tayx::Graphy::GraphyManager_ModuleState,
}
#[cfg(feature = "Tayx+Graphy+Fps+G_FpsManager")]
unsafe impl quest_hook::libil2cpp::Type for crate::Tayx::Graphy::Fps::G_FpsManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Tayx.Graphy.Fps";
    const CLASS_NAME: &'static str = "G_FpsManager";
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
#[cfg(feature = "Tayx+Graphy+Fps+G_FpsManager")]
impl std::ops::Deref for crate::Tayx::Graphy::Fps::G_FpsManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+Fps+G_FpsManager")]
impl std::ops::DerefMut for crate::Tayx::Graphy::Fps::G_FpsManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+Fps+G_FpsManager")]
impl crate::Tayx::Graphy::Fps::G_FpsManager {
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
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Init", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RefreshParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("RefreshParameters")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RefreshParameters", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn RestorePreviousState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("RestorePreviousState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RestorePreviousState", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetGraphActive(
        &mut self,
        active: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>("SetGraphActive")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetGraphActive", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (active))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetPosition(
        &mut self,
        newModulePosition: crate::Tayx::Graphy::GraphyManager_ModulePosition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Tayx::Graphy::GraphyManager_ModulePosition),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetPosition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetPosition", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (newModulePosition))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetState(
        &mut self,
        state: crate::Tayx::Graphy::GraphyManager_ModuleState,
        silentUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Tayx::Graphy::GraphyManager_ModuleState, bool),
                quest_hook::libil2cpp::Void,
                2usize,
            >("SetState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetState", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (state, silentUpdate))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Start")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Start", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("UpdateParameters")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdateParameters", 0usize
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
}
#[cfg(feature = "Tayx+Graphy+Fps+G_FpsManager")]
impl quest_hook::libil2cpp::ObjectType for crate::Tayx::Graphy::Fps::G_FpsManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Tayx+Graphy+Fps+G_FpsManager")]
impl AsRef<crate::Tayx::Graphy::UI::IModifiableState>
for crate::Tayx::Graphy::Fps::G_FpsManager {
    fn as_ref(&self) -> &crate::Tayx::Graphy::UI::IModifiableState {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Tayx+Graphy+Fps+G_FpsManager")]
impl AsMut<crate::Tayx::Graphy::UI::IModifiableState>
for crate::Tayx::Graphy::Fps::G_FpsManager {
    fn as_mut(&mut self) -> &mut crate::Tayx::Graphy::UI::IModifiableState {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Tayx+Graphy+Fps+G_FpsManager")]
impl AsRef<crate::Tayx::Graphy::UI::IMovable>
for crate::Tayx::Graphy::Fps::G_FpsManager {
    fn as_ref(&self) -> &crate::Tayx::Graphy::UI::IMovable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Tayx+Graphy+Fps+G_FpsManager")]
impl AsMut<crate::Tayx::Graphy::UI::IMovable>
for crate::Tayx::Graphy::Fps::G_FpsManager {
    fn as_mut(&mut self) -> &mut crate::Tayx::Graphy::UI::IMovable {
        unsafe { std::mem::transmute(self) }
    }
}
