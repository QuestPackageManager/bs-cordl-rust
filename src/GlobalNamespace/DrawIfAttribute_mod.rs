#[cfg(feature = "DrawIfAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DrawIfAttribute {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::PropertyAttribute>,
    pub propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub orValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub disablingType: crate::GlobalNamespace::DrawIfAttribute_DisablingType,
}
#[cfg(feature = "DrawIfAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DrawIfAttribute => ""
    ."DrawIfAttribute"
);
#[cfg(feature = "DrawIfAttribute")]
impl std::ops::Deref for crate::GlobalNamespace::DrawIfAttribute {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::PropertyAttribute>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DrawIfAttribute")]
impl std::ops::DerefMut for crate::GlobalNamespace::DrawIfAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DrawIfAttribute")]
impl crate::GlobalNamespace::DrawIfAttribute {
    #[cfg(feature = "DrawIfAttribute+DisablingType")]
    pub type DisablingType = crate::GlobalNamespace::DrawIfAttribute_DisablingType;
    pub fn New_DrawIfAttribute_DisablingType0(
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        disablingType: crate::GlobalNamespace::DrawIfAttribute_DisablingType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (propertyName, value, disablingType))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Gc_DrawIfAttribute_DisablingType1(
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        orValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        disablingType: crate::GlobalNamespace::DrawIfAttribute_DisablingType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (propertyName, value, orValue, disablingType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_DrawIfAttribute_DisablingType0(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        disablingType: crate::GlobalNamespace::DrawIfAttribute_DisablingType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (propertyName, value, disablingType))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Gc_DrawIfAttribute_DisablingType1(
        &mut self,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        orValue: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        disablingType: crate::GlobalNamespace::DrawIfAttribute_DisablingType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (propertyName, value, orValue, disablingType))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DrawIfAttribute")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::DrawIfAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "DrawIfAttribute+DisablingType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawIfAttribute_DisablingType {
    #[default]
    DontDraw = 1i32,
    ReadOnly = 0i32,
}
#[cfg(feature = "DrawIfAttribute+DisablingType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DrawIfAttribute_DisablingType
    => ""."DrawIfAttribute/DisablingType"
);
