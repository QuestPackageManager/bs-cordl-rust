#[cfg(feature = "UnityEngine+UIElements+ICustomStyle")]
#[repr(C)]
#[derive(Debug)]
pub struct ICustomStyle {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+ICustomStyle")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::ICustomStyle =>
    "UnityEngine.UIElements"."ICustomStyle"
);
#[cfg(feature = "UnityEngine+UIElements+ICustomStyle")]
impl std::ops::Deref for crate::UnityEngine::UIElements::ICustomStyle {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ICustomStyle")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::ICustomStyle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ICustomStyle")]
impl crate::UnityEngine::UIElements::ICustomStyle {
    pub fn TryGetValue_CustomStyleProperty_1_ByRefMut0(
        &mut self,
        property: crate::UnityEngine::UIElements::CustomStyleProperty_1<f32>,
        value: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetValue", (property, value))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetValue_CustomStyleProperty_1_ByRefMut1(
        &mut self,
        property: crate::UnityEngine::UIElements::CustomStyleProperty_1<i32>,
        value: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetValue", (property, value))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetValue_CustomStyleProperty_1_ByRefMut2(
        &mut self,
        property: crate::UnityEngine::UIElements::CustomStyleProperty_1<
            crate::UnityEngine::Color,
        >,
        value: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Color>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetValue", (property, value))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetValue_CustomStyleProperty_1_ByRefMut3(
        &mut self,
        property: crate::UnityEngine::UIElements::CustomStyleProperty_1<
            *mut crate::UnityEngine::Texture2D,
        >,
        value: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::Texture2D>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetValue", (property, value))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetValue_CustomStyleProperty_1_ByRefMut4(
        &mut self,
        property: crate::UnityEngine::UIElements::CustomStyleProperty_1<
            *mut crate::UnityEngine::Sprite,
        >,
        value: quest_hook::libil2cpp::ByRefMut<*mut crate::UnityEngine::Sprite>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetValue", (property, value))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetValue_CustomStyleProperty_1_ByRefMut5(
        &mut self,
        property: crate::UnityEngine::UIElements::CustomStyleProperty_1<
            *mut crate::UnityEngine::UIElements::VectorImage,
        >,
        value: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::UIElements::VectorImage,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetValue", (property, value))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetValue_CustomStyleProperty_1_ByRefMut6(
        &mut self,
        property: crate::UnityEngine::UIElements::CustomStyleProperty_1<
            *mut crate::System::String,
        >,
        value: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetValue", (property, value))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+ICustomStyle")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::UIElements::ICustomStyle {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
