#[cfg(feature = "LightColorGroupParent")]
#[repr(C)]
#[derive(Debug)]
pub struct LightColorGroupParent {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
}
#[cfg(feature = "LightColorGroupParent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LightColorGroupParent => ""
    ."LightColorGroupParent"
);
#[cfg(feature = "LightColorGroupParent")]
impl std::ops::Deref for crate::GlobalNamespace::LightColorGroupParent {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LightColorGroupParent")]
impl std::ops::DerefMut for crate::GlobalNamespace::LightColorGroupParent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LightColorGroupParent")]
impl crate::GlobalNamespace::LightColorGroupParent {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LightColorGroupParent")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LightColorGroupParent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "LightColorGroupParent")]
impl AsRef<crate::GlobalNamespace::IEditTimeValidated>
for crate::GlobalNamespace::LightColorGroupParent {
    fn as_ref(&self) -> &crate::GlobalNamespace::IEditTimeValidated {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "LightColorGroupParent")]
impl AsMut<crate::GlobalNamespace::IEditTimeValidated>
for crate::GlobalNamespace::LightColorGroupParent {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IEditTimeValidated {
        unsafe { std::mem::transmute(self) }
    }
}
