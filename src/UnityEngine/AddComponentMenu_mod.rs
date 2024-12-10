#[cfg(feature = "UnityEngine+AddComponentMenu")]
#[repr(C)]
#[derive(Debug)]
pub struct AddComponentMenu {
    __cordl_parent: crate::System::Attribute,
    pub m_AddComponentMenu: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_Ordering: i32,
}
#[cfg(feature = "UnityEngine+AddComponentMenu")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::AddComponentMenu => "UnityEngine"
    ."AddComponentMenu"
);
#[cfg(feature = "UnityEngine+AddComponentMenu")]
impl std::ops::Deref for crate::UnityEngine::AddComponentMenu {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddComponentMenu")]
impl std::ops::DerefMut for crate::UnityEngine::AddComponentMenu {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+AddComponentMenu")]
impl crate::UnityEngine::AddComponentMenu {
    pub fn New_Il2CppString0(
        menuName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (menuName))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_1(
        menuName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        order: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (menuName, order))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Il2CppString0(
        &mut self,
        menuName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (menuName))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        menuName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        order: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (menuName, order))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+AddComponentMenu")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::AddComponentMenu {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
