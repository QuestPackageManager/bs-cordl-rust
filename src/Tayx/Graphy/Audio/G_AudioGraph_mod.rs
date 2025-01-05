#[cfg(feature = "Tayx+Graphy+Audio+G_AudioGraph")]
#[repr(C)]
#[derive(Debug)]
pub struct G_AudioGraph {
    __cordl_parent: crate::Tayx::Graphy::Graph::G_Graph,
    pub m_imageGraph: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Image>,
    pub m_imageGraphHighestValues: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::Image,
    >,
    pub ShaderFull: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub ShaderLight: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub m_isInitialized: bool,
    pub m_graphyManager: quest_hook::libil2cpp::Gc<crate::Tayx::Graphy::GraphyManager>,
    pub m_audioMonitor: quest_hook::libil2cpp::Gc<
        crate::Tayx::Graphy::Audio::G_AudioMonitor,
    >,
    pub m_resolution: i32,
    pub m_shaderGraph: quest_hook::libil2cpp::Gc<crate::Tayx::Graphy::G_GraphShader>,
    pub m_shaderGraphHighestValues: quest_hook::libil2cpp::Gc<
        crate::Tayx::Graphy::G_GraphShader,
    >,
    pub m_graphArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
    pub m_graphArrayHighestValue: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<f32>,
    >,
}
#[cfg(feature = "Tayx+Graphy+Audio+G_AudioGraph")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::Audio::G_AudioGraph =>
    "Tayx.Graphy.Audio"."G_AudioGraph"
);
#[cfg(feature = "Tayx+Graphy+Audio+G_AudioGraph")]
impl std::ops::Deref for crate::Tayx::Graphy::Audio::G_AudioGraph {
    type Target = crate::Tayx::Graphy::Graph::G_Graph;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+Audio+G_AudioGraph")]
impl std::ops::DerefMut for crate::Tayx::Graphy::Audio::G_AudioGraph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+Audio+G_AudioGraph")]
impl crate::Tayx::Graphy::Audio::G_AudioGraph {
    pub fn CreatePoints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreatePoints", ())?;
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
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
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
    pub fn UpdateGraph(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateGraph", ())?;
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
#[cfg(feature = "Tayx+Graphy+Audio+G_AudioGraph")]
impl quest_hook::libil2cpp::ObjectType for crate::Tayx::Graphy::Audio::G_AudioGraph {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
