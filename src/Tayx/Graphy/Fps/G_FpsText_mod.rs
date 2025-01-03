#[cfg(feature = "Tayx+Graphy+Fps+G_FpsText")]
#[repr(C)]
#[derive(Debug)]
pub struct G_FpsText {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub m_fpsText: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
    pub m_msText: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
    pub m_avgFpsText: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
    pub m_onePercentFpsText: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
    pub m_zero1PercentFpsText: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
    pub m_graphyManager: quest_hook::libil2cpp::Gc<crate::Tayx::Graphy::GraphyManager>,
    pub m_fpsMonitor: quest_hook::libil2cpp::Gc<crate::Tayx::Graphy::Fps::G_FpsMonitor>,
    pub m_updateRate: i32,
    pub m_frameCount: i32,
    pub m_deltaTime: f32,
    pub m_fps: f32,
    pub m_ms: f32,
}
#[cfg(feature = "Tayx+Graphy+Fps+G_FpsText")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::Fps::G_FpsText =>
    "Tayx.Graphy.Fps"."G_FpsText"
);
#[cfg(feature = "Tayx+Graphy+Fps+G_FpsText")]
impl std::ops::Deref for crate::Tayx::Graphy::Fps::G_FpsText {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+Fps+G_FpsText")]
impl std::ops::DerefMut for crate::Tayx::Graphy::Fps::G_FpsText {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+Fps+G_FpsText")]
impl crate::Tayx::Graphy::Fps::G_FpsText {
    pub const m_msStringFormat: &'static str = "0.0";
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
    pub fn SetFpsRelatedTextColor(
        &mut self,
        text: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
        fps: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetFpsRelatedTextColor", (text, fps))?;
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
#[cfg(feature = "Tayx+Graphy+Fps+G_FpsText")]
impl quest_hook::libil2cpp::ObjectType for crate::Tayx::Graphy::Fps::G_FpsText {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
