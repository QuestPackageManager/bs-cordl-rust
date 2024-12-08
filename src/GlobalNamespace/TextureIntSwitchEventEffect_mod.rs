#[cfg(feature = "TextureIntSwitchEventEffect")]
#[repr(C)]
#[derive(Debug)]
pub struct TextureIntSwitchEventEffect {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _materialPropertyBlockController: *mut crate::GlobalNamespace::MaterialPropertyBlockController,
    pub _texturePropertyName: *mut crate::System::String,
    pub _beatmapEventType: crate::GlobalNamespace::BasicBeatmapEventType,
    pub _defaultIndex: i32,
    pub _textureValueTuples: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::GlobalNamespace::TextureIntSwitchEventEffect_TextureValueTuple,
    >,
    pub _beatmapCallbacksController: *mut crate::GlobalNamespace::BeatmapCallbacksController,
    pub _beatmapDataCallbackWrapper: *mut crate::GlobalNamespace::BeatmapDataCallbackWrapper,
    pub _texturePropertyId: i32,
    pub _valueToTextureMap: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::UnityEngine::Texture,
    >,
}
#[cfg(feature = "TextureIntSwitchEventEffect")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TextureIntSwitchEventEffect =>
    ""."TextureIntSwitchEventEffect"
);
#[cfg(feature = "TextureIntSwitchEventEffect")]
impl std::ops::Deref for crate::GlobalNamespace::TextureIntSwitchEventEffect {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TextureIntSwitchEventEffect")]
impl std::ops::DerefMut for crate::GlobalNamespace::TextureIntSwitchEventEffect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TextureIntSwitchEventEffect")]
impl crate::GlobalNamespace::TextureIntSwitchEventEffect {
    #[cfg(feature = "TextureIntSwitchEventEffect+TextureValueTuple")]
    pub type TextureValueTuple = crate::GlobalNamespace::TextureIntSwitchEventEffect_TextureValueTuple;
    pub fn HandleBasicBeatmapEventData(
        &mut self,
        data: *mut crate::GlobalNamespace::BasicBeatmapEventData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleBasicBeatmapEventData", (data))?;
        Ok(__cordl_ret)
    }
    pub fn Initialize(
        &mut self,
        beatmapCallbacksController: *mut crate::GlobalNamespace::BeatmapCallbacksController,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", (beatmapCallbacksController))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
    pub fn SetTextureByIndex(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTextureByIndex", (value))?;
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
#[cfg(feature = "TextureIntSwitchEventEffect")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::TextureIntSwitchEventEffect {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TextureIntSwitchEventEffect+TextureValueTuple")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TextureIntSwitchEventEffect_TextureValueTuple {
    pub value: i32,
    pub texture: *mut crate::UnityEngine::Texture,
}
#[cfg(feature = "TextureIntSwitchEventEffect+TextureValueTuple")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::TextureIntSwitchEventEffect_TextureValueTuple => ""
    ."TextureIntSwitchEventEffect/TextureValueTuple"
);
#[cfg(feature = "TextureIntSwitchEventEffect+TextureValueTuple")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::TextureIntSwitchEventEffect_TextureValueTuple {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "TextureIntSwitchEventEffect+TextureValueTuple")]
impl crate::GlobalNamespace::TextureIntSwitchEventEffect_TextureValueTuple {}
