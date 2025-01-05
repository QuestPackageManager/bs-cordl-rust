#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData+Allocator")]
#[repr(C)]
#[derive(Debug)]
pub struct AllocMeshData_MeshBuilder_Allocator {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData+Allocator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::AllocMeshData_MeshBuilder_Allocator =>
    "UnityEngine.UIElements.UIR"."MeshBuilder/AllocMeshData/Allocator"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData+Allocator")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::UIR::AllocMeshData_MeshBuilder_Allocator {
    type Target = quest_hook::libil2cpp::Gc<crate::System::MulticastDelegate>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData+Allocator")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::UIR::AllocMeshData_MeshBuilder_Allocator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData+Allocator")]
impl crate::UnityEngine::UIElements::UIR::AllocMeshData_MeshBuilder_Allocator {
    pub fn Invoke(
        &mut self,
        vertexCount: u32,
        indexCount: u32,
        allocatorData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::MeshBuilder_AllocMeshData,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MeshWriteData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshWriteData,
        > = __cordl_object.invoke("Invoke", (vertexCount, indexCount, allocatorData))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData+Allocator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::AllocMeshData_MeshBuilder_Allocator {
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
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::MeshBuilder =>
    "UnityEngine.UIElements.UIR"."MeshBuilder"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::MeshBuilder {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
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
    pub fn ConvertTextVertexToUIRVertex(
        info: crate::UnityEngine::TextCore::Text::MeshInfo,
        index: i32,
        offset: crate::UnityEngine::Vector2,
        flags: crate::UnityEngine::UIElements::UIR::VertexFlags,
        isDynamicColor: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::Vertex> {
        let __cordl_ret: crate::UnityEngine::UIElements::Vertex = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ConvertTextVertexToUIRVertex",
                (info, index, offset, flags, isDynamicColor),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LimitTextVertices(
        vertexCount: i32,
        logTruncation: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LimitTextVertices", (vertexCount, logTruncation))?;
        Ok(__cordl_ret.into())
    }
    pub fn MakeText(
        meshInfo: crate::UnityEngine::TextCore::Text::MeshInfo,
        offset: crate::UnityEngine::Vector2,
        meshAlloc: crate::UnityEngine::UIElements::UIR::MeshBuilder_AllocMeshData,
        flags: crate::UnityEngine::UIElements::UIR::VertexFlags,
        isDynamicColor: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MakeText", (meshInfo, offset, meshAlloc, flags, isDynamicColor))?;
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct MeshBuilder_AllocMeshData {
    pub alloc: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::AllocMeshData_MeshBuilder_Allocator,
    >,
    pub texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture>,
    pub svgTexture: crate::UnityEngine::UIElements::TextureId,
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
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
    pub type Allocator = crate::UnityEngine::UIElements::UIR::AllocMeshData_MeshBuilder_Allocator;
    pub fn Allocate(
        &mut self,
        vertexCount: u32,
        indexCount: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::MeshWriteData>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshWriteData,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Allocate",
            (vertexCount, indexCount),
        )?;
        Ok(__cordl_ret.into())
    }
}
