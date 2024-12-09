#[cfg(feature = "UnityEngine+ProBuilder+SelectionPickerRenderer")]
#[repr(C)]
#[derive(Debug)]
pub struct SelectionPickerRenderer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectionPickerRenderer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::SelectionPickerRenderer
    => "UnityEngine.ProBuilder"."SelectionPickerRenderer"
);
#[cfg(feature = "UnityEngine+ProBuilder+SelectionPickerRenderer")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::SelectionPickerRenderer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectionPickerRenderer")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::SelectionPickerRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectionPickerRenderer")]
impl crate::UnityEngine::ProBuilder::SelectionPickerRenderer {
    pub const k_FacePickerOcclusionTintUniform: &'static str = "_Tint";
    pub const k_MinEdgePixelsForValidSelection: u32 = 16843009u32;
    pub const k_PickerHashMax: u32 = 4294967232u32;
    pub const k_PickerHashMin: u32 = 4294950913u32;
    pub const k_PickerHashNone: u32 = 4290773248u32;
    #[cfg(
        feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+ISelectionPickerRenderer"
    )]
    type ISelectionPickerRenderer = crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer;
    #[cfg(
        feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererHDRP"
    )]
    pub type SelectionPickerRendererHDRP = crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererHDRP;
    #[cfg(
        feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererStandard"
    )]
    pub type SelectionPickerRendererStandard = crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererStandard;
    #[cfg(feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+__c")]
    pub type __c = crate::UnityEngine::ProBuilder::SelectionPickerRenderer___c;
    #[cfg(
        feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+__c__DisplayClass19_0"
    )]
    pub type __c__DisplayClass19_0 = crate::UnityEngine::ProBuilder::SelectionPickerRenderer___c__DisplayClass19_0;
}
#[cfg(feature = "UnityEngine+ProBuilder+SelectionPickerRenderer")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+ISelectionPickerRenderer"
)]
#[repr(C)]
#[derive(Debug)]
pub struct SelectionPickerRenderer_ISelectionPickerRenderer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+ISelectionPickerRenderer"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer =>
    "UnityEngine.ProBuilder"."SelectionPickerRenderer/ISelectionPickerRenderer"
);
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+ISelectionPickerRenderer"
)]
impl std::ops::Deref
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+ISelectionPickerRenderer"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+ISelectionPickerRenderer"
)]
impl crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer {
    pub fn RenderLookupTexture(
        &mut self,
        camera: *mut crate::UnityEngine::Camera,
        shader: *mut crate::UnityEngine::Shader,
        tag: *mut quest_hook::libil2cpp::Il2CppString,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture2D> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture2D = __cordl_object
            .invoke("RenderLookupTexture", (camera, shader, tag, width, height))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+ISelectionPickerRenderer"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_ISelectionPickerRenderer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererHDRP"
)]
#[repr(C)]
#[derive(Debug)]
pub struct SelectionPickerRenderer_SelectionPickerRendererHDRP {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererHDRP"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererHDRP =>
    "UnityEngine.ProBuilder"."SelectionPickerRenderer/SelectionPickerRendererHDRP"
);
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererHDRP"
)]
impl std::ops::Deref
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererHDRP {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererHDRP"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererHDRP {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererHDRP"
)]
impl crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererHDRP {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RenderLookupTexture(
        &mut self,
        camera: *mut crate::UnityEngine::Camera,
        shader: *mut crate::UnityEngine::Shader,
        tag: *mut quest_hook::libil2cpp::Il2CppString,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture2D> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture2D = __cordl_object
            .invoke("RenderLookupTexture", (camera, shader, tag, width, height))?;
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
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererHDRP"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererHDRP {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererStandard"
)]
#[repr(C)]
#[derive(Debug)]
pub struct SelectionPickerRenderer_SelectionPickerRendererStandard {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererStandard"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererStandard =>
    "UnityEngine.ProBuilder"."SelectionPickerRenderer/SelectionPickerRendererStandard"
);
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererStandard"
)]
impl std::ops::Deref
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererStandard {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererStandard"
)]
impl std::ops::DerefMut
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererStandard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererStandard"
)]
impl crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererStandard {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RenderLookupTexture(
        &mut self,
        camera: *mut crate::UnityEngine::Camera,
        shader: *mut crate::UnityEngine::Shader,
        tag: *mut quest_hook::libil2cpp::Il2CppString,
        width: i32,
        height: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture2D> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture2D = __cordl_object
            .invoke("RenderLookupTexture", (camera, shader, tag, width, height))?;
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
#[cfg(
    feature = "UnityEngine+ProBuilder+SelectionPickerRenderer+SelectionPickerRendererStandard"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::SelectionPickerRenderer_SelectionPickerRendererStandard {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
