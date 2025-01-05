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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::Fps::G_FpsManager =>
    "Tayx.Graphy.Fps"."G_FpsManager"
);
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshParameters", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RestorePreviousState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RestorePreviousState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetGraphActive(
        &mut self,
        active: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetGraphActive", (active))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPosition(
        &mut self,
        newModulePosition: crate::Tayx::Graphy::GraphyManager_ModulePosition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetPosition", (newModulePosition))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetState(
        &mut self,
        state: crate::Tayx::Graphy::GraphyManager_ModuleState,
        silentUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetState", (state, silentUpdate))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateParameters", ())?;
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
