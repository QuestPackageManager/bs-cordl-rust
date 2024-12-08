#[cfg(feature = "UnityEngine+ProBuilder+HSVColor")]
#[repr(C)]
#[derive(Debug)]
pub struct HSVColor {
    __cordl_parent: crate::System::Object,
    pub h: f32,
    pub s: f32,
    pub v: f32,
}
#[cfg(feature = "UnityEngine+ProBuilder+HSVColor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::HSVColor =>
    "UnityEngine.ProBuilder"."HSVColor"
);
#[cfg(feature = "UnityEngine+ProBuilder+HSVColor")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::HSVColor {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+HSVColor")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::HSVColor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+HSVColor")]
impl crate::UnityEngine::ProBuilder::HSVColor {
    pub fn New_f32_1(
        h: f32,
        s: f32,
        v: f32,
        sv_modifier: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (h, s, v, sv_modifier))?;
        Ok(__cordl_object)
    }
    pub fn New_f32_f32_f32_0(
        h: f32,
        s: f32,
        v: f32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (h, s, v))?;
        Ok(__cordl_object)
    }
    pub fn SqrDistance(
        &mut self,
        InColor: *mut crate::UnityEngine::ProBuilder::HSVColor,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("SqrDistance", (InColor))?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_f32_1(
        &mut self,
        h: f32,
        s: f32,
        v: f32,
        sv_modifier: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (h, s, v, sv_modifier))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_f32_f32_f32_0(
        &mut self,
        h: f32,
        s: f32,
        v: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (h, s, v))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+HSVColor")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::HSVColor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
