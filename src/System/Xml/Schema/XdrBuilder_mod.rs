#[cfg(feature = "System+Xml+Schema+XdrBuilder+AttributeContent")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder_AttributeContent {
    __cordl_parent: crate::System::Object,
    pub _AttDef: *mut crate::System::Xml::Schema::SchemaAttDef,
    pub _Name: *mut crate::System::Xml::XmlQualifiedName,
    pub _Prefix: *mut crate::System::String,
    pub _Required: bool,
    pub _MinVal: u32,
    pub _MaxVal: u32,
    pub _MaxLength: u32,
    pub _MinLength: u32,
    pub _EnumerationRequired: bool,
    pub _HasDataType: bool,
    pub _Global: bool,
    pub _Default: *mut crate::System::Object,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+AttributeContent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XdrBuilder_AttributeContent
    => "System.Xml.Schema"."XdrBuilder/AttributeContent"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder+AttributeContent")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder_AttributeContent {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+AttributeContent")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XdrBuilder_AttributeContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+AttributeContent")]
impl crate::System::Xml::Schema::XdrBuilder_AttributeContent {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+AttributeContent")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XdrBuilder_AttributeContent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+DeclBaseInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder_DeclBaseInfo {
    __cordl_parent: crate::System::Object,
    pub _Name: *mut crate::System::Xml::XmlQualifiedName,
    pub _Prefix: *mut crate::System::String,
    pub _TypeName: *mut crate::System::Xml::XmlQualifiedName,
    pub _TypePrefix: *mut crate::System::String,
    pub _Default: *mut crate::System::Object,
    pub _Revises: *mut crate::System::Object,
    pub _MaxOccurs: u32,
    pub _MinOccurs: u32,
    pub _Checking: bool,
    pub _ElementDecl: *mut crate::System::Xml::Schema::SchemaElementDecl,
    pub _Attdef: *mut crate::System::Xml::Schema::SchemaAttDef,
    pub _Next: *mut crate::System::Xml::Schema::XdrBuilder_DeclBaseInfo,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+DeclBaseInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XdrBuilder_DeclBaseInfo =>
    "System.Xml.Schema"."XdrBuilder/DeclBaseInfo"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder+DeclBaseInfo")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder_DeclBaseInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+DeclBaseInfo")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XdrBuilder_DeclBaseInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+DeclBaseInfo")]
impl crate::System::Xml::Schema::XdrBuilder_DeclBaseInfo {
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
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+DeclBaseInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XdrBuilder_DeclBaseInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+ElementContent")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder_ElementContent {
    __cordl_parent: crate::System::Object,
    pub _ElementDecl: *mut crate::System::Xml::Schema::SchemaElementDecl,
    pub _ContentAttr: i32,
    pub _OrderAttr: i32,
    pub _MasterGroupRequired: bool,
    pub _ExistTerminal: bool,
    pub _AllowDataType: bool,
    pub _HasDataType: bool,
    pub _HasType: bool,
    pub _EnumerationRequired: bool,
    pub _MinVal: u32,
    pub _MaxVal: u32,
    pub _MaxLength: u32,
    pub _MinLength: u32,
    pub _AttDefList: *mut crate::System::Collections::Hashtable,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+ElementContent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XdrBuilder_ElementContent
    => "System.Xml.Schema"."XdrBuilder/ElementContent"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder+ElementContent")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder_ElementContent {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+ElementContent")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XdrBuilder_ElementContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+ElementContent")]
impl crate::System::Xml::Schema::XdrBuilder_ElementContent {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+ElementContent")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XdrBuilder_ElementContent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+GroupContent")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder_GroupContent {
    __cordl_parent: crate::System::Object,
    pub _MinVal: u32,
    pub _MaxVal: u32,
    pub _HasMaxAttr: bool,
    pub _HasMinAttr: bool,
    pub _Order: i32,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+GroupContent")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XdrBuilder_GroupContent =>
    "System.Xml.Schema"."XdrBuilder/GroupContent"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder+GroupContent")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder_GroupContent {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+GroupContent")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XdrBuilder_GroupContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+GroupContent")]
