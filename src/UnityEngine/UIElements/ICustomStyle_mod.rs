#[cfg(feature = "UnityEngine+UIElements+ICustomStyle")]
#[repr(C)]
#[derive(Debug)]
pub struct ICustomStyle {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+ICustomStyle")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::ICustomStyle {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "ICustomStyle";
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn TryGetValue_CustomStyleProperty_1_ByRefMut3(
        &mut self,
        property: crate::UnityEngine::UIElements::CustomStyleProperty_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
        value: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetValue", (property, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetValue_CustomStyleProperty_1_ByRefMut4(
        &mut self,
        property: crate::UnityEngine::UIElements::CustomStyleProperty_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        >,
        value: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Sprite>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetValue", (property, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetValue_CustomStyleProperty_1_ByRefMut5(
        &mut self,
        property: crate::UnityEngine::UIElements::CustomStyleProperty_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VectorImage>,
        >,
        value: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::VectorImage>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetValue", (property, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetValue_CustomStyleProperty_1_ByRefMut6(
        &mut self,
        property: crate::UnityEngine::UIElements::CustomStyleProperty_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        value: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetValue", (property, value))?;
        Ok(__cordl_ret.into())
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
