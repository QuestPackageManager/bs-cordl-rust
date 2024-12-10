#[cfg(feature = "LightWithIdManager")]
#[repr(C)]
#[derive(Debug)]
pub struct LightWithIdManager {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub didChangeSomeColorsThisFrameEvent: *mut crate::System::Action,
    pub _lights: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::ILightWithId,
        >,
    >,
    pub _colors: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Nullable_1<crate::UnityEngine::Color>,
    >,
    pub _lightsToUnregister: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::ILightWithId,
    >,
    pub _didChangeSomeColorsThisFrame: bool,
}
#[cfg(feature = "LightWithIdManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightWithIdManager => ""
    ."LightWithIdManager"
);
#[cfg(feature = "LightWithIdManager")]
impl std::ops::Deref for crate::GlobalNamespace::LightWithIdManager {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightWithIdManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightWithIdManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightWithIdManager")]
impl crate::GlobalNamespace::LightWithIdManager {
    pub const kMaxLightId: i32 = 500i32;
    pub fn GetColorForId(
        &mut self,
        lightId: i32,
        initializeIfNull: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("GetColorForId", (lightId, initializeIfNull))?;
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RegisterLight(
        &mut self,
        lightWithId: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILightWithId>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterLight", (lightWithId))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColorForId(
        &mut self,
        lightId: i32,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColorForId", (lightId, color))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterLight(
        &mut self,
        lightWithId: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ILightWithId>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterLight", (lightWithId))?;
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
    pub fn add_didChangeSomeColorsThisFrameEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_didChangeSomeColorsThisFrameEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_didChangeSomeColorsThisFrameEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_didChangeSomeColorsThisFrameEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightWithIdManager")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LightWithIdManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
