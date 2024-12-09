#[cfg(feature = "UnityEngine+Rendering+SubMeshDescriptor")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SubMeshDescriptor {
    pub _bounds_k__BackingField: crate::UnityEngine::Bounds,
    pub _topology_k__BackingField: crate::UnityEngine::MeshTopology,
    pub _indexStart_k__BackingField: i32,
    pub _indexCount_k__BackingField: i32,
    pub _baseVertex_k__BackingField: i32,
    pub _firstVertex_k__BackingField: i32,
    pub _vertexCount_k__BackingField: i32,
}
#[cfg(feature = "UnityEngine+Rendering+SubMeshDescriptor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rendering::SubMeshDescriptor =>
    "UnityEngine.Rendering"."SubMeshDescriptor"
);
#[cfg(feature = "UnityEngine+Rendering+SubMeshDescriptor")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::SubMeshDescriptor {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+SubMeshDescriptor")]
impl crate::UnityEngine::Rendering::SubMeshDescriptor {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        indexStart: i32,
        indexCount: i32,
        topology: crate::UnityEngine::MeshTopology,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (indexStart, indexCount, topology),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_baseVertex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_baseVertex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_bounds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Bounds> {
        let __cordl_ret: crate::UnityEngine::Bounds = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_bounds",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_firstVertex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_firstVertex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_indexCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_indexCount",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_indexStart(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_indexStart",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_topology(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::MeshTopology> {
        let __cordl_ret: crate::UnityEngine::MeshTopology = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_topology",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_vertexCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_vertexCount",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_baseVertex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_baseVertex",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_bounds(
        &mut self,
        value: crate::UnityEngine::Bounds,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_bounds",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_firstVertex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_firstVertex",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_indexCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_indexCount",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_indexStart(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_indexStart",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_topology(
        &mut self,
        value: crate::UnityEngine::MeshTopology,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_topology",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_vertexCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_vertexCount",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
