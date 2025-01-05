#[cfg(feature = "System+ComponentModel+ListChangedEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct ListChangedEventArgs {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::EventArgs>,
    pub _ListChangedType_k__BackingField: crate::System::ComponentModel::ListChangedType,
    pub _NewIndex_k__BackingField: i32,
    pub _OldIndex_k__BackingField: i32,
    pub _PropertyDescriptor_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::ComponentModel::PropertyDescriptor,
    >,
}
#[cfg(feature = "System+ComponentModel+ListChangedEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ComponentModel::ListChangedEventArgs =>
    "System.ComponentModel"."ListChangedEventArgs"
);
#[cfg(feature = "System+ComponentModel+ListChangedEventArgs")]
impl std::ops::Deref for crate::System::ComponentModel::ListChangedEventArgs {
    type Target = quest_hook::libil2cpp::Gc<crate::System::EventArgs>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ListChangedEventArgs")]
impl std::ops::DerefMut for crate::System::ComponentModel::ListChangedEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ComponentModel+ListChangedEventArgs")]
impl crate::System::ComponentModel::ListChangedEventArgs {
    pub fn New_Gc2(
        listChangedType: crate::System::ComponentModel::ListChangedType,
        propDesc: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (listChangedType, propDesc))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_0(
        listChangedType: crate::System::ComponentModel::ListChangedType,
        newIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (listChangedType, newIndex))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_Gc1(
        listChangedType: crate::System::ComponentModel::ListChangedType,
        newIndex: i32,
        propDesc: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (listChangedType, newIndex, propDesc))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_i32_3(
        listChangedType: crate::System::ComponentModel::ListChangedType,
        newIndex: i32,
        oldIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (listChangedType, newIndex, oldIndex))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Gc2(
        &mut self,
        listChangedType: crate::System::ComponentModel::ListChangedType,
        propDesc: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (listChangedType, propDesc))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_0(
        &mut self,
        listChangedType: crate::System::ComponentModel::ListChangedType,
        newIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (listChangedType, newIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_Gc1(
        &mut self,
        listChangedType: crate::System::ComponentModel::ListChangedType,
        newIndex: i32,
        propDesc: quest_hook::libil2cpp::Gc<
            crate::System::ComponentModel::PropertyDescriptor,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (listChangedType, newIndex, propDesc))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_3(
        &mut self,
        listChangedType: crate::System::ComponentModel::ListChangedType,
        newIndex: i32,
        oldIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (listChangedType, newIndex, oldIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ListChangedType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::ComponentModel::ListChangedType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ComponentModel::ListChangedType = __cordl_object
            .invoke("get_ListChangedType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_NewIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_NewIndex", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OldIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_OldIndex", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+ComponentModel+ListChangedEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::ComponentModel::ListChangedEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
