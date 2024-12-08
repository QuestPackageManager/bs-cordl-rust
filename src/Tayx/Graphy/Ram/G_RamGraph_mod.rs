#[cfg(feature = "Tayx+Graphy+Ram+G_RamGraph")]
#[repr(C)]
#[derive(Debug)]
pub struct G_RamGraph {
    __cordl_parent: crate::Tayx::Graphy::Graph::G_Graph,
    pub m_imageAllocated: *mut crate::UnityEngine::UI::Image,
    pub m_imageReserved: *mut crate::UnityEngine::UI::Image,
    pub m_imageMono: *mut crate::UnityEngine::UI::Image,
    pub ShaderFull: *mut crate::UnityEngine::Shader,
    pub ShaderLight: *mut crate::UnityEngine::Shader,
    pub m_isInitialized: bool,
    pub m_graphyManager: *mut crate::Tayx::Graphy::GraphyManager,
    pub m_ramMonitor: *mut crate::Tayx::Graphy::Ram::G_RamMonitor,
    pub m_resolution: i32,
    pub m_shaderGraphAllocated: *mut crate::Tayx::Graphy::G_GraphShader,
    pub m_shaderGraphReserved: *mut crate::Tayx::Graphy::G_GraphShader,
    pub m_shaderGraphMono: *mut crate::Tayx::Graphy::G_GraphShader,
    pub m_allocatedArray: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub m_reservedArray: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub m_monoArray: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub m_highestMemory: f32,
}
#[cfg(feature = "Tayx+Graphy+Ram+G_RamGraph")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::Ram::G_RamGraph =>
    "Tayx.Graphy.Ram"."G_RamGraph"
);
#[cfg(feature = "Tayx+Graphy+Ram+G_RamGraph")]
impl std::ops::Deref for crate::Tayx::Graphy::Ram::G_RamGraph {
    type Target = crate::Tayx::Graphy::Graph::G_Graph;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+Ram+G_RamGraph")]
impl std::ops::DerefMut for crate::Tayx::Graphy::Ram::G_RamGraph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+Ram+G_RamGraph")]
impl crate::Tayx::Graphy::Ram::G_RamGraph {
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateGraph(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateGraph", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreatePoints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreatePoints", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateParameters(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateParameters", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Tayx+Graphy+Ram+G_RamGraph")]
impl quest_hook::libil2cpp::ObjectType for crate::Tayx::Graphy::Ram::G_RamGraph {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
