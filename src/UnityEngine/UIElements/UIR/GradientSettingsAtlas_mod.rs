#[cfg(feature = "UnityEngine+UIElements+UIR+GradientSettingsAtlas")]
#[repr(C)]
#[derive(Debug)]
pub struct GradientSettingsAtlas {
    __cordl_parent: crate::System::Object,
    pub m_Length: i32,
    pub m_ElemWidth: i32,
    pub m_Allocator: *mut crate::UnityEngine::UIElements::UIR::BestFitAllocator,
    pub m_Atlas: *mut crate::UnityEngine::Texture2D,
    pub m_RawAtlas: crate::UnityEngine::UIElements::UIR::GradientSettingsAtlas_RawTexture,
    pub _disposed_k__BackingField: bool,
    pub _MustCommit_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+GradientSettingsAtlas")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::GradientSettingsAtlas => "UnityEngine.UIElements.UIR"
    ."GradientSettingsAtlas"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+GradientSettingsAtlas")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::GradientSettingsAtlas {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+GradientSettingsAtlas")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::GradientSettingsAtlas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+GradientSettingsAtlas")]
impl crate::UnityEngine::UIElements::UIR::GradientSettingsAtlas {
    #[cfg(feature = "UnityEngine+UIElements+UIR+GradientSettingsAtlas+RawTexture")]
    pub type RawTexture = crate::UnityEngine::UIElements::UIR::GradientSettingsAtlas_RawTexture;
    pub fn PrepareAtlas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PrepareAtlas", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret)
    }
    pub fn get_length(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_length", ())?;
        Ok(__cordl_ret)
    }
    pub fn Commit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Commit", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MustCommit(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_MustCommit", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_disposed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disposed", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_MustCommit(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MustCommit", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn Add(
        &mut self,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::Alloc> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::Alloc = __cordl_object
            .invoke("Add", (count))?;
        Ok(__cordl_ret)
    }
    pub fn get_atlas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture2D> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture2D = __cordl_object
            .invoke("get_atlas", ())?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::Alloc,
        settings: *mut quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::UIElements::GradientSettings,
        >,
        remap: *mut crate::UnityEngine::UIElements::UIR::GradientRemap,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (alloc, settings, remap))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (length))?;
        Ok(__cordl_ret)
    }
    pub fn get_disposed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_disposed", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(length: i32) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (length))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+GradientSettingsAtlas")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::GradientSettingsAtlas {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+GradientSettingsAtlas+RawTexture")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GradientSettingsAtlas_RawTexture {
    pub rgba: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
    pub width: i32,
    pub height: i32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+GradientSettingsAtlas+RawTexture")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::GradientSettingsAtlas_RawTexture =>
    "UnityEngine.UIElements.UIR"."GradientSettingsAtlas/RawTexture"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+GradientSettingsAtlas+RawTexture")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::UIElements::UIR::GradientSettingsAtlas_RawTexture {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+GradientSettingsAtlas+RawTexture")]
impl crate::UnityEngine::UIElements::UIR::GradientSettingsAtlas_RawTexture {
    pub fn WriteRawInt2Packed(
        &mut self,
        v0: i32,
        v1: i32,
        destX: i32,
        destY: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WriteRawInt2Packed",
            (v0, v1, destX, destY),
        )?;
        Ok(__cordl_ret)
    }
    pub fn WriteRawFloat4Packed(
        &mut self,
        f0: f32,
        f1: f32,
        f2: f32,
        f3: f32,
        destX: i32,
        destY: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WriteRawFloat4Packed",
            (f0, f1, f2, f3, destX, destY),
        )?;
        Ok(__cordl_ret)
    }
}
