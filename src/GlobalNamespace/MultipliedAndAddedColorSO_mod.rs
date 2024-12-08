#[cfg(feature = "MultipliedAndAddedColorSO")]
#[repr(C)]
#[derive(Debug)]
pub struct MultipliedAndAddedColorSO {
    __cordl_parent: crate::GlobalNamespace::ColorSO,
    pub _baseColor: *mut crate::GlobalNamespace::SimpleColorSO,
    pub _multiplierColor: crate::UnityEngine::Color,
    pub _addColor: crate::UnityEngine::Color,
}
#[cfg(feature = "MultipliedAndAddedColorSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultipliedAndAddedColorSO => ""
    ."MultipliedAndAddedColorSO"
);
#[cfg(feature = "MultipliedAndAddedColorSO")]
impl std::ops::Deref for crate::GlobalNamespace::MultipliedAndAddedColorSO {
    type Target = crate::GlobalNamespace::ColorSO;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultipliedAndAddedColorSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultipliedAndAddedColorSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultipliedAndAddedColorSO")]
impl crate::GlobalNamespace::MultipliedAndAddedColorSO {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
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
}
#[cfg(feature = "MultipliedAndAddedColorSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultipliedAndAddedColorSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
