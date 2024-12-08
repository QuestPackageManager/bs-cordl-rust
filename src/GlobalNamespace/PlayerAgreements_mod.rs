#[cfg(feature = "PlayerAgreements")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerAgreements {
    __cordl_parent: crate::System::Object,
    pub eulaVersion: i32,
    pub privacyPolicyVersion: i32,
    pub healthAndSafetyVersion: i32,
    pub playerSensitivityFlagVersion: i32,
    pub endOfLifeNoticeVersion: i32,
}
#[cfg(feature = "PlayerAgreements")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PlayerAgreements => ""."PlayerAgreements"
);
#[cfg(feature = "PlayerAgreements")]
impl std::ops::Deref for PlayerAgreements {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerAgreements")]
impl std::ops::DerefMut for PlayerAgreements {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerAgreements")]
impl PlayerAgreements {
    pub const kCurrentEndOfLifeNoticeVersion: i32 = 1i32;
    pub const kCurrentEulaVersion: i32 = 5i32;
    pub const kCurrentHealthAndSafetyVersion: i32 = 1i32;
    pub const kCurrentPlayerSensitivityFlagVersion: i32 = 3i32;
    pub const kCurrentPrivacyPolicyVersion: i32 = 5i32;
    pub const kFirstEulaVersion: i32 = 1i32;
    pub const kFirstPrivacyPolicyVersion: i32 = 1i32;
    pub fn AgreedToHealthAndSafety(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AgreedToHealthAndSafety", ())?;
        Ok(__cordl_ret)
    }
    pub fn AgreedToAnyPreviousHealthAndSafety(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AgreedToAnyPreviousHealthAndSafety", ())?;
        Ok(__cordl_ret)
    }
    pub fn AgreedToPlayerSenstivityFlag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AgreedToPlayerSenstivityFlag", ())?;
        Ok(__cordl_ret)
    }
    pub fn AgreedToAnyPreviousPrivacyPolicy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AgreedToAnyPreviousPrivacyPolicy", ())?;
        Ok(__cordl_ret)
    }
    pub fn AgreeToHealthAndSafety(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AgreeToHealthAndSafety", ())?;
        Ok(__cordl_ret)
    }
    pub fn AgreedToPrivacyPolicy(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AgreedToPrivacyPolicy", ())?;
        Ok(__cordl_ret)
    }
    pub fn AgreedToEndOfLifeNotice(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AgreedToEndOfLifeNotice", ())?;
        Ok(__cordl_ret)
    }
    pub fn AgreeToPlayerSensitivityFlag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AgreeToPlayerSensitivityFlag", ())?;
        Ok(__cordl_ret)
    }
    pub fn AgreedToPreviousPrivacyPolicy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AgreedToPreviousPrivacyPolicy", ())?;
        Ok(__cordl_ret)
    }
    pub fn AgreeToEula(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AgreeToEula", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_i32_i32_i32_1(
        &mut self,
        eulaVersion: i32,
        privacyPolicyVersion: i32,
        healthAndSafetyVersion: i32,
        playerSensitivityFlagVersion: i32,
        endOfLifeNoticeVersion: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    eulaVersion,
                    privacyPolicyVersion,
                    healthAndSafetyVersion,
                    playerSensitivityFlagVersion,
                    endOfLifeNoticeVersion,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AgreedToAnyPreviousEula(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AgreedToAnyPreviousEula", ())?;
        Ok(__cordl_ret)
    }
    pub fn AgreeToPrivacyPolicy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AgreeToPrivacyPolicy", ())?;
        Ok(__cordl_ret)
    }
    pub fn AgreeToEndOfLifeNotice(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AgreeToEndOfLifeNotice", ())?;
        Ok(__cordl_ret)
    }
    pub fn AgreedToAnyPreviousPlayerSensitivityFlag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("AgreedToAnyPreviousPlayerSensitivityFlag", ())?;
        Ok(__cordl_ret)
    }
    pub fn AgreedToEula(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("AgreedToEula", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_i32_i32_i32_1(
        eulaVersion: i32,
        privacyPolicyVersion: i32,
        healthAndSafetyVersion: i32,
        playerSensitivityFlagVersion: i32,
        endOfLifeNoticeVersion: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    eulaVersion,
                    privacyPolicyVersion,
                    healthAndSafetyVersion,
                    playerSensitivityFlagVersion,
                    endOfLifeNoticeVersion,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayerAgreements")]
impl quest_hook::libil2cpp::ObjectType for PlayerAgreements {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
