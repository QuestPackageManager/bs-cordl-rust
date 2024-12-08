#[cfg(feature = "UnityEngine+Rendering+VertexAttributeDescriptor")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct VertexAttributeDescriptor {
    pub _attribute_k__BackingField: crate::UnityEngine::Rendering::VertexAttribute,
    pub _format_k__BackingField: crate::UnityEngine::Rendering::VertexAttributeFormat,
    pub _dimension_k__BackingField: i32,
    pub _stream_k__BackingField: i32,
}
#[cfg(feature = "UnityEngine+Rendering+VertexAttributeDescriptor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Rendering::VertexAttributeDescriptor => "UnityEngine.Rendering"
    ."VertexAttributeDescriptor"
);
#[cfg(feature = "UnityEngine+Rendering+VertexAttributeDescriptor")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::VertexAttributeDescriptor {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+VertexAttributeDescriptor")]
impl crate::UnityEngine::Rendering::VertexAttributeDescriptor {
    pub fn Equals_Object0(
        &mut self,
        other: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_VertexAttributeDescriptor1(
        &mut self,
        other: crate::UnityEngine::Rendering::VertexAttributeDescriptor,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        attribute: crate::UnityEngine::Rendering::VertexAttribute,
        format: crate::UnityEngine::Rendering::VertexAttributeFormat,
        dimension: i32,
        stream: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (attribute, format, dimension, stream),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_attribute(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::VertexAttribute> {
        let __cordl_ret: crate::UnityEngine::Rendering::VertexAttribute = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_attribute",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_dimension(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_dimension",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_format(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::VertexAttributeFormat,
    > {
        let __cordl_ret: crate::UnityEngine::Rendering::VertexAttributeFormat = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_format",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_stream(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_stream",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_attribute(
        &mut self,
        value: crate::UnityEngine::Rendering::VertexAttribute,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_attribute",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_dimension(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_dimension",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_format(
        &mut self,
        value: crate::UnityEngine::Rendering::VertexAttributeFormat,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_format",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_stream(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_stream",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
