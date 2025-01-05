#[cfg(feature = "StyledUITemplates+ColorOverrideOption")]
#[repr(C)]
#[derive(Debug)]
pub struct ColorOverrideOption {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _override: bool,
    pub _useFixedColor: bool,
    pub _fixedColor: crate::UnityEngine::Color,
    pub _scriptableObjectColor: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ColorSO,
    >,
}
#[cfg(feature = "StyledUITemplates+ColorOverrideOption")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::StyledUITemplates::ColorOverrideOption =>
    "StyledUITemplates"."ColorOverrideOption"
);
#[cfg(feature = "StyledUITemplates+ColorOverrideOption")]
impl std::ops::Deref for crate::StyledUITemplates::ColorOverrideOption {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "StyledUITemplates+ColorOverrideOption")]
impl std::ops::DerefMut for crate::StyledUITemplates::ColorOverrideOption {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "StyledUITemplates+ColorOverrideOption")]
impl crate::StyledUITemplates::ColorOverrideOption {
    pub fn Equals_ColorOverrideOption0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::StyledUITemplates::ColorOverrideOption>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject1(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_ColorOverrideOption1(
        other: quest_hook::libil2cpp::Gc<crate::StyledUITemplates::ColorOverrideOption>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (other))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ColorOverrideOption1(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::StyledUITemplates::ColorOverrideOption>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fixedColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_fixedColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_overrideEnabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_overrideEnabled", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scriptableObjectColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO> = __cordl_object
            .invoke("get_scriptableObjectColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useFixedColor(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_useFixedColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_fixedColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_fixedColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_overrideEnabled(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_overrideEnabled", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_scriptableObjectColor(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::ColorSO>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_scriptableObjectColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_useFixedColor(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_useFixedColor", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "StyledUITemplates+ColorOverrideOption")]
impl quest_hook::libil2cpp::ObjectType
for crate::StyledUITemplates::ColorOverrideOption {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "StyledUITemplates+ColorOverrideOption")]
impl AsRef<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::StyledUITemplates::ColorOverrideOption>,
    >,
> for crate::StyledUITemplates::ColorOverrideOption {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::StyledUITemplates::ColorOverrideOption>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "StyledUITemplates+ColorOverrideOption")]
impl AsMut<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::StyledUITemplates::ColorOverrideOption>,
    >,
> for crate::StyledUITemplates::ColorOverrideOption {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::StyledUITemplates::ColorOverrideOption>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
