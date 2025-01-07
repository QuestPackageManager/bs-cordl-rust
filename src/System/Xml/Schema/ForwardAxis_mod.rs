#[cfg(feature = "System+Xml+Schema+ForwardAxis")]
#[repr(C)]
#[derive(Debug)]
pub struct ForwardAxis {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _topNode: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DoubleLinkAxis>,
    pub _rootNode: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DoubleLinkAxis>,
    pub _isAttribute: bool,
    pub _isDss: bool,
    pub _isSelfAxis: bool,
}
#[cfg(feature = "System+Xml+Schema+ForwardAxis")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::Schema::ForwardAxis {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "ForwardAxis";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Xml+Schema+ForwardAxis")]
impl std::ops::Deref for crate::System::Xml::Schema::ForwardAxis {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ForwardAxis")]
impl std::ops::DerefMut for crate::System::Xml::Schema::ForwardAxis {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+ForwardAxis")]
impl crate::System::Xml::Schema::ForwardAxis {
    pub fn New(
        axis: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DoubleLinkAxis>,
        isdesorself: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (axis, isdesorself))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        axis: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DoubleLinkAxis>,
        isdesorself: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (axis, isdesorself))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsAttribute(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDss(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDss", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSelfAxis(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsSelfAxis", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RootNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DoubleLinkAxis>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::DoubleLinkAxis,
        > = __cordl_object.invoke("get_RootNode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TopNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DoubleLinkAxis>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::DoubleLinkAxis,
        > = __cordl_object.invoke("get_TopNode", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+ForwardAxis")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::ForwardAxis {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
