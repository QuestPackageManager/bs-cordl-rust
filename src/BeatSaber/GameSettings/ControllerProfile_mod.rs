#[cfg(feature = "BeatSaber+GameSettings+ControllerProfile")]
#[repr(C)]
#[derive(Debug)]
pub struct ControllerProfile {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub localizationKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub index: i32,
    pub modifiable: bool,
    pub _alternativeHandling_k__BackingField: bool,
    pub _leftController_k__BackingField: crate::BeatSaber::GameSettings::Controller,
    pub _rightController_k__BackingField: crate::BeatSaber::GameSettings::Controller,
    pub _vrPlatformHelper: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IVRPlatformHelper,
    >,
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfile")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::GameSettings::ControllerProfile =>
    "BeatSaber.GameSettings"."ControllerProfile"
);
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfile")]
impl std::ops::Deref for crate::BeatSaber::GameSettings::ControllerProfile {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfile")]
impl std::ops::DerefMut for crate::BeatSaber::GameSettings::ControllerProfile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfile")]
impl crate::BeatSaber::GameSettings::ControllerProfile {
    pub fn Activate(
        &mut self,
        vrPlatformHelper: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IVRPlatformHelper,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Activate", (vrPlatformHelper))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyFromLeftToRight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyFromLeftToRight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyFromOtherControllerProfile(
        &mut self,
        other: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::GameSettings::ControllerProfile,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyFromOtherControllerProfile", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyFromRightToLeft(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyFromRightToLeft", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Deactivate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deactivate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FromSaveData(
        controllerProfileSaveData: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::GameSettings::ControllerProfileSaveData,
        >,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::BeatSaber::GameSettings::ControllerProfile>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::GameSettings::ControllerProfile,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromSaveData", (controllerProfileSaveData, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasDefaultValues(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasDefaultValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        localizationKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        modifiable: bool,
        alternativeHandling: bool,
        leftController: crate::BeatSaber::GameSettings::Controller,
        rightController: crate::BeatSaber::GameSettings::Controller,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    localizationKey,
                    index,
                    modifiable,
                    alternativeHandling,
                    leftController,
                    rightController,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn RefreshControllers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshControllers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetRotateThanMove(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRotateThanMove", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToSaveData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::BeatSaber::GameSettings::ControllerProfileSaveData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::GameSettings::ControllerProfileSaveData,
        > = __cordl_object.invoke("ToSaveData", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateControllerPosition(
        &mut self,
        isLeft: bool,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateControllerPosition", (isLeft, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateControllerRotation(
        &mut self,
        isLeft: bool,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateControllerRotation", (isLeft, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        localizationKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        index: i32,
        modifiable: bool,
        alternativeHandling: bool,
        leftController: crate::BeatSaber::GameSettings::Controller,
        rightController: crate::BeatSaber::GameSettings::Controller,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    localizationKey,
                    index,
                    modifiable,
                    alternativeHandling,
                    leftController,
                    rightController,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_alternativeHandling(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_alternativeHandling", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_leftController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::GameSettings::Controller> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::GameSettings::Controller = __cordl_object
            .invoke("get_leftController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rightController(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::GameSettings::Controller> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::GameSettings::Controller = __cordl_object
            .invoke("get_rightController", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_alternativeHandling(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_alternativeHandling", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_leftController(
        &mut self,
        value: crate::BeatSaber::GameSettings::Controller,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_leftController", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_rightController(
        &mut self,
        value: crate::BeatSaber::GameSettings::Controller,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_rightController", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+GameSettings+ControllerProfile")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::GameSettings::ControllerProfile {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
