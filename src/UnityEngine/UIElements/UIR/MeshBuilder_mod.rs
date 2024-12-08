#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MeshBuilder_AllocMeshData {
    pub alloc: *mut crate::UnityEngine::UIElements::UIR::AllocMeshData_Allocator,
    pub texture: *mut crate::UnityEngine::Texture,
    pub svgTexture: crate::UnityEngine::UIElements::TextureId,
    pub material: *mut crate::UnityEngine::Material,
    pub flags: crate::UnityEngine::UIElements::MeshGenerationContext_MeshFlags,
    pub colorAlloc: crate::UnityEngine::UIElements::UIR::BMPAlloc,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::MeshBuilder_AllocMeshData =>
    "UnityEngine.UIElements.UIR"."MeshBuilder/AllocMeshData"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::MeshBuilder_AllocMeshData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData")]
impl crate::UnityEngine::UIElements::UIR::MeshBuilder_AllocMeshData {
    #[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData+Allocator")]
    pub type Allocator = crate::UnityEngine::UIElements::UIR::AllocMeshData_Allocator;
    pub fn Allocate(
        &mut self,
        vertexCount: u32,
        indexCount: u32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::MeshWriteData,
    > {
        let __cordl_ret: *mut crate::UnityEngine::UIElements::MeshWriteData = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Allocate",
            (vertexCount, indexCount),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData+Allocator")]
#[repr(C)]
#[derive(Debug)]
pub struct AllocMeshData_Allocator {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData+Allocator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::AllocMeshData_Allocator =>
    "UnityEngine.UIElements.UIR"."MeshBuilder/AllocMeshData/Allocator"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData+Allocator")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::AllocMeshData_Allocator {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData+Allocator")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::UIR::AllocMeshData_Allocator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData+Allocator")]
impl crate::UnityEngine::UIElements::UIR::AllocMeshData_Allocator {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        vertexCount: u32,
        indexCount: u32,
        allocatorData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::MeshBuilder_AllocMeshData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::MeshWriteData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::MeshWriteData = __cordl_object
            .invoke("Invoke", (vertexCount, indexCount, allocatorData))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData+Allocator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::AllocMeshData_Allocator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct MeshBuilder {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::MeshBuilder =>
    "UnityEngine.UIElements.UIR"."MeshBuilder"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::MeshBuilder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::MeshBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder")]
impl crate::UnityEngine::UIElements::UIR::MeshBuilder {
    #[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData")]
    pub type AllocMeshData = crate::UnityEngine::UIElements::UIR::MeshBuilder_AllocMeshData;
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::MeshBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
