#[cfg(feature = "OVRPassthroughLayer+BCSStyleHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPassthroughLayer_BCSStyleHandler {
    __cordl_parent: crate::GlobalNamespace::OVRPassthroughLayer_BaseGeneratedStyleHandler,
}
#[cfg(feature = "OVRPassthroughLayer+BCSStyleHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPassthroughLayer_BCSStyleHandler => ""
    ."OVRPassthroughLayer/BCSStyleHandler"
);
#[cfg(feature = "OVRPassthroughLayer+BCSStyleHandler")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPassthroughLayer_BCSStyleHandler {
    type Target = crate::GlobalNamespace::OVRPassthroughLayer_BaseGeneratedStyleHandler;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughLayer+BCSStyleHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPassthroughLayer_BCSStyleHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughLayer+BCSStyleHandler")]
impl crate::GlobalNamespace::OVRPassthroughLayer_BCSStyleHandler {
    pub fn Update(
        &mut self,
        settings: crate::GlobalNamespace::OVRPassthroughLayer_Settings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (settings))?;
        Ok(__cordl_ret)
    }
    pub fn get_MapSize(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_MapSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        colorMapDataHandler: quest_hook::libil2cpp::ByRefMut<
            crate::System::Runtime::InteropServices::GCHandle,
        >,
        colorMapData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (colorMapDataHandler, colorMapData))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        colorMapDataHandler: quest_hook::libil2cpp::ByRefMut<
            crate::System::Runtime::InteropServices::GCHandle,
        >,
        colorMapData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (colorMapDataHandler, colorMapData))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVRPassthroughLayer+BCSStyleHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPassthroughLayer_BCSStyleHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPassthroughLayer+BaseGeneratedStyleHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPassthroughLayer_BaseGeneratedStyleHandler {
    __cordl_parent: crate::System::Object,
    pub _colorMapDataHandle: crate::System::Runtime::InteropServices::GCHandle,
    pub _colorMapData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "OVRPassthroughLayer+BaseGeneratedStyleHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPassthroughLayer_BaseGeneratedStyleHandler => ""
    ."OVRPassthroughLayer/BaseGeneratedStyleHandler"
);
#[cfg(feature = "OVRPassthroughLayer+BaseGeneratedStyleHandler")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRPassthroughLayer_BaseGeneratedStyleHandler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughLayer+BaseGeneratedStyleHandler")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRPassthroughLayer_BaseGeneratedStyleHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughLayer+BaseGeneratedStyleHandler")]
impl crate::GlobalNamespace::OVRPassthroughLayer_BaseGeneratedStyleHandler {
    pub fn _ctor(
        &mut self,
        colorMapDataHandler: quest_hook::libil2cpp::ByRefMut<
            crate::System::Runtime::InteropServices::GCHandle,
        >,
        colorMapData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (colorMapDataHandler, colorMapData))?;
        Ok(__cordl_ret)
    }
    pub fn ApplyStyleSettings(
        &mut self,
        style: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyle2,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyStyleSettings", (style))?;
        Ok(__cordl_ret)
    }
    pub fn DeallocateColorMapData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeallocateColorMapData", ())?;
        Ok(__cordl_ret)
    }
    pub fn AllocateColorMapData(
        &mut self,
        _cordl_size: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AllocateColorMapData", (_cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn WriteFloatToColorMap(
        &mut self,
        index: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteFloatToColorMap", (index, value))?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn WriteColorToColorMap(
        &mut self,
        colorIndex: i32,
        color: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteColorToColorMap", (colorIndex, color))?;
        Ok(__cordl_ret)
    }
    pub fn get_MapSize(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_MapSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsValid", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
        settings: crate::GlobalNamespace::OVRPassthroughLayer_Settings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (settings))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        colorMapDataHandler: quest_hook::libil2cpp::ByRefMut<
            crate::System::Runtime::InteropServices::GCHandle,
        >,
        colorMapData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (colorMapDataHandler, colorMapData))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVRPassthroughLayer+BaseGeneratedStyleHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPassthroughLayer_BaseGeneratedStyleHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPassthroughLayer+ColorLutHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPassthroughLayer_ColorLutHandler {
    __cordl_parent: crate::System::Object,
    pub _currentFlipLutY: bool,
    pub _currentColorLutSourceTexture: *mut crate::UnityEngine::Texture2D,
    pub _Lut_k__BackingField: *mut OVRPassthroughColorLut,
    pub _Weight_k__BackingField: f32,
    pub _IsValid_k__BackingField: bool,
}
#[cfg(feature = "OVRPassthroughLayer+ColorLutHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPassthroughLayer_ColorLutHandler => ""
    ."OVRPassthroughLayer/ColorLutHandler"
);
#[cfg(feature = "OVRPassthroughLayer+ColorLutHandler")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPassthroughLayer_ColorLutHandler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughLayer+ColorLutHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPassthroughLayer_ColorLutHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughLayer+ColorLutHandler")]
impl crate::GlobalNamespace::OVRPassthroughLayer_ColorLutHandler {
    pub fn set_IsValid(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsValid", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Update_OVRPassthroughLayer_Settings0(
        &mut self,
        settings: crate::GlobalNamespace::OVRPassthroughLayer_Settings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (settings))?;
        Ok(__cordl_ret)
    }
    pub fn Update_OVRPassthroughColorLut_f32_1(
        &mut self,
        lut: *mut OVRPassthroughColorLut,
        weight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (lut, weight))?;
        Ok(__cordl_ret)
    }
    pub fn GetColorLutForTexture(
        &mut self,
        newTexture: *mut crate::UnityEngine::Texture2D,
        lut: *mut OVRPassthroughColorLut,
        lastTexture: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::Texture2D>,
        flipY: bool,
    ) -> quest_hook::libil2cpp::Result<*mut OVRPassthroughColorLut> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut OVRPassthroughColorLut = __cordl_object
            .invoke("GetColorLutForTexture", (newTexture, lut, lastTexture, flipY))?;
        Ok(__cordl_ret)
    }
    pub fn get_Weight(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_Weight", ())?;
        Ok(__cordl_ret)
    }
    pub fn ApplyStyleSettings(
        &mut self,
        style: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyle2,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyStyleSettings", (style))?;
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
    pub fn get_IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsValid", ())?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Lut(
        &mut self,
        value: *mut OVRPassthroughColorLut,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Lut", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_Lut(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut OVRPassthroughColorLut> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut OVRPassthroughColorLut = __cordl_object
            .invoke("get_Lut", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Weight(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Weight", (value))?;
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
#[cfg(feature = "OVRPassthroughLayer+ColorLutHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPassthroughLayer_ColorLutHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPassthroughLayer+ColorMapEditorType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPassthroughLayer_ColorMapEditorType {
    ColorAdjustment = 4i32,
    ColorLut = 5i32,
    Controls = 1i32,
    Custom = 2i32,
    Grayscale = 3i32,
    InterpolatedColorLut = 6i32,
    None = 0i32,
}
#[cfg(feature = "OVRPassthroughLayer+ColorMapEditorType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPassthroughLayer_ColorMapEditorType => ""
    ."OVRPassthroughLayer/ColorMapEditorType"
);
#[cfg(feature = "OVRPassthroughLayer+DeferredPassthroughMeshAddition")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPassthroughLayer_DeferredPassthroughMeshAddition {
    pub gameObject: *mut crate::UnityEngine::GameObject,
    pub updateTransform: bool,
}
#[cfg(feature = "OVRPassthroughLayer+DeferredPassthroughMeshAddition")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPassthroughLayer_DeferredPassthroughMeshAddition => ""
    ."OVRPassthroughLayer/DeferredPassthroughMeshAddition"
);
#[cfg(feature = "OVRPassthroughLayer+DeferredPassthroughMeshAddition")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPassthroughLayer_DeferredPassthroughMeshAddition {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPassthroughLayer+DeferredPassthroughMeshAddition")]
impl crate::GlobalNamespace::OVRPassthroughLayer_DeferredPassthroughMeshAddition {}
#[cfg(feature = "OVRPassthroughLayer+IStyleHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPassthroughLayer_IStyleHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPassthroughLayer+IStyleHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPassthroughLayer_IStyleHandler => ""
    ."OVRPassthroughLayer/IStyleHandler"
);
#[cfg(feature = "OVRPassthroughLayer+IStyleHandler")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPassthroughLayer_IStyleHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughLayer+IStyleHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPassthroughLayer_IStyleHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughLayer+IStyleHandler")]
impl crate::GlobalNamespace::OVRPassthroughLayer_IStyleHandler {
    pub fn get_IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsValid", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
        settings: crate::GlobalNamespace::OVRPassthroughLayer_Settings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (settings))?;
        Ok(__cordl_ret)
    }
    pub fn ApplyStyleSettings(
        &mut self,
        style: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyle2,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyStyleSettings", (style))?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "OVRPassthroughLayer+IStyleHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPassthroughLayer_IStyleHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPassthroughLayer+InterpolatedColorLutHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPassthroughLayer_InterpolatedColorLutHandler {
    __cordl_parent: crate::GlobalNamespace::OVRPassthroughLayer_ColorLutHandler,
    pub _currentColorLutTargetTexture: *mut crate::UnityEngine::Texture2D,
    pub _LutTarget_k__BackingField: *mut OVRPassthroughColorLut,
}
#[cfg(feature = "OVRPassthroughLayer+InterpolatedColorLutHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPassthroughLayer_InterpolatedColorLutHandler => ""
    ."OVRPassthroughLayer/InterpolatedColorLutHandler"
);
#[cfg(feature = "OVRPassthroughLayer+InterpolatedColorLutHandler")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRPassthroughLayer_InterpolatedColorLutHandler {
    type Target = crate::GlobalNamespace::OVRPassthroughLayer_ColorLutHandler;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughLayer+InterpolatedColorLutHandler")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRPassthroughLayer_InterpolatedColorLutHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughLayer+InterpolatedColorLutHandler")]
