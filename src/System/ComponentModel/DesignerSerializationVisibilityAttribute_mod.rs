#[cfg(feature = "System+ComponentModel+DesignerSerializationVisibilityAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DesignerSerializationVisibilityAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _Visibility_k__BackingField: crate::System::ComponentModel::DesignerSerializationVisibility,
}
#[cfg(feature = "System+ComponentModel+DesignerSerializationVisibilityAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::ComponentModel::DesignerSerializationVisibilityAttribute =>
    "System.ComponentModel"."DesignerSerializationVisibilityAttribute"
);
#[cfg(feature = "System+ComponentModel+DesignerSerializationVisibilityAttribute")]
impl std::ops::Deref
for crate::System::ComponentModel::DesignerSerializationVisibilityAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+DesignerSerializationVisibilityAttribute")]
impl std::ops::DerefMut
for crate::System::ComponentModel::DesignerSerializationVisibilityAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+DesignerSerializationVisibilityAttribute")]
impl crate::System::ComponentModel::DesignerSerializationVisibilityAttribute {
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
    pub fn IsDefaultAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsDefaultAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        visibility: crate::System::ComponentModel::DesignerSerializationVisibility,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (visibility))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        visibility: crate::System::ComponentModel::DesignerSerializationVisibility,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (visibility))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Visibility(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::ComponentModel::DesignerSerializationVisibility,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ComponentModel::DesignerSerializationVisibility = __cordl_object
            .invoke("get_Visibility", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ComponentModel+DesignerSerializationVisibilityAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::DesignerSerializationVisibilityAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
