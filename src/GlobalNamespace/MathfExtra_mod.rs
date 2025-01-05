#[cfg(feature = "MathfExtra")]
#[repr(C)]
#[derive(Debug)]
pub struct MathfExtra {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "MathfExtra")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MathfExtra => ""."MathfExtra"
);
#[cfg(feature = "MathfExtra")]
impl std::ops::Deref for crate::GlobalNamespace::MathfExtra {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MathfExtra")]
impl std::ops::DerefMut for crate::GlobalNamespace::MathfExtra {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MathfExtra")]
impl crate::GlobalNamespace::MathfExtra {
    pub fn Approximately(
        a: f32,
        b: f32,
        precision: f32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Approximately", (a, b, precision))?;
        Ok(__cordl_ret.into())
    }
    pub fn MaxAbs(a: f32, b: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MaxAbs", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Repeat(t: i32, length: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Repeat", (t, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn Round_Vector4_1(
        value: crate::UnityEngine::Vector4,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Round", (value, digits))?;
        Ok(__cordl_ret.into())
    }
    pub fn Round_f32_0(value: f32, decimals: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Round", (value, decimals))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShortestAngleDifference(
        from: f32,
        to: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShortestAngleDifference", (from, to))?;
        Ok(__cordl_ret.into())
    }
    pub fn _cordl_Mod_f32_f32_0(
        value: f32,
        _cordl_mod: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Mod", (value, _cordl_mod))?;
        Ok(__cordl_ret.into())
    }
    pub fn _cordl_Mod_i32_i32_1(
        value: i32,
        _cordl_mod: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Mod", (value, _cordl_mod))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MathfExtra")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MathfExtra {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
