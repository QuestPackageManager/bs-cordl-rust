#[cfg(feature = "UnityEngine+Timeline+TrackColorAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct TrackColorAttribute {
    __cordl_parent: crate::System::Attribute,
    pub m_Color: crate::UnityEngine::Color,
}
#[cfg(feature = "UnityEngine+Timeline+TrackColorAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::TrackColorAttribute =>
    "UnityEngine.Timeline"."TrackColorAttribute"
);
#[cfg(feature = "UnityEngine+Timeline+TrackColorAttribute")]
impl std::ops::Deref for crate::UnityEngine::Timeline::TrackColorAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackColorAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::TrackColorAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackColorAttribute")]
impl crate::UnityEngine::Timeline::TrackColorAttribute {
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        r: f32,
        g: f32,
        b: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (r, g, b))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        r: f32,
        g: f32,
        b: f32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (r, g, b))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+Timeline+TrackColorAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Timeline::TrackColorAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
