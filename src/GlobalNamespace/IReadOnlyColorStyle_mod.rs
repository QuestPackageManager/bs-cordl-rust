#[cfg(feature = "IReadOnlyColorStyle")]
#[repr(C)]
#[derive(Debug)]
pub struct IReadOnlyColorStyle {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IReadOnlyColorStyle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IReadOnlyColorStyle => ""
    ."IReadOnlyColorStyle"
);
#[cfg(feature = "IReadOnlyColorStyle")]
impl std::ops::Deref for crate::GlobalNamespace::IReadOnlyColorStyle {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IReadOnlyColorStyle")]
impl std::ops::DerefMut for crate::GlobalNamespace::IReadOnlyColorStyle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IReadOnlyColorStyle")]
impl crate::GlobalNamespace::IReadOnlyColorStyle {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
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
    pub fn get_color0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_color1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color1", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_flipGradientColors(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_flipGradientColors", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_globalLightTintIntensity(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("get_globalLightTintIntensity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gradient(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_gradient", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_gradientDirection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::GradientDirection> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::GradientDirection = __cordl_object
            .invoke("get_gradientDirection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_useScriptableObjectColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_useScriptableObjectColor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IReadOnlyColorStyle")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IReadOnlyColorStyle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
