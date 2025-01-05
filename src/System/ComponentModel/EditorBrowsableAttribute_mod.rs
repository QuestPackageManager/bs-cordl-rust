#[cfg(feature = "System+ComponentModel+EditorBrowsableAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct EditorBrowsableAttribute {
    __cordl_parent: crate::System::Attribute,
    pub browsableState: crate::System::ComponentModel::EditorBrowsableState,
}
#[cfg(feature = "System+ComponentModel+EditorBrowsableAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::EditorBrowsableAttribute
    => "System.ComponentModel"."EditorBrowsableAttribute"
);
#[cfg(feature = "System+ComponentModel+EditorBrowsableAttribute")]
impl std::ops::Deref for crate::System::ComponentModel::EditorBrowsableAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+EditorBrowsableAttribute")]
impl std::ops::DerefMut for crate::System::ComponentModel::EditorBrowsableAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+EditorBrowsableAttribute")]
impl crate::System::ComponentModel::EditorBrowsableAttribute {
    pub fn Equals(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        state: crate::System::ComponentModel::EditorBrowsableState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (state))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        state: crate::System::ComponentModel::EditorBrowsableState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (state))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ComponentModel+EditorBrowsableAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::EditorBrowsableAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
