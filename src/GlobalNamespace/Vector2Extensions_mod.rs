#[cfg(feature = "Vector2Extensions")]
#[repr(C)]
#[derive(Debug)]
pub struct Vector2Extensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Vector2Extensions")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::Vector2Extensions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "Vector2Extensions";
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
#[cfg(feature = "Vector2Extensions")]
impl std::ops::Deref for crate::GlobalNamespace::Vector2Extensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Vector2Extensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::Vector2Extensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Vector2Extensions")]
impl crate::GlobalNamespace::Vector2Extensions {
    pub fn Clamp_Rect1(
        value: crate::UnityEngine::Vector2,
        within: crate::UnityEngine::Rect,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Clamp", (value, within))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clamp_Vector2_Vector2_0(
        value: crate::UnityEngine::Vector2,
        min: crate::UnityEngine::Vector2,
        max: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Clamp", (value, min, max))?;
        Ok(__cordl_ret.into())
    }
    pub fn SignedAngleToLine(
        vec: crate::UnityEngine::Vector2,
        line: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SignedAngleToLine", (vec, line))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Vector2Extensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::Vector2Extensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
