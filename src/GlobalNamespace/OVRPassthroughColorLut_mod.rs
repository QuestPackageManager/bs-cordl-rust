#[cfg(feature = "OVRPassthroughColorLut+ColorLutTextureConverter+MapColorValuesJob")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ColorLutTextureConverter_OVRPassthroughColorLut_MapColorValuesJob {
    pub settings: crate::GlobalNamespace::ColorLutTextureConverter_OVRPassthroughColorLut_TextureSettings,
    pub target: crate::Unity::Collections::NativeArray_1<u8>,
    pub source: crate::Unity::Collections::NativeArray_1<u8>,
}
#[cfg(feature = "OVRPassthroughColorLut+ColorLutTextureConverter+MapColorValuesJob")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ColorLutTextureConverter_OVRPassthroughColorLut_MapColorValuesJob
    => ""."OVRPassthroughColorLut/ColorLutTextureConverter/MapColorValuesJob"
);
#[cfg(feature = "OVRPassthroughColorLut+ColorLutTextureConverter+MapColorValuesJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::ColorLutTextureConverter_OVRPassthroughColorLut_MapColorValuesJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPassthroughColorLut+ColorLutTextureConverter+MapColorValuesJob")]
impl crate::GlobalNamespace::ColorLutTextureConverter_OVRPassthroughColorLut_MapColorValuesJob {
    pub fn Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPassthroughColorLut+ColorLutTextureConverter+MapColorValuesJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelFor>
for crate::GlobalNamespace::ColorLutTextureConverter_OVRPassthroughColorLut_MapColorValuesJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "OVRPassthroughColorLut+ColorLutTextureConverter+MapColorValuesJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelFor>
for crate::GlobalNamespace::ColorLutTextureConverter_OVRPassthroughColorLut_MapColorValuesJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "OVRPassthroughColorLut+ColorLutTextureConverter+TextureSettings")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ColorLutTextureConverter_OVRPassthroughColorLut_TextureSettings {
    pub _Width_k__BackingField: i32,
    pub _Height_k__BackingField: i32,
    pub _Resolution_k__BackingField: i32,
    pub _SlicesPerRow_k__BackingField: i32,
    pub _ChannelCount_k__BackingField: i32,
    pub _FlipY_k__BackingField: bool,
}
#[cfg(feature = "OVRPassthroughColorLut+ColorLutTextureConverter+TextureSettings")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::ColorLutTextureConverter_OVRPassthroughColorLut_TextureSettings =>
    ""."OVRPassthroughColorLut/ColorLutTextureConverter/TextureSettings"
);
#[cfg(feature = "OVRPassthroughColorLut+ColorLutTextureConverter+TextureSettings")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::ColorLutTextureConverter_OVRPassthroughColorLut_TextureSettings {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPassthroughColorLut+ColorLutTextureConverter+TextureSettings")]
impl crate::GlobalNamespace::ColorLutTextureConverter_OVRPassthroughColorLut_TextureSettings {
    pub fn _ctor(
        &mut self,
        width: i32,
        height: i32,
        resolution: i32,
        slicesPerRow: i32,
        channelCount: i32,
        flipY: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (width, height, resolution, slicesPerRow, channelCount, flipY),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ChannelCount(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ChannelCount",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FlipY(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_FlipY",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Height(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Height",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Resolution(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Resolution",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SlicesPerRow(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_SlicesPerRow",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Width(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Width",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPassthroughColorLut")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPassthroughColorLut {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Resolution_k__BackingField: u32,
    pub _Channels_k__BackingField: crate::GlobalNamespace::OVRPassthroughColorLut_ColorChannels,
    pub _IsInitialized_k__BackingField: bool,
    pub _colorLutHandle: u64,
    pub _allocHandle: crate::System::Runtime::InteropServices::GCHandle,
    pub _lutData: crate::GlobalNamespace::OVRPlugin_PassthroughColorLutData,
    pub _channelCount: i32,
    pub _colorBytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub _locker: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "OVRPassthroughColorLut")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRPassthroughColorLut => ""
    ."OVRPassthroughColorLut"
);
#[cfg(feature = "OVRPassthroughColorLut")]
impl std::ops::Deref for crate::GlobalNamespace::OVRPassthroughColorLut {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughColorLut")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRPassthroughColorLut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughColorLut")]
impl crate::GlobalNamespace::OVRPassthroughColorLut {
    pub const RecomendedBatchSize: i32 = 128i32;
    #[cfg(feature = "OVRPassthroughColorLut+ColorChannels")]
    pub type ColorChannels = crate::GlobalNamespace::OVRPassthroughColorLut_ColorChannels;
    #[cfg(feature = "OVRPassthroughColorLut+ColorLutTextureConverter")]
    pub type ColorLutTextureConverter = crate::GlobalNamespace::OVRPassthroughColorLut_ColorLutTextureConverter;
    #[cfg(feature = "OVRPassthroughColorLut+WriteColorsAsBytesJob")]
    pub type WriteColorsAsBytesJob = crate::GlobalNamespace::OVRPassthroughColorLut_WriteColorsAsBytesJob;
    pub fn ChannelsToCount(
        channels: crate::GlobalNamespace::OVRPassthroughColorLut_ColorChannels,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ChannelsToCount", (channels))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create(
        &mut self,
        lutData: crate::GlobalNamespace::OVRPlugin_PassthroughColorLutData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Create", (lutData))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateLutData(
        &mut self,
        colorBytes: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_PassthroughColorLutData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_PassthroughColorLutData = __cordl_object
            .invoke("CreateLutData", (colorBytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateLutDataFromArray_Il2CppArray0(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_PassthroughColorLutData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_PassthroughColorLutData = __cordl_object
            .invoke("CreateLutDataFromArray", (colors))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateLutDataFromArray_Il2CppArray1(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_PassthroughColorLutData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_PassthroughColorLutData = __cordl_object
            .invoke("CreateLutDataFromArray", (colors))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateLutDataFromArray_Il2CppArray2(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_PassthroughColorLutData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_PassthroughColorLutData = __cordl_object
            .invoke("CreateLutDataFromArray", (colors))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateLutDataFromTexture(
        &mut self,
        lut: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        flipY: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_PassthroughColorLutData,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_PassthroughColorLutData = __cordl_object
            .invoke("CreateLutDataFromTexture", (lut, flipY))?;
        Ok(__cordl_ret.into())
    }
    pub fn Destroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Destroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetArraySize<T>(
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetArraySize", (array))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetChannelsForTextureFormat(
        format: crate::UnityEngine::TextureFormat,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPassthroughColorLut_ColorChannels,
    > {
        let __cordl_ret: crate::GlobalNamespace::OVRPassthroughColorLut_ColorChannels = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetChannelsForTextureFormat", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResolutionFromSize(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetResolutionFromSize", (_cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTextureSize(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTextureSize", (texture))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTextureSizeFromByteArray(
        initialColorLut: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        channels: crate::GlobalNamespace::OVRPassthroughColorLut_ColorChannels,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTextureSizeFromByteArray", (initialColorLut, channels))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPowerOfTwo(x: u32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPowerOfTwo", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsResolutionAccepted(
        resolution: u32,
        _cordl_size: i32,
        errorMessage: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsResolutionAccepted", (resolution, _cordl_size, errorMessage))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTextureSupported(
        texture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        errorMessage: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsTextureSupported", (texture, errorMessage))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidLutUpdate<T>(
        &mut self,
        colorArray: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        elementByteSize: i32,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsValidLutUpdate", (colorArray, elementByteSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValidUpdateResolution(
        &mut self,
        lutSize: i32,
        elementByteSize: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsValidUpdateResolution", (lutSize, elementByteSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_Il2CppArray_OVRPassthroughColorLut_ColorChannels1(
        initialColorLut: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
        channels: crate::GlobalNamespace::OVRPassthroughColorLut_ColorChannels,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialColorLut, channels))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray_OVRPassthroughColorLut_ColorChannels2(
        initialColorLut: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
        channels: crate::GlobalNamespace::OVRPassthroughColorLut_ColorChannels,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialColorLut, channels))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppArray_OVRPassthroughColorLut_ColorChannels3(
        initialColorLut: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        channels: crate::GlobalNamespace::OVRPassthroughColorLut_ColorChannels,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialColorLut, channels))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Texture2D__cordl_bool0(
        initialLutTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        flipY: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialLutTexture, flipY))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_OVRPassthroughColorLut_ColorChannels4(
        _cordl_size: i32,
        channels: crate::GlobalNamespace::OVRPassthroughColorLut_ColorChannels,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_size, channels))?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateFrom_Il2CppArray0(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateFrom", (colors))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateFrom_Il2CppArray1(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateFrom", (colors))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateFrom_Il2CppArray2(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateFrom", (colors))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateFrom_Texture2D__cordl_bool3(
        &mut self,
        lutTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        flipY: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateFrom", (lutTexture, flipY))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteColorsAsBytes_Il2CppArray_Il2CppArray0(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteColorsAsBytes", (colors, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteColorsAsBytes_Il2CppArray_Il2CppArray1(
        &mut self,
        colors: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteColorsAsBytes", (colors, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray_OVRPassthroughColorLut_ColorChannels1(
        &mut self,
        initialColorLut: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
        >,
        channels: crate::GlobalNamespace::OVRPassthroughColorLut_ColorChannels,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialColorLut, channels))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray_OVRPassthroughColorLut_ColorChannels2(
        &mut self,
        initialColorLut: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color32>,
        >,
        channels: crate::GlobalNamespace::OVRPassthroughColorLut_ColorChannels,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialColorLut, channels))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray_OVRPassthroughColorLut_ColorChannels3(
        &mut self,
        initialColorLut: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        channels: crate::GlobalNamespace::OVRPassthroughColorLut_ColorChannels,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialColorLut, channels))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Texture2D__cordl_bool0(
        &mut self,
        initialLutTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        flipY: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialLutTexture, flipY))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_OVRPassthroughColorLut_ColorChannels4(
        &mut self,
        _cordl_size: i32,
        channels: crate::GlobalNamespace::OVRPassthroughColorLut_ColorChannels,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_size, channels))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Channels(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPassthroughColorLut_ColorChannels,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRPassthroughColorLut_ColorChannels = __cordl_object
            .invoke("get_Channels", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsInitialized(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsInitialized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Resolution(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_Resolution", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Channels(
        &mut self,
        value: crate::GlobalNamespace::OVRPassthroughColorLut_ColorChannels,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Channels", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsInitialized(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsInitialized", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Resolution(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Resolution", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPassthroughColorLut")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPassthroughColorLut {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPassthroughColorLut")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::OVRPassthroughColorLut {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRPassthroughColorLut")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::OVRPassthroughColorLut {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRPassthroughColorLut+ColorChannels")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRPassthroughColorLut_ColorChannels {
    #[default]
    Rgb = 1i32,
    Rgba = 2i32,
}
#[cfg(feature = "OVRPassthroughColorLut+ColorChannels")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPassthroughColorLut_ColorChannels => ""
    ."OVRPassthroughColorLut/ColorChannels"
);
#[cfg(feature = "OVRPassthroughColorLut+ColorLutTextureConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRPassthroughColorLut_ColorLutTextureConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRPassthroughColorLut+ColorLutTextureConverter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPassthroughColorLut_ColorLutTextureConverter => ""
    ."OVRPassthroughColorLut/ColorLutTextureConverter"
);
#[cfg(feature = "OVRPassthroughColorLut+ColorLutTextureConverter")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRPassthroughColorLut_ColorLutTextureConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughColorLut+ColorLutTextureConverter")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRPassthroughColorLut_ColorLutTextureConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRPassthroughColorLut+ColorLutTextureConverter")]
impl crate::GlobalNamespace::OVRPassthroughColorLut_ColorLutTextureConverter {
    #[cfg(feature = "OVRPassthroughColorLut+ColorLutTextureConverter+MapColorValuesJob")]
    pub type MapColorValuesJob = crate::GlobalNamespace::ColorLutTextureConverter_OVRPassthroughColorLut_MapColorValuesJob;
    #[cfg(feature = "OVRPassthroughColorLut+ColorLutTextureConverter+TextureSettings")]
    pub type TextureSettings = crate::GlobalNamespace::ColorLutTextureConverter_OVRPassthroughColorLut_TextureSettings;
    pub fn GetTextureSettings(
        lut: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        channelCount: i32,
        flipY: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::ColorLutTextureConverter_OVRPassthroughColorLut_TextureSettings,
    > {
        let __cordl_ret: crate::GlobalNamespace::ColorLutTextureConverter_OVRPassthroughColorLut_TextureSettings = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTextureSettings", (lut, channelCount, flipY))?;
        Ok(__cordl_ret.into())
    }
    pub fn MapColorValues(
        settings: crate::GlobalNamespace::ColorLutTextureConverter_OVRPassthroughColorLut_TextureSettings,
        source: crate::Unity::Collections::NativeArray_1<u8>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MapColorValues", (settings, source, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn TextureToColorByteMap(
        lut: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        channelCount: i32,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        flipY: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TextureToColorByteMap", (lut, channelCount, target, flipY))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetTextureLayout(
        width: i32,
        height: i32,
        resolution: quest_hook::libil2cpp::ByRefMut<i32>,
        slicesPerRow: quest_hook::libil2cpp::ByRefMut<i32>,
        errorMessage: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryGetTextureLayout",
                (width, height, resolution, slicesPerRow, errorMessage),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPassthroughColorLut+ColorLutTextureConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRPassthroughColorLut_ColorLutTextureConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRPassthroughColorLut+WriteColorsAsBytesJob")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRPassthroughColorLut_WriteColorsAsBytesJob {
    pub target: crate::Unity::Collections::NativeArray_1<u8>,
    pub source: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Color>,
    pub channelCount: i32,
}
#[cfg(feature = "OVRPassthroughColorLut+WriteColorsAsBytesJob")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRPassthroughColorLut_WriteColorsAsBytesJob => ""
    ."OVRPassthroughColorLut/WriteColorsAsBytesJob"
);
#[cfg(feature = "OVRPassthroughColorLut+WriteColorsAsBytesJob")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRPassthroughColorLut_WriteColorsAsBytesJob {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRPassthroughColorLut+WriteColorsAsBytesJob")]
impl crate::GlobalNamespace::OVRPassthroughColorLut_WriteColorsAsBytesJob {
    pub fn Execute(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Execute",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRPassthroughColorLut+WriteColorsAsBytesJob")]
impl AsRef<crate::Unity::Jobs::IJobParallelFor>
for crate::GlobalNamespace::OVRPassthroughColorLut_WriteColorsAsBytesJob {
    fn as_ref(&self) -> &crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
#[cfg(feature = "OVRPassthroughColorLut+WriteColorsAsBytesJob")]
impl AsMut<crate::Unity::Jobs::IJobParallelFor>
for crate::GlobalNamespace::OVRPassthroughColorLut_WriteColorsAsBytesJob {
    fn as_mut(&mut self) -> &mut crate::Unity::Jobs::IJobParallelFor {
        todo!()
    }
}
