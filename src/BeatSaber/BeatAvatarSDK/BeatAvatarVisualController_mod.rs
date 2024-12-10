#[cfg(feature = "BeatSaber+BeatAvatarSDK+BeatAvatarVisualController")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatAvatarVisualController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _headTopMeshFilter: *mut crate::UnityEngine::MeshFilter,
    pub _glassesMeshFilter: *mut crate::UnityEngine::MeshFilter,
    pub _facialHairMeshFilter: *mut crate::UnityEngine::MeshFilter,
    pub _leftHandsHairMeshFilter: *mut crate::UnityEngine::MeshFilter,
    pub _rightHandsHairMeshFilter: *mut crate::UnityEngine::MeshFilter,
    pub _eyesSprite: *mut crate::UnityEngine::SpriteRenderer,
    pub _mouthSprite: *mut crate::UnityEngine::SpriteRenderer,
    pub _bodyMeshFilter: *mut crate::UnityEngine::MeshFilter,
    pub _headTopPropertyBlockColorSetter: *mut crate::BeatSaber::BeatAvatarSDK::MulticolorAvatarPartPropertyBlockSetter,
    pub _glassesPropertyBlockColorSetter: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPropertyBlockColorSetter,
    pub _facialHairPropertyBlockColorSetter: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPropertyBlockColorSetter,
    pub _skinPropertyBlockColorSetter: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPropertyBlockColorSetter,
    pub _clothesPropertyBlockSetter: *mut crate::BeatSaber::BeatAvatarSDK::MulticolorAvatarPartPropertyBlockSetter,
    pub _leftHandPropertyBlockSetter: *mut crate::BeatSaber::BeatAvatarSDK::MulticolorAvatarPartPropertyBlockSetter,
    pub _rightHandPropertyBlockSetter: *mut crate::BeatSaber::BeatAvatarSDK::MulticolorAvatarPartPropertyBlockSetter,
    pub _avatarPartsModel: *mut crate::BeatSaber::BeatAvatarSDK::AvatarPartsModel,
    pub _avatarPartHighlightSetters: *mut crate::System::Collections::Generic::Dictionary_2<
        crate::BeatSaber::BeatAvatarSDK::AvatarPart,
        *mut crate::BeatSaber::BeatAvatarSDK::BeatAvatarVisualController_HighlighterDelegate,
    >,
    pub _lightColor: crate::UnityEngine::Color,
    pub _avatarData: *mut crate::BeatSaber::BeatAvatarSDK::AvatarData,
    pub _currentHighlighter: *mut crate::BeatSaber::BeatAvatarSDK::BeatAvatarVisualController_HighlighterDelegate,
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+BeatAvatarVisualController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::BeatAvatarSDK::BeatAvatarVisualController => "BeatSaber.BeatAvatarSDK"
    ."BeatAvatarVisualController"
);
#[cfg(feature = "BeatSaber+BeatAvatarSDK+BeatAvatarVisualController")]
impl std::ops::Deref for crate::BeatSaber::BeatAvatarSDK::BeatAvatarVisualController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+BeatAvatarVisualController")]
impl std::ops::DerefMut for crate::BeatSaber::BeatAvatarSDK::BeatAvatarVisualController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+BeatAvatarVisualController")]
impl crate::BeatSaber::BeatAvatarSDK::BeatAvatarVisualController {
    #[cfg(
        feature = "BeatSaber+BeatAvatarSDK+BeatAvatarVisualController+HighlighterDelegate"
    )]
    pub type HighlighterDelegate = crate::BeatSaber::BeatAvatarSDK::BeatAvatarVisualController_HighlighterDelegate;
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DisableEditedPartHighlight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisableEditedPartHighlight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HighlightEditedPart(
        &mut self,
        editPart: crate::BeatSaber::BeatAvatarSDK::AvatarPart,
        uvSegment: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HighlightEditedPart", (editPart, uvSegment))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetHandsHighlight(
        &mut self,
        highlighted: bool,
        uvSegment: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetHandsHighlight", (highlighted, uvSegment))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLightColor(
        &mut self,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLightColor", (color))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAvatarColors(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAvatarColors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAvatarVisual(
        &mut self,
        avatarData: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::BeatAvatarSDK::AvatarData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAvatarVisual", (avatarData))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+BeatAvatarVisualController")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarSDK::BeatAvatarVisualController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "BeatSaber+BeatAvatarSDK+BeatAvatarVisualController+HighlighterDelegate"
)]
#[repr(C)]
#[derive(Debug)]
pub struct BeatAvatarVisualController_HighlighterDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(
    feature = "BeatSaber+BeatAvatarSDK+BeatAvatarVisualController+HighlighterDelegate"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::BeatAvatarSDK::BeatAvatarVisualController_HighlighterDelegate =>
    "BeatSaber.BeatAvatarSDK"."BeatAvatarVisualController/HighlighterDelegate"
);
#[cfg(
    feature = "BeatSaber+BeatAvatarSDK+BeatAvatarVisualController+HighlighterDelegate"
)]
impl std::ops::Deref
for crate::BeatSaber::BeatAvatarSDK::BeatAvatarVisualController_HighlighterDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+BeatAvatarSDK+BeatAvatarVisualController+HighlighterDelegate"
)]
impl std::ops::DerefMut
for crate::BeatSaber::BeatAvatarSDK::BeatAvatarVisualController_HighlighterDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "BeatSaber+BeatAvatarSDK+BeatAvatarVisualController+HighlighterDelegate"
)]
impl crate::BeatSaber::BeatAvatarSDK::BeatAvatarVisualController_HighlighterDelegate {
    pub fn BeginInvoke(
        &mut self,
        highlighted: bool,
        uvSegmentNumber: i32,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (highlighted, uvSegmentNumber, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        highlighted: bool,
        uvSegmentNumber: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (highlighted, uvSegmentNumber))?;
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
#[cfg(
    feature = "BeatSaber+BeatAvatarSDK+BeatAvatarVisualController+HighlighterDelegate"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::BeatAvatarSDK::BeatAvatarVisualController_HighlighterDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
