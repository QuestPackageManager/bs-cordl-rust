#[cfg(feature = "System+Xml+Schema+XmlSchemaException")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlSchemaException {
    __cordl_parent: crate::System::SystemException,
    pub res: *mut crate::System::String,
    pub args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub sourceUri: *mut crate::System::String,
    pub lineNumber: i32,
    pub linePosition: i32,
    pub sourceSchemaObject: *mut crate::System::Xml::Schema::XmlSchemaObject,
    pub message: *mut crate::System::String,
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Schema::XmlSchemaException =>
    "System.Xml.Schema"."XmlSchemaException"
);
#[cfg(feature = "System+Xml+Schema+XmlSchemaException")]
impl std::ops::Deref for crate::System::Xml::Schema::XmlSchemaException {
    type Target = crate::System::SystemException;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaException")]
impl std::ops::DerefMut for crate::System::Xml::Schema::XmlSchemaException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaException")]
impl crate::System::Xml::Schema::XmlSchemaException {
    pub fn SetSchemaObject(
        &mut self,
        source: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSchemaObject", (source))?;
        Ok(__cordl_ret)
    }
    pub fn get_SourceUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_SourceUri", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SourceSchemaObject(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Xml::Schema::XmlSchemaObject,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Xml::Schema::XmlSchemaObject = __cordl_object
            .invoke("get_SourceSchemaObject", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GetRes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_GetRes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LinePosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LinePosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Args(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_Args", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SerializationInfo_StreamingContext0(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, context))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String2(
        &mut self,
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Exception3(
        &mut self,
        message: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, innerException))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Exception_i32_i32_4(
        &mut self,
        message: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
        lineNumber: i32,
        linePosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, innerException, lineNumber, linePosition))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray5(
        &mut self,
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (res, args))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_String6(
        &mut self,
        res: *mut crate::System::String,
        arg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (res, arg))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_String_String_i32_i32_7(
        &mut self,
        res: *mut crate::System::String,
        arg: *mut crate::System::String,
        sourceUri: *mut crate::System::String,
        lineNumber: i32,
        linePosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (res, arg, sourceUri, lineNumber, linePosition))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_String_i32_i32_8(
        &mut self,
        res: *mut crate::System::String,
        sourceUri: *mut crate::System::String,
        lineNumber: i32,
        linePosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (res, sourceUri, lineNumber, linePosition))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray_String_i32_i32_9(
        &mut self,
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        sourceUri: *mut crate::System::String,
        lineNumber: i32,
        linePosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (res, args, sourceUri, lineNumber, linePosition))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_XmlSchemaObject10(
        &mut self,
        res: *mut crate::System::String,
        source: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (res, source))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_String_XmlSchemaObject11(
        &mut self,
        res: *mut crate::System::String,
        arg: *mut crate::System::String,
        source: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (res, arg, source))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray_XmlSchemaObject12(
        &mut self,
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        source: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (res, args, source))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray_Exception_String_i32_i32_XmlSchemaObject13(
        &mut self,
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        innerException: *mut crate::System::Exception,
        sourceUri: *mut crate::System::String,
        lineNumber: i32,
        linePosition: i32,
        source: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (res, args, innerException, sourceUri, lineNumber, linePosition, source),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_Message(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Message", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetSource_String_i32_i32_0(
        &mut self,
        sourceUri: *mut crate::System::String,
        lineNumber: i32,
        linePosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSource", (sourceUri, lineNumber, linePosition))?;
        Ok(__cordl_ret)
    }
    pub fn SetSource_XmlSchemaObject1(
        &mut self,
        source: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSource", (source))?;
        Ok(__cordl_ret)
    }
    pub fn get_LineNumber(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LineNumber", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetObjectData(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (info, context))?;
        Ok(__cordl_ret)
    }
    pub fn New_SerializationInfo_StreamingContext0(
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object)
    }
    pub fn New_1() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_String2(
        message: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Exception3(
        message: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, innerException))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Exception_i32_i32_4(
        message: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
        lineNumber: i32,
        linePosition: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, innerException, lineNumber, linePosition))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray5(
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (res, args))?;
        Ok(__cordl_object)
    }
    pub fn New_String_String6(
        res: *mut crate::System::String,
        arg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (res, arg))?;
        Ok(__cordl_object)
    }
    pub fn New_String_String_String_i32_i32_7(
        res: *mut crate::System::String,
        arg: *mut crate::System::String,
        sourceUri: *mut crate::System::String,
        lineNumber: i32,
        linePosition: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (res, arg, sourceUri, lineNumber, linePosition))?;
        Ok(__cordl_object)
    }
    pub fn New_String_String_i32_i32_8(
        res: *mut crate::System::String,
        sourceUri: *mut crate::System::String,
        lineNumber: i32,
        linePosition: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (res, sourceUri, lineNumber, linePosition))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray_String_i32_i32_9(
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        sourceUri: *mut crate::System::String,
        lineNumber: i32,
        linePosition: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (res, args, sourceUri, lineNumber, linePosition))?;
        Ok(__cordl_object)
    }
    pub fn New_String_XmlSchemaObject10(
        res: *mut crate::System::String,
        source: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (res, source))?;
        Ok(__cordl_object)
    }
    pub fn New_String_String_XmlSchemaObject11(
        res: *mut crate::System::String,
        arg: *mut crate::System::String,
        source: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (res, arg, source))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray_XmlSchemaObject12(
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        source: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (res, args, source))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray_Exception_String_i32_i32_XmlSchemaObject13(
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        innerException: *mut crate::System::Exception,
        sourceUri: *mut crate::System::String,
        lineNumber: i32,
        linePosition: i32,
        source: *mut crate::System::Xml::Schema::XmlSchemaObject,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (res, args, innerException, sourceUri, lineNumber, linePosition, source),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+Schema+XmlSchemaException")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Xml::Schema::XmlSchemaException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