impl crate::GlobalNamespace::OVRPassthroughLayer_InterpolatedColorLutHandler {
    pub fn set_LutTarget(
        &mut self,
        value: *mut OVRPassthroughColorLut,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_LutTarget", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_LutTarget(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut OVRPassthroughColorLut> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut OVRPassthroughColorLut = __cordl_object
            .invoke("get_LutTarget", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update_OVRPassthroughLayer_Settings0(
        &mut self,
        settings: crate::GlobalNamespace::OVRPassthroughLayer_Settings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (settings))?;
        Ok(__cordl_ret)
    }
    pub fn Update_OVRPassthroughColorLut_OVRPassthroughColorLut_f32_1(
        &mut self,
        lutSource: *mut OVRPassthroughColorLut,
        lutTarget: *mut OVRPassthroughColorLut,
        weight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (lutSource, lutTarget, weight))?;
        Ok(__cordl_ret)
    }
    pub fn ApplyStyleSettings(
        &mut self,
        style: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyle2,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyStyleSettings", (style))?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
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
#[cfg(feature = "OVRPassthroughLayer+InterpolatedColorLutHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPassthroughLayer_InterpolatedColorLutHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPassthroughLayer+MonoToMonoStyleHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPassthroughLayer_MonoToMonoStyleHandler {
    __cordl_parent: crate::GlobalNamespace::OVRPassthroughLayer_BaseGeneratedStyleHandler,
}
#[cfg(feature = "OVRPassthroughLayer+MonoToMonoStyleHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPassthroughLayer_MonoToMonoStyleHandler => ""
    ."OVRPassthroughLayer/MonoToMonoStyleHandler"
);
#[cfg(feature = "OVRPassthroughLayer+MonoToMonoStyleHandler")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRPassthroughLayer_MonoToMonoStyleHandler {
    type Target = crate::GlobalNamespace::OVRPassthroughLayer_BaseGeneratedStyleHandler;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughLayer+MonoToMonoStyleHandler")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRPassthroughLayer_MonoToMonoStyleHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughLayer+MonoToMonoStyleHandler")]