impl crate::System::Xml::Schema::XdrBuilder_GroupContent {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+GroupContent")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XdrBuilder_GroupContent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrAttributeEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder_XdrAttributeEntry {
    __cordl_parent: crate::System::Object,
    pub _Attribute: crate::System::Xml::Schema::SchemaNames_Token,
    pub _SchemaFlags: i32,
    pub _Datatype: *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    pub _BuildFunc: *mut crate::System::Xml::Schema::XdrBuilder_XdrBuildFunction,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrAttributeEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::XdrBuilder_XdrAttributeEntry => "System.Xml.Schema"
    ."XdrBuilder/XdrAttributeEntry"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrAttributeEntry")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder_XdrAttributeEntry {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrAttributeEntry")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XdrBuilder_XdrAttributeEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrAttributeEntry")]
impl crate::System::Xml::Schema::XdrBuilder_XdrAttributeEntry {
    pub fn _ctor_XdrBuilder_XdrBuildFunction0(
        &mut self,
        a: crate::System::Xml::Schema::SchemaNames_Token,
        ttype: crate::System::Xml::XmlTokenizedType,
        build: *mut crate::System::Xml::Schema::XdrBuilder_XdrBuildFunction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (a, ttype, build))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_XdrBuilder_XdrBuildFunction1(
        &mut self,
        a: crate::System::Xml::Schema::SchemaNames_Token,
        ttype: crate::System::Xml::XmlTokenizedType,
        schemaFlags: i32,
        build: *mut crate::System::Xml::Schema::XdrBuilder_XdrBuildFunction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (a, ttype, schemaFlags, build))?;
        Ok(__cordl_ret)
    }
    pub fn New_XdrBuilder_XdrBuildFunction0(
        a: crate::System::Xml::Schema::SchemaNames_Token,
        ttype: crate::System::Xml::XmlTokenizedType,
        build: *mut crate::System::Xml::Schema::XdrBuilder_XdrBuildFunction,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (a, ttype, build))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_XdrBuilder_XdrBuildFunction1(
        a: crate::System::Xml::Schema::SchemaNames_Token,
        ttype: crate::System::Xml::XmlTokenizedType,
        schemaFlags: i32,
        build: *mut crate::System::Xml::Schema::XdrBuilder_XdrBuildFunction,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (a, ttype, schemaFlags, build))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrAttributeEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XdrBuilder_XdrAttributeEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBeginChildFunction")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder_XdrBeginChildFunction {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBeginChildFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::XdrBuilder_XdrBeginChildFunction => "System.Xml.Schema"
    ."XdrBuilder/XdrBeginChildFunction"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBeginChildFunction")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder_XdrBeginChildFunction {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBeginChildFunction")]
impl std::ops::DerefMut
for crate::System::Xml::Schema::XdrBuilder_XdrBeginChildFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBeginChildFunction")]
impl crate::System::Xml::Schema::XdrBuilder_XdrBeginChildFunction {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        builder: *mut crate::System::Xml::Schema::XdrBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (builder))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBeginChildFunction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XdrBuilder_XdrBeginChildFunction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBuildFunction")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder_XdrBuildFunction {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBuildFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XdrBuilder_XdrBuildFunction
    => "System.Xml.Schema"."XdrBuilder/XdrBuildFunction"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBuildFunction")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder_XdrBuildFunction {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBuildFunction")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XdrBuilder_XdrBuildFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBuildFunction")]
