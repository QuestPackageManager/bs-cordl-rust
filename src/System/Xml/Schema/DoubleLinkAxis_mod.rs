#[cfg(feature = "System+Xml+Schema+DoubleLinkAxis")]
#[repr(C)]
#[derive(Debug)]
pub struct DoubleLinkAxis {
    __cordl_parent: crate::MS::Internal::Xml::XPath::Axis,
    pub next: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>,
}
#[cfg(feature = "System+Xml+Schema+DoubleLinkAxis")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::DoubleLinkAxis =>
    "System.Xml.Schema"."DoubleLinkAxis"
);
#[cfg(feature = "System+Xml+Schema+DoubleLinkAxis")]
impl std::ops::Deref for crate::System::Xml::Schema::DoubleLinkAxis {
    type Target = crate::MS::Internal::Xml::XPath::Axis;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+DoubleLinkAxis")]
impl std::ops::DerefMut for crate::System::Xml::Schema::DoubleLinkAxis {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+DoubleLinkAxis")]
impl crate::System::Xml::Schema::DoubleLinkAxis {
    pub fn ConvertTree(
        axis: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DoubleLinkAxis>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::DoubleLinkAxis,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertTree", (axis))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        axis: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>,
        inputaxis: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DoubleLinkAxis>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (axis, inputaxis))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        axis: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>,
        inputaxis: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DoubleLinkAxis>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (axis, inputaxis))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Next(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::MS::Internal::Xml::XPath::Axis,
        > = __cordl_object.invoke("get_Next", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Next(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::MS::Internal::Xml::XPath::Axis>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Next", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+DoubleLinkAxis")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::DoubleLinkAxis {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