impl crate::GlobalNamespace::OVRPassthroughLayer_MonoToMonoStyleHandler {
    pub fn _ctor(
        &mut self,
        colorMapDataHandler: quest_hook::libil2cpp::ByRefMut<
            crate::System::Runtime::InteropServices::GCHandle,
        >,
        colorMapData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (colorMapDataHandler, colorMapData))?;
        Ok(__cordl_ret)
    }
    pub fn Update_OVRPassthroughLayer_Settings0(
        &mut self,
        settings: crate::GlobalNamespace::OVRPassthroughLayer_Settings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (settings))?;
        Ok(__cordl_ret)
    }
    pub fn Update_Il2CppArray1(
        &mut self,
        values: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (values))?;
        Ok(__cordl_ret)
    }
    pub fn get_MapSize(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_MapSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        colorMapDataHandler: quest_hook::libil2cpp::ByRefMut<
            crate::System::Runtime::InteropServices::GCHandle,
        >,
        colorMapData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (colorMapDataHandler, colorMapData))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVRPassthroughLayer+MonoToMonoStyleHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPassthroughLayer_MonoToMonoStyleHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPassthroughLayer+MonoToRgbaStyleHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPassthroughLayer_MonoToRgbaStyleHandler {
    __cordl_parent: crate::GlobalNamespace::OVRPassthroughLayer_BaseGeneratedStyleHandler,
    pub _tmpColorMapData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "OVRPassthroughLayer+MonoToRgbaStyleHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPassthroughLayer_MonoToRgbaStyleHandler => ""
    ."OVRPassthroughLayer/MonoToRgbaStyleHandler"
);
#[cfg(feature = "OVRPassthroughLayer+MonoToRgbaStyleHandler")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRPassthroughLayer_MonoToRgbaStyleHandler {
    type Target = crate::GlobalNamespace::OVRPassthroughLayer_BaseGeneratedStyleHandler;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughLayer+MonoToRgbaStyleHandler")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRPassthroughLayer_MonoToRgbaStyleHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughLayer+MonoToRgbaStyleHandler")]