impl crate::System::Xml::Schema::XdrBuilder_XdrBuildFunction {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        builder: *mut crate::System::Xml::Schema::XdrBuilder,
        obj: *mut crate::System::Object,
        prefix: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (builder, obj, prefix))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBuildFunction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XdrBuilder_XdrBuildFunction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder {
    __cordl_parent: crate::System::Xml::Schema::SchemaBuilder,
    pub _SchemaInfo: *mut crate::System::Xml::Schema::SchemaInfo,
    pub _TargetNamespace: *mut crate::System::String,
    pub _reader: *mut crate::System::Xml::XmlReader,
    pub positionInfo: *mut crate::System::Xml::PositionInfo,
    pub _contentValidator: *mut crate::System::Xml::Schema::ParticleContentValidator,
    pub _CurState: *mut crate::System::Xml::Schema::XdrBuilder_XdrEntry,
    pub _NextState: *mut crate::System::Xml::Schema::XdrBuilder_XdrEntry,
    pub _StateHistory: *mut crate::System::Xml::HWStack,
    pub _GroupStack: *mut crate::System::Xml::HWStack,
    pub _XdrName: *mut crate::System::String,
    pub _XdrPrefix: *mut crate::System::String,
    pub _ElementDef: *mut crate::System::Xml::Schema::XdrBuilder_ElementContent,
    pub _GroupDef: *mut crate::System::Xml::Schema::XdrBuilder_GroupContent,
    pub _AttributeDef: *mut crate::System::Xml::Schema::XdrBuilder_AttributeContent,
    pub _UndefinedAttributeTypes: *mut crate::System::Xml::Schema::XdrBuilder_DeclBaseInfo,
    pub _BaseDecl: *mut crate::System::Xml::Schema::XdrBuilder_DeclBaseInfo,
    pub _NameTable: *mut crate::System::Xml::XmlNameTable,
    pub _SchemaNames: *mut crate::System::Xml::Schema::SchemaNames,
    pub _CurNsMgr: *mut crate::System::Xml::XmlNamespaceManager,
    pub _Text: *mut crate::System::String,
    pub validationEventHandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
    pub _UndeclaredElements: *mut crate::System::Collections::Hashtable,
    pub xmlResolver: *mut crate::System::Xml::XmlResolver,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XdrBuilder =>
    "System.Xml.Schema"."XdrBuilder"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder {
    type Target = crate::System::Xml::Schema::SchemaBuilder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XdrBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder")]
impl crate::System::Xml::Schema::XdrBuilder {
    #[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrAttributeEntry")]
    pub type XdrAttributeEntry = crate::System::Xml::Schema::XdrBuilder_XdrAttributeEntry;
    #[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBuildFunction")]
    pub type XdrBuildFunction = crate::System::Xml::Schema::XdrBuilder_XdrBuildFunction;
    #[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEntry")]
    pub type XdrEntry = crate::System::Xml::Schema::XdrBuilder_XdrEntry;
    #[cfg(feature = "System+Xml+Schema+XdrBuilder+ElementContent")]
    pub type ElementContent = crate::System::Xml::Schema::XdrBuilder_ElementContent;
    #[cfg(feature = "System+Xml+Schema+XdrBuilder+DeclBaseInfo")]
    pub type DeclBaseInfo = crate::System::Xml::Schema::XdrBuilder_DeclBaseInfo;
    #[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEndChildFunction")]
    pub type XdrEndChildFunction = crate::System::Xml::Schema::XdrBuilder_XdrEndChildFunction;
    #[cfg(feature = "System+Xml+Schema+XdrBuilder+AttributeContent")]
    pub type AttributeContent = crate::System::Xml::Schema::XdrBuilder_AttributeContent;
    #[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrInitFunction")]
    pub type XdrInitFunction = crate::System::Xml::Schema::XdrBuilder_XdrInitFunction;
    #[cfg(feature = "System+Xml+Schema+XdrBuilder+GroupContent")]
    pub type GroupContent = crate::System::Xml::Schema::XdrBuilder_GroupContent;
    #[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrBeginChildFunction")]
    pub type XdrBeginChildFunction = crate::System::Xml::Schema::XdrBuilder_XdrBeginChildFunction;
    pub fn IsSkipableAttribute(
        &mut self,
        qname: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSkipableAttribute", (qname))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessCData(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessCData", (value))?;
        Ok(__cordl_ret)
    }
    pub fn LoadSchema(
        &mut self,
        uri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("LoadSchema", (uri))?;
        Ok(__cordl_ret)
    }
    pub fn XDR_CheckAttributeDefault(
        &mut self,
        decl: *mut crate::System::Xml::Schema::XdrBuilder_DeclBaseInfo,
        pAttdef: *mut crate::System::Xml::Schema::SchemaAttDef,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("XDR_CheckAttributeDefault", (decl, pAttdef))?;
        Ok(__cordl_ret)
    }
    pub fn set_XmlResolver(
        &mut self,
        value: *mut crate::System::Xml::XmlResolver,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_XmlResolver", (value))?;
        Ok(__cordl_ret)
    }
    pub fn CheckDatatype(
        &mut self,
        str: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaDatatype,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaDatatype = __cordl_object
            .invoke("CheckDatatype", (str))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_String_Il2CppArray_XmlSeverityType0(
        &mut self,
        code: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        severity: crate::System::Xml::Schema::XmlSeverityType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, args, severity))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_String1(
        &mut self,
        code: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_String_String2(
        &mut self,
        code: *mut crate::System::String,
        msg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (code, msg))?;
        Ok(__cordl_ret)
    }
    pub fn SendValidationEvent_XmlSchemaException_XmlSeverityType3(
        &mut self,
        e: *mut crate::System::Xml::Schema::XmlSchemaException,
        severity: crate::System::Xml::Schema::XmlSeverityType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendValidationEvent", (e, severity))?;
        Ok(__cordl_ret)
    }
    pub fn AddOrder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddOrder", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetContent(
        &mut self,
        qname: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetContent", (qname))?;
        Ok(__cordl_ret)
    }
    pub fn CheckDefaultAttValue(
        &mut self,
        attDef: *mut crate::System::Xml::Schema::SchemaAttDef,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckDefaultAttValue", (attDef))?;
        Ok(__cordl_ret)
    }
    pub fn IsGlobal(&mut self, flags: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsGlobal", (flags))?;
        Ok(__cordl_ret)
    }
    pub fn PushGroupInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PushGroupInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsContentParsed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsContentParsed", ())?;
        Ok(__cordl_ret)
    }
    pub fn StartChildren(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartChildren", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessElement(
        &mut self,
        prefix: *mut crate::System::String,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ProcessElement", (prefix, name, ns))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        reader: *mut crate::System::Xml::XmlReader,
        curmgr: *mut crate::System::Xml::XmlNamespaceManager,
        sinfo: *mut crate::System::Xml::Schema::SchemaInfo,
        targetNamspace: *mut crate::System::String,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        schemaNames: *mut crate::System::Xml::Schema::SchemaNames,
        eventhandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    reader,
                    curmgr,
                    sinfo,
                    targetNamspace,
                    nameTable,
                    schemaNames,
                    eventhandler,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn EndChildren(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndChildren", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsSkipableElement(
        &mut self,
        qname: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsSkipableElement", (qname))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessMarkup(
        &mut self,
        markup: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Xml::XmlNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessMarkup", (markup))?;
        Ok(__cordl_ret)
    }
    pub fn GetOrder(
        &mut self,
        qname: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetOrder", (qname))?;
        Ok(__cordl_ret)
    }
    pub fn SetAttributePresence(
        &mut self,
        pAttdef: *mut crate::System::Xml::Schema::SchemaAttDef,
        fRequired: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAttributePresence", (pAttdef, fRequired))?;
        Ok(__cordl_ret)
    }
    pub fn GetNextState(
        &mut self,
        qname: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetNextState", (qname))?;
        Ok(__cordl_ret)
    }
    pub fn GetModel(
        &mut self,
        qname: *mut crate::System::Xml::XmlQualifiedName,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetModel", (qname))?;
        Ok(__cordl_ret)
    }
    pub fn PopGroupInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopGroupInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessAttribute(
        &mut self,
        prefix: *mut crate::System::String,
        name: *mut crate::System::String,
        ns: *mut crate::System::String,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessAttribute", (prefix, name, ns, value))?;
        Ok(__cordl_ret)
    }
    pub fn Pop(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object.invoke("Pop", ())?;
        Ok(__cordl_ret)
    }
    pub fn Push(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Push", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        reader: *mut crate::System::Xml::XmlReader,
        curmgr: *mut crate::System::Xml::XmlNamespaceManager,
        sinfo: *mut crate::System::Xml::Schema::SchemaInfo,
        targetNamspace: *mut crate::System::String,
        nameTable: *mut crate::System::Xml::XmlNameTable,
        schemaNames: *mut crate::System::Xml::Schema::SchemaNames,
        eventhandler: *mut crate::System::Xml::Schema::ValidationEventHandler,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    reader,
                    curmgr,
                    sinfo,
                    targetNamspace,
                    nameTable,
                    schemaNames,
                    eventhandler,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Schema::XdrBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEndChildFunction")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder_XdrEndChildFunction {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEndChildFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Xml::Schema::XdrBuilder_XdrEndChildFunction => "System.Xml.Schema"
    ."XdrBuilder/XdrEndChildFunction"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEndChildFunction")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder_XdrEndChildFunction {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEndChildFunction")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XdrBuilder_XdrEndChildFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEndChildFunction")]
impl crate::System::Xml::Schema::XdrBuilder_XdrEndChildFunction {
    pub fn Invoke(
        &mut self,
        builder: *mut crate::System::Xml::Schema::XdrBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (builder))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEndChildFunction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XdrBuilder_XdrEndChildFunction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder_XdrEntry {
    __cordl_parent: crate::System::Object,
    pub _Name: crate::System::Xml::Schema::SchemaNames_Token,
    pub _NextStates: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _Attributes: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Xml::Schema::XdrBuilder_XdrAttributeEntry,
    >,
    pub _InitFunc: *mut crate::System::Xml::Schema::XdrBuilder_XdrInitFunction,
    pub _BeginChildFunc: *mut crate::System::Xml::Schema::XdrBuilder_XdrBeginChildFunction,
    pub _EndChildFunc: *mut crate::System::Xml::Schema::XdrBuilder_XdrEndChildFunction,
    pub _AllowText: bool,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XdrBuilder_XdrEntry =>
    "System.Xml.Schema"."XdrBuilder/XdrEntry"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEntry")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder_XdrEntry {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEntry")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XdrBuilder_XdrEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEntry")]
impl crate::System::Xml::Schema::XdrBuilder_XdrEntry {
    pub fn _ctor(
        &mut self,
        n: crate::System::Xml::Schema::SchemaNames_Token,
        states: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        attributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Schema::XdrBuilder_XdrAttributeEntry,
        >,
        init: *mut crate::System::Xml::Schema::XdrBuilder_XdrInitFunction,
        begin: *mut crate::System::Xml::Schema::XdrBuilder_XdrBeginChildFunction,
        end: *mut crate::System::Xml::Schema::XdrBuilder_XdrEndChildFunction,
        fText: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (n, states, attributes, init, begin, end, fText))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        n: crate::System::Xml::Schema::SchemaNames_Token,
        states: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        attributes: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Xml::Schema::XdrBuilder_XdrAttributeEntry,
        >,
        init: *mut crate::System::Xml::Schema::XdrBuilder_XdrInitFunction,
        begin: *mut crate::System::Xml::Schema::XdrBuilder_XdrBeginChildFunction,
        end: *mut crate::System::Xml::Schema::XdrBuilder_XdrEndChildFunction,
        fText: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (n, states, attributes, init, begin, end, fText))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XdrBuilder_XdrEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrInitFunction")]
#[repr(C)]
#[derive(Debug)]
pub struct XdrBuilder_XdrInitFunction {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrInitFunction")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XdrBuilder_XdrInitFunction
    => "System.Xml.Schema"."XdrBuilder/XdrInitFunction"
);
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrInitFunction")]
impl std::ops::Deref for crate::System::Xml::Schema::XdrBuilder_XdrInitFunction {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrInitFunction")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XdrBuilder_XdrInitFunction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrInitFunction")]
impl crate::System::Xml::Schema::XdrBuilder_XdrInitFunction {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        builder: *mut crate::System::Xml::Schema::XdrBuilder,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (builder, obj))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+XdrBuilder+XdrInitFunction")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XdrBuilder_XdrInitFunction {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
