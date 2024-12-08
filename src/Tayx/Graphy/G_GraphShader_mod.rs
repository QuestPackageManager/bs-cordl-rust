#[cfg(feature = "Tayx+Graphy+G_GraphShader")]
#[repr(C)]
#[derive(Debug)]
pub struct G_GraphShader {
    __cordl_parent: crate::System::Object,
    pub ArrayMaxSize: i32,
    pub ShaderArrayValues: *mut quest_hook::libil2cpp::Il2CppArray<f32>,
    pub Image: *mut crate::UnityEngine::UI::Image,
    pub Name: *mut crate::System::String,
    pub Name_Length: *mut crate::System::String,
    pub Average: f32,
    pub m_averagePropertyId: i32,
    pub GoodThreshold: f32,
    pub CautionThreshold: f32,
    pub m_goodThresholdPropertyId: i32,
    pub m_cautionThresholdPropertyId: i32,
    pub GoodColor: crate::UnityEngine::Color,
    pub CautionColor: crate::UnityEngine::Color,
    pub CriticalColor: crate::UnityEngine::Color,
    pub m_goodColorPropertyId: i32,
    pub m_cautionColorPropertyId: i32,
    pub m_criticalColorPropertyId: i32,
}
#[cfg(feature = "Tayx+Graphy+G_GraphShader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Tayx::Graphy::G_GraphShader => "Tayx.Graphy"
    ."G_GraphShader"
);
#[cfg(feature = "Tayx+Graphy+G_GraphShader")]
impl std::ops::Deref for crate::Tayx::Graphy::G_GraphShader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+G_GraphShader")]
impl std::ops::DerefMut for crate::Tayx::Graphy::G_GraphShader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Tayx+Graphy+G_GraphShader")]
impl crate::Tayx::Graphy::G_GraphShader {
    pub const ArrayMaxSizeFull: i32 = 512i32;
    pub const ArrayMaxSizeLight: i32 = 128i32;
    pub fn InitializeShader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitializeShader", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn UpdateArray(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateArray", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateAverage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAverage", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateColors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateColors", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdatePoints(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdatePoints", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateThresholds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateThresholds", ())?;
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
}
#[cfg(feature = "Tayx+Graphy+G_GraphShader")]
impl quest_hook::libil2cpp::ObjectType for crate::Tayx::Graphy::G_GraphShader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