impl crate::GlobalNamespace::OVRPassthroughLayer_MonoToRgbaStyleHandler {
    pub fn _ctor(
        &mut self,
        colorMapDataHandler: quest_hook::libil2cpp::ByRefMut<
            crate::System::Runtime::InteropServices::GCHandle,
        >,
        colorMapData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (colorMapDataHandler, colorMapData))?;
        Ok(__cordl_ret)
    }
    pub fn DeallocateColorMapData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DeallocateColorMapData", ())?;
        Ok(__cordl_ret)
    }
    pub fn AllocateColorMapData(
        &mut self,
        _cordl_size: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AllocateColorMapData", (_cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn get_MapSize(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_MapSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update_OVRPassthroughLayer_Settings0(
        &mut self,
        settings: crate::GlobalNamespace::OVRPassthroughLayer_Settings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (settings))?;
        Ok(__cordl_ret)
    }
    pub fn Update_Il2CppArray1(
        &mut self,
        values: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (values))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        colorMapDataHandler: quest_hook::libil2cpp::ByRefMut<
            crate::System::Runtime::InteropServices::GCHandle,
        >,
        colorMapData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (colorMapDataHandler, colorMapData))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVRPassthroughLayer+MonoToRgbaStyleHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPassthroughLayer_MonoToRgbaStyleHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPassthroughLayer+NoneStyleHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPassthroughLayer_NoneStyleHandler {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRPassthroughLayer+NoneStyleHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPassthroughLayer_NoneStyleHandler => ""
    ."OVRPassthroughLayer/NoneStyleHandler"
);
#[cfg(feature = "OVRPassthroughLayer+NoneStyleHandler")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPassthroughLayer_NoneStyleHandler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughLayer+NoneStyleHandler")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRPassthroughLayer_NoneStyleHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughLayer+NoneStyleHandler")]
impl crate::GlobalNamespace::OVRPassthroughLayer_NoneStyleHandler {
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
    pub fn ApplyStyleSettings(
        &mut self,
        style: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyle2,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyStyleSettings", (style))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsValid(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsValid", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
        settings: crate::GlobalNamespace::OVRPassthroughLayer_Settings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (settings))?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
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
#[cfg(feature = "OVRPassthroughLayer+NoneStyleHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPassthroughLayer_NoneStyleHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPassthroughLayer")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPassthroughLayer {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub projectionSurfaceType: crate::GlobalNamespace::OVRPassthroughLayer_ProjectionSurfaceType,
    pub overlayType: crate::GlobalNamespace::OVROverlay_OverlayType,
    pub compositionDepth: i32,
    pub hidden: bool,
    pub overridePerLayerColorScaleAndOffset: bool,
    pub colorScale: crate::UnityEngine::Vector4,
    pub colorOffset: crate::UnityEngine::Vector4,
    pub colorMapEditorType_: crate::GlobalNamespace::OVRPassthroughLayer_ColorMapEditorType,
    pub colorMapEditorGradient: *mut crate::UnityEngine::Gradient,
    pub colorMapEditorContrast: f32,
    pub colorMapEditorBrightness: f32,
    pub colorMapEditorPosterize: f32,
    pub colorMapEditorSaturation: f32,
    pub _colorLutSourceTexture: *mut crate::UnityEngine::Texture2D,
    pub _colorLutTargetTexture: *mut crate::UnityEngine::Texture2D,
    pub _lutWeight: f32,
    pub _flipLutY: bool,
    pub _settings: crate::GlobalNamespace::OVRPassthroughLayer_Settings,
    pub cameraRig: *mut OVRCameraRig,
    pub cameraRigInitialized: bool,
    pub auxGameObject: *mut crate::UnityEngine::GameObject,
    pub passthroughOverlay: *mut OVROverlay,
    pub surfaceGameObjects: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::UnityEngine::GameObject,
        crate::GlobalNamespace::OVRPassthroughLayer_PassthroughMeshInstance,
    >,
    pub deferredSurfaceGameObjects: *mut crate::System::Collections::Generic::List_1<
        crate::GlobalNamespace::OVRPassthroughLayer_DeferredPassthroughMeshAddition,
    >,
    pub serializedSurfaceGeometry: *mut crate::System::Collections::Generic::List_1<
        crate::GlobalNamespace::OVRPassthroughLayer_SerializedSurfaceGeometry,
    >,
    pub textureOpacity_: f32,
    pub edgeRenderingEnabled_: bool,
    pub edgeColor_: crate::UnityEngine::Color,
    pub colorMapType: crate::GlobalNamespace::OVRPlugin_InsightPassthroughColorMapType,
    pub styleDirty: bool,
    pub _stylesHandler: *mut crate::GlobalNamespace::OVRPassthroughLayer_StylesHandler,
}
#[cfg(feature = "OVRPassthroughLayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRPassthroughLayer => ""."OVRPassthroughLayer"
);
#[cfg(feature = "OVRPassthroughLayer")]
impl std::ops::Deref for OVRPassthroughLayer {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughLayer")]
impl std::ops::DerefMut for OVRPassthroughLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughLayer")]
impl OVRPassthroughLayer {
    #[cfg(feature = "OVRPassthroughLayer+InterpolatedColorLutHandler")]
    pub type InterpolatedColorLutHandler = crate::GlobalNamespace::OVRPassthroughLayer_InterpolatedColorLutHandler;
    #[cfg(feature = "OVRPassthroughLayer+PassthroughMeshInstance")]
    pub type PassthroughMeshInstance = crate::GlobalNamespace::OVRPassthroughLayer_PassthroughMeshInstance;
    #[cfg(feature = "OVRPassthroughLayer+SerializedSurfaceGeometry")]
    pub type SerializedSurfaceGeometry = crate::GlobalNamespace::OVRPassthroughLayer_SerializedSurfaceGeometry;
    #[cfg(feature = "OVRPassthroughLayer+IStyleHandler")]
    type IStyleHandler = crate::GlobalNamespace::OVRPassthroughLayer_IStyleHandler;
    #[cfg(feature = "OVRPassthroughLayer+ProjectionSurfaceType")]
    pub type ProjectionSurfaceType = crate::GlobalNamespace::OVRPassthroughLayer_ProjectionSurfaceType;
    #[cfg(feature = "OVRPassthroughLayer+MonoToRgbaStyleHandler")]
    pub type MonoToRgbaStyleHandler = crate::GlobalNamespace::OVRPassthroughLayer_MonoToRgbaStyleHandler;
    #[cfg(feature = "OVRPassthroughLayer+ColorMapEditorType")]
    pub type ColorMapEditorType = crate::GlobalNamespace::OVRPassthroughLayer_ColorMapEditorType;
    #[cfg(feature = "OVRPassthroughLayer+MonoToMonoStyleHandler")]
    pub type MonoToMonoStyleHandler = crate::GlobalNamespace::OVRPassthroughLayer_MonoToMonoStyleHandler;
    #[cfg(feature = "OVRPassthroughLayer+DeferredPassthroughMeshAddition")]
    pub type DeferredPassthroughMeshAddition = crate::GlobalNamespace::OVRPassthroughLayer_DeferredPassthroughMeshAddition;
    #[cfg(feature = "OVRPassthroughLayer+BaseGeneratedStyleHandler")]
    pub type BaseGeneratedStyleHandler = crate::GlobalNamespace::OVRPassthroughLayer_BaseGeneratedStyleHandler;
    #[cfg(feature = "OVRPassthroughLayer+ColorLutHandler")]
    pub type ColorLutHandler = crate::GlobalNamespace::OVRPassthroughLayer_ColorLutHandler;
    #[cfg(feature = "OVRPassthroughLayer+StylesHandler")]
    pub type StylesHandler = crate::GlobalNamespace::OVRPassthroughLayer_StylesHandler;
    #[cfg(feature = "OVRPassthroughLayer+__c__DisplayClass10_0")]
    pub type __c__DisplayClass10_0 = crate::GlobalNamespace::OVRPassthroughLayer___c__DisplayClass10_0;
    #[cfg(feature = "OVRPassthroughLayer+__c__DisplayClass9_0")]
    pub type __c__DisplayClass9_0 = crate::GlobalNamespace::OVRPassthroughLayer___c__DisplayClass9_0;
    #[cfg(feature = "OVRPassthroughLayer+BCSStyleHandler")]
    pub type BCSStyleHandler = crate::GlobalNamespace::OVRPassthroughLayer_BCSStyleHandler;
    #[cfg(feature = "OVRPassthroughLayer+Settings")]
    pub type Settings = crate::GlobalNamespace::OVRPassthroughLayer_Settings;
    #[cfg(feature = "OVRPassthroughLayer+NoneStyleHandler")]
    pub type NoneStyleHandler = crate::GlobalNamespace::OVRPassthroughLayer_NoneStyleHandler;
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetStyleDirty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStyleDirty", ())?;
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
    pub fn UpdateColorMapFromControls(
        &mut self,
        forceUpdate: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateColorMapFromControls", (forceUpdate))?;
        Ok(__cordl_ret)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn DestroySurfaceGeometries(
        &mut self,
        addBackToDeferredQueue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroySurfaceGeometries", (addBackToDeferredQueue))?;
        Ok(__cordl_ret)
    }
    pub fn AddSurfaceGeometry(
        &mut self,
        obj: *mut crate::UnityEngine::GameObject,
        updateTransform: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSurfaceGeometry", (obj, updateTransform))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateSurfaceGeometryTransform(
        &mut self,
        instanceHandle: u64,
        localToWorld: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateSurfaceGeometryTransform", (instanceHandle, localToWorld))?;
        Ok(__cordl_ret)
    }
    pub fn set_edgeRenderingEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_edgeRenderingEnabled", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetColorMapControls(
        &mut self,
        contrast: f32,
        brightness: f32,
        posterize: f32,
        gradient: *mut crate::UnityEngine::Gradient,
        colorMapType: crate::GlobalNamespace::OVRPassthroughLayer_ColorMapEditorType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetColorMapControls",
                (contrast, brightness, posterize, gradient, colorMapType),
            )?;
        Ok(__cordl_ret)
    }
    pub fn set_textureOpacity(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_textureOpacity", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetColorMap(
        &mut self,
        values: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorMap", (values))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateSurfaceGeometryTransforms(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateSurfaceGeometryTransforms", ())?;
        Ok(__cordl_ret)
    }
    pub fn SyncToOverlay(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SyncToOverlay", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetColorMapMonochromatic(
        &mut self,
        values: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorMapMonochromatic", (values))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveSurfaceGeometry(
        &mut self,
        obj: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveSurfaceGeometry", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn get_textureOpacity(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_textureOpacity", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_overlayShape(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVROverlay_OverlayShape> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVROverlay_OverlayShape = __cordl_object
            .invoke("get_overlayShape", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsSurfaceGeometry(
        &mut self,
        obj: *mut crate::UnityEngine::GameObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSurfaceGeometry", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn CreateOvrPluginStyleObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyle2,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_InsightPassthroughStyle2 = __cordl_object
            .invoke("CreateOvrPluginStyleObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateAndAddMesh(
        &mut self,
        obj: *mut crate::UnityEngine::GameObject,
        meshHandle: quest_hook::libil2cpp::ByRefMut<u64>,
        instanceHandle: quest_hook::libil2cpp::ByRefMut<u64>,
        localToWorld: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "CreateAndAddMesh",
                (obj, meshHandle, instanceHandle, localToWorld),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_edgeRenderingEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_edgeRenderingEnabled", ())?;
        Ok(__cordl_ret)
    }
    pub fn HasControlsBasedColorMap(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasControlsBasedColorMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn GetTransformMatrixForPassthroughSurfaceObject(
        &mut self,
        worldFromObj: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = __cordl_object
            .invoke("GetTransformMatrixForPassthroughSurfaceObject", (worldFromObj))?;
        Ok(__cordl_ret)
    }
    pub fn SetColorLut_f32_0(
        &mut self,
        lut: *mut OVRPassthroughColorLut,
        weight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorLut", (lut, weight))?;
        Ok(__cordl_ret)
    }
    pub fn SetColorLut_OVRPassthroughColorLut_f32_1(
        &mut self,
        lutSource: *mut OVRPassthroughColorLut,
        lutTarget: *mut OVRPassthroughColorLut,
        weight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorLut", (lutSource, lutTarget, weight))?;
        Ok(__cordl_ret)
    }
    pub fn AddDeferredSurfaceGeometries(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddDeferredSurfaceGeometries", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_edgeColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_edgeColor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn DisableColorMap(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableColorMap", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_edgeColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_edgeColor", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_colorMapEditorType(
        &mut self,
        value: crate::GlobalNamespace::OVRPassthroughLayer_ColorMapEditorType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_colorMapEditorType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SetBrightnessContrastSaturation(
        &mut self,
        brightness: f32,
        contrast: f32,
        saturation: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SetBrightnessContrastSaturation",
                (brightness, contrast, saturation),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_colorMapEditorType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPassthroughLayer_ColorMapEditorType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRPassthroughLayer_ColorMapEditorType = __cordl_object
            .invoke("get_colorMapEditorType", ())?;
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
#[cfg(feature = "OVRPassthroughLayer")]
impl quest_hook::libil2cpp::ObjectType for OVRPassthroughLayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPassthroughLayer+PassthroughMeshInstance")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPassthroughLayer_PassthroughMeshInstance {
    pub meshHandle: u64,
    pub instanceHandle: u64,
    pub updateTransform: bool,
    pub localToWorld: crate::UnityEngine::Matrix4x4,
}
#[cfg(feature = "OVRPassthroughLayer+PassthroughMeshInstance")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPassthroughLayer_PassthroughMeshInstance => ""
    ."OVRPassthroughLayer/PassthroughMeshInstance"
);
#[cfg(feature = "OVRPassthroughLayer+PassthroughMeshInstance")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPassthroughLayer_PassthroughMeshInstance {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPassthroughLayer+PassthroughMeshInstance")]
impl crate::GlobalNamespace::OVRPassthroughLayer_PassthroughMeshInstance {}
#[cfg(feature = "OVRPassthroughLayer+ProjectionSurfaceType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRPassthroughLayer_ProjectionSurfaceType {
    Reconstructed = 0i32,
    UserDefined = 1i32,
}
#[cfg(feature = "OVRPassthroughLayer+ProjectionSurfaceType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPassthroughLayer_ProjectionSurfaceType => ""
    ."OVRPassthroughLayer/ProjectionSurfaceType"
);
#[cfg(feature = "OVRPassthroughLayer+SerializedSurfaceGeometry")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPassthroughLayer_SerializedSurfaceGeometry {
    pub meshFilter: *mut crate::UnityEngine::MeshFilter,
    pub updateTransform: bool,
}
#[cfg(feature = "OVRPassthroughLayer+SerializedSurfaceGeometry")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPassthroughLayer_SerializedSurfaceGeometry => ""
    ."OVRPassthroughLayer/SerializedSurfaceGeometry"
);
#[cfg(feature = "OVRPassthroughLayer+SerializedSurfaceGeometry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPassthroughLayer_SerializedSurfaceGeometry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPassthroughLayer+SerializedSurfaceGeometry")]
impl crate::GlobalNamespace::OVRPassthroughLayer_SerializedSurfaceGeometry {}
#[cfg(feature = "OVRPassthroughLayer+Settings")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRPassthroughLayer_Settings {
    pub colorLutTargetTexture: *mut crate::UnityEngine::Texture2D,
    pub colorLutSourceTexture: *mut crate::UnityEngine::Texture2D,
    pub saturation: f32,
    pub posterize: f32,
    pub brightness: f32,
    pub contrast: f32,
    pub gradient: *mut crate::UnityEngine::Gradient,
    pub lutWeight: f32,
    pub flipLutY: bool,
}
#[cfg(feature = "OVRPassthroughLayer+Settings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPassthroughLayer_Settings =>
    ""."OVRPassthroughLayer/Settings"
);
#[cfg(feature = "OVRPassthroughLayer+Settings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPassthroughLayer_Settings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPassthroughLayer+Settings")]
impl crate::GlobalNamespace::OVRPassthroughLayer_Settings {
    pub fn _ctor(
        &mut self,
        colorLutTargetTexture: *mut crate::UnityEngine::Texture2D,
        colorLutSourceTexture: *mut crate::UnityEngine::Texture2D,
        saturation: f32,
        posterize: f32,
        brightness: f32,
        contrast: f32,
        gradient: *mut crate::UnityEngine::Gradient,
        lutWeight: f32,
        flipLutY: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (
                colorLutTargetTexture,
                colorLutSourceTexture,
                saturation,
                posterize,
                brightness,
                contrast,
                gradient,
                lutWeight,
                flipLutY,
            ),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRPassthroughLayer+StylesHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPassthroughLayer_StylesHandler {
    __cordl_parent: crate::System::Object,
    pub _noneHandler: *mut crate::GlobalNamespace::OVRPassthroughLayer_NoneStyleHandler,
    pub _lutHandler: *mut crate::GlobalNamespace::OVRPassthroughLayer_ColorLutHandler,
    pub _interpolatedLutHandler: *mut crate::GlobalNamespace::OVRPassthroughLayer_InterpolatedColorLutHandler,
    pub _monoToRgbaHandler: *mut crate::GlobalNamespace::OVRPassthroughLayer_MonoToRgbaStyleHandler,
    pub _monoToMonoHandler: *mut crate::GlobalNamespace::OVRPassthroughLayer_MonoToMonoStyleHandler,
    pub _bcsHandler: *mut crate::GlobalNamespace::OVRPassthroughLayer_BCSStyleHandler,
    pub _colorMapDataHandle: crate::System::Runtime::InteropServices::GCHandle,
    pub _colorMapData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub CurrentStyleHandler: *mut crate::GlobalNamespace::OVRPassthroughLayer_IStyleHandler,
}
#[cfg(feature = "OVRPassthroughLayer+StylesHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPassthroughLayer_StylesHandler => ""
    ."OVRPassthroughLayer/StylesHandler"
);
#[cfg(feature = "OVRPassthroughLayer+StylesHandler")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPassthroughLayer_StylesHandler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughLayer+StylesHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPassthroughLayer_StylesHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughLayer+StylesHandler")]
impl crate::GlobalNamespace::OVRPassthroughLayer_StylesHandler {
    pub fn SetMonoToMonoHandler(
        &mut self,
        values: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMonoToMonoHandler", (values))?;
        Ok(__cordl_ret)
    }
    pub fn SetMonoToRgbaHandler(
        &mut self,
        values: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetMonoToRgbaHandler", (values))?;
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
    pub fn GetStyleHandler(
        &mut self,
        _cordl_type: crate::GlobalNamespace::OVRPlugin_InsightPassthroughColorMapType,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::GlobalNamespace::OVRPassthroughLayer_IStyleHandler,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::OVRPassthroughLayer_IStyleHandler = __cordl_object
            .invoke("GetStyleHandler", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn SetInterpolatedColorLutHandler(
        &mut self,
        lutSource: *mut OVRPassthroughColorLut,
        lutTarget: *mut OVRPassthroughColorLut,
        weight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInterpolatedColorLutHandler", (lutSource, lutTarget, weight))?;
        Ok(__cordl_ret)
    }
    pub fn SetStyleHandler(
        &mut self,
        _cordl_type: crate::GlobalNamespace::OVRPlugin_InsightPassthroughColorMapType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetStyleHandler", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn SetColorLutHandler(
        &mut self,
        lut: *mut OVRPassthroughColorLut,
        weight: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorLutHandler", (lut, weight))?;
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
#[cfg(feature = "OVRPassthroughLayer+StylesHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPassthroughLayer_StylesHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
