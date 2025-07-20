#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData+Allocator")]
#[repr(C)]
#[derive(Debug)]
pub struct AllocMeshData_MeshBuilder_Allocator {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData+Allocator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::AllocMeshData_MeshBuilder_Allocator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "MeshBuilder/AllocMeshData/Allocator";
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
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData+Allocator")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::UIR::AllocMeshData_MeshBuilder_Allocator {
    type Target = crate::System::MulticastDelegate;
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            u32,
                            u32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::UIElements::UIR::MeshBuilder_AllocMeshData,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::MeshWriteData,
                        >,
                        3usize,
                    >("Invoke")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Invoke", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshWriteData,
        > = unsafe {
            method.invoke_unchecked(self, (vertexCount, indexCount, allocatorData))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppObject,
                            >,
                            crate::System::IntPtr,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (object, method))?
        };
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::MeshBuilder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "MeshBuilder";
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
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::MeshBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::TextCore::Text::MeshInfo,
                            i32,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::UIElements::UIR::VertexFlags,
                            bool,
                        ),
                        crate::UnityEngine::UIElements::Vertex,
                        5usize,
                    >("ConvertTextVertexToUIRVertex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "ConvertTextVertexToUIRVertex", 5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::UIElements::Vertex = unsafe {
            method.invoke_unchecked((), (info, index, offset, flags, isDynamicColor))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LimitTextVertices(
        vertexCount: i32,
        logTruncation: bool,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<(i32, bool), i32, 2usize>("LimitTextVertices")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "LimitTextVertices", 2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (vertexCount, logTruncation))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MakeText(
        meshInfo: crate::UnityEngine::TextCore::Text::MeshInfo,
        offset: crate::UnityEngine::Vector2,
        meshAlloc: crate::UnityEngine::UIElements::UIR::MeshBuilder_AllocMeshData,
        flags: crate::UnityEngine::UIElements::UIR::VertexFlags,
        isDynamicColor: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::TextCore::Text::MeshInfo,
                            crate::UnityEngine::Vector2,
                            crate::UnityEngine::UIElements::UIR::MeshBuilder_AllocMeshData,
                            crate::UnityEngine::UIElements::UIR::VertexFlags,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("MakeText")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "MakeText", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (meshInfo, offset, meshAlloc, flags, isDynamicColor),
                )?
        };
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
#[derive(Debug, Clone, Default, PartialEq)]
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
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::MeshBuilder_AllocMeshData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "MeshBuilder/AllocMeshData";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::UIElements::UIR::MeshBuilder_AllocMeshData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::UIElements::UIR::MeshBuilder_AllocMeshData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::UIElements::UIR::MeshBuilder_AllocMeshData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+MeshBuilder+AllocMeshData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::UIElements::UIR::MeshBuilder_AllocMeshData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (u32, u32),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::UIElements::MeshWriteData,
                        >,
                        2usize,
                    >("Allocate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Allocate", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::MeshWriteData,
        > = unsafe { method.invoke_unchecked(self, (vertexCount, indexCount))? };
        Ok(__cordl_ret.into())
    }
}
