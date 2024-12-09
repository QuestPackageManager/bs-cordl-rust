#[cfg(feature = "SongProgressUIController")]
#[repr(C)]
#[derive(Debug)]
pub struct SongProgressUIController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _slider: *mut crate::UnityEngine::UI::Slider,
    pub _progressImage: *mut crate::UnityEngine::UI::Image,
    pub _durationMinutesText: *mut crate::TMPro::TextMeshProUGUI,
    pub _durationSecondsText: *mut crate::TMPro::TextMeshProUGUI,
    pub _progressMinutesText: *mut crate::TMPro::TextMeshProUGUI,
    pub _progressSecondsText: *mut crate::TMPro::TextMeshProUGUI,
    pub _durationRectTransform: *mut crate::UnityEngine::RectTransform,
    pub _durationTextOffsetShort: f32,
    pub _durationTextOffsetLong: f32,
    pub _audioTimeSource: *mut crate::GlobalNamespace::IAudioTimeSource,
    pub _songController: *mut crate::GlobalNamespace::SongController,
    pub _prevMinutes: i32,
    pub _prevSeconds: i32,
    pub _stringBuilder: *mut crate::System::Text::StringBuilder,
    pub _progressImageRectTransform: *mut crate::UnityEngine::RectTransform,
    pub _songLength: f32,
    pub _progressMinutesFormatString: *mut quest_hook::libil2cpp::Il2CppString,
    pub _songDidFinish: bool,
}
#[cfg(feature = "SongProgressUIController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SongProgressUIController => ""
    ."SongProgressUIController"
);
#[cfg(feature = "SongProgressUIController")]
impl std::ops::Deref for crate::GlobalNamespace::SongProgressUIController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SongProgressUIController")]
impl std::ops::DerefMut for crate::GlobalNamespace::SongProgressUIController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SongProgressUIController")]
impl crate::GlobalNamespace::SongProgressUIController {
    pub fn HandleSongDidFinish(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleSongDidFinish", ())?;
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
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
    pub fn UpdateSongProgressUIElements(
        &mut self,
        minutes: i32,
        seconds: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateSongProgressUIElements", (minutes, seconds))?;
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
#[cfg(feature = "SongProgressUIController")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SongProgressUIController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
