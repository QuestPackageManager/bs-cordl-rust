#[cfg(feature = "System+Xml+Schema+AxisElement")]
#[repr(C)]
#[derive(Debug)]
pub struct AxisElement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub curNode: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DoubleLinkAxis>,
    pub rootDepth: i32,
    pub curDepth: i32,
    pub isMatch: bool,
}
#[cfg(feature = "System+Xml+Schema+AxisElement")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::Schema::AxisElement {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "AxisElement";
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
#[cfg(feature = "System+Xml+Schema+AxisElement")]
impl std::ops::Deref for crate::System::Xml::Schema::AxisElement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+AxisElement")]
impl std::ops::DerefMut for crate::System::Xml::Schema::AxisElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+AxisElement")]
impl crate::System::Xml::Schema::AxisElement {
    pub fn MoveToChild(
        &mut self,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        URN: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        depth: i32,
        parent: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ForwardAxis>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("MoveToChild", (name, URN, depth, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveToParent(
        &mut self,
        depth: i32,
        parent: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::ForwardAxis>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveToParent", (depth, parent))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DoubleLinkAxis>,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (node, depth))?;
        Ok(__cordl_object.into())
    }
    pub fn SetDepth(
        &mut self,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetDepth", (depth))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        node: quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DoubleLinkAxis>,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (node, depth))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CurNode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Xml::Schema::DoubleLinkAxis>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Xml::Schema::DoubleLinkAxis,
        > = __cordl_object.invoke("get_CurNode", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+AxisElement")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::AxisElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
