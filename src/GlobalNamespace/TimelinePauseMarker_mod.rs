#[cfg(feature = "TimelinePauseMarker")]
#[repr(C)]
#[derive(Debug)]
pub struct TimelinePauseMarker {
    __cordl_parent: crate::UnityEngine::Timeline::Marker,
    pub _id_k__BackingField: crate::UnityEngine::PropertyName,
}
#[cfg(feature = "TimelinePauseMarker")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::TimelinePauseMarker {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "TimelinePauseMarker";
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
#[cfg(feature = "TimelinePauseMarker")]
impl std::ops::Deref for crate::GlobalNamespace::TimelinePauseMarker {
    type Target = crate::UnityEngine::Timeline::Marker;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TimelinePauseMarker")]
impl std::ops::DerefMut for crate::GlobalNamespace::TimelinePauseMarker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TimelinePauseMarker")]
impl crate::GlobalNamespace::TimelinePauseMarker {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::PropertyName> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::PropertyName = __cordl_object
            .invoke("get_id", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TimelinePauseMarker")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::TimelinePauseMarker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "TimelinePauseMarker")]
impl AsRef<crate::UnityEngine::Playables::INotification>
for crate::GlobalNamespace::TimelinePauseMarker {
    fn as_ref(&self) -> &crate::UnityEngine::Playables::INotification {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "TimelinePauseMarker")]
impl AsMut<crate::UnityEngine::Playables::INotification>
for crate::GlobalNamespace::TimelinePauseMarker {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::Playables::INotification {
        unsafe { std::mem::transmute(self) }
    }
}
