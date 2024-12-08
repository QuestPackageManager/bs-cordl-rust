#[cfg(feature = "System+Xml+XmlException")]
#[repr(C)]
#[derive(Debug)]
pub struct XmlException {
    __cordl_parent: crate::System::SystemException,
    pub res: *mut crate::System::String,
    pub args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub lineNumber: i32,
    pub linePosition: i32,
    pub sourceUri: *mut crate::System::String,
    pub message: *mut crate::System::String,
}
#[cfg(feature = "System+Xml+XmlException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlException => "System.Xml"
    ."XmlException"
);
#[cfg(feature = "System+Xml+XmlException")]
impl std::ops::Deref for crate::System::Xml::XmlException {
    type Target = crate::System::SystemException;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlException")]
impl std::ops::DerefMut for crate::System::Xml::XmlException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XmlException")]
impl crate::System::Xml::XmlException {
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
    pub fn _ctor_String_Exception_i32_i32_String5(
        &mut self,
        message: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
        lineNumber: i32,
        linePosition: i32,
        sourceUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (message, innerException, lineNumber, linePosition, sourceUri),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray6(
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
    pub fn _ctor_String_String7(
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
    pub fn _ctor_String_String_String8(
        &mut self,
        res: *mut crate::System::String,
        arg: *mut crate::System::String,
        sourceUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (res, arg, sourceUri))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_String_IXmlLineInfo9(
        &mut self,
        res: *mut crate::System::String,
        arg: *mut crate::System::String,
        lineInfo: *mut crate::System::Xml::IXmlLineInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (res, arg, lineInfo))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray_IXmlLineInfo10(
        &mut self,
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        lineInfo: *mut crate::System::Xml::IXmlLineInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (res, args, lineInfo))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray_IXmlLineInfo_String11(
        &mut self,
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        lineInfo: *mut crate::System::Xml::IXmlLineInfo,
        sourceUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (res, args, lineInfo, sourceUri))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_String_i32_i32_12(
        &mut self,
        res: *mut crate::System::String,
        arg: *mut crate::System::String,
        lineNumber: i32,
        linePosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (res, arg, lineNumber, linePosition))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_String_i32_i32_String13(
        &mut self,
        res: *mut crate::System::String,
        arg: *mut crate::System::String,
        lineNumber: i32,
        linePosition: i32,
        sourceUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (res, arg, lineNumber, linePosition, sourceUri))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray_i32_i32_14(
        &mut self,
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        lineNumber: i32,
        linePosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (res, args, lineNumber, linePosition))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray_i32_i32_String15(
        &mut self,
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        lineNumber: i32,
        linePosition: i32,
        sourceUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (res, args, lineNumber, linePosition, sourceUri))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray_Exception_i32_i32_16(
        &mut self,
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        innerException: *mut crate::System::Exception,
        lineNumber: i32,
        linePosition: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (res, args, innerException, lineNumber, linePosition))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_Il2CppArray_Exception_i32_i32_String17(
        &mut self,
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        innerException: *mut crate::System::Exception,
        lineNumber: i32,
        linePosition: i32,
        sourceUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (res, args, innerException, lineNumber, linePosition, sourceUri),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_LinePosition(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LinePosition", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ResString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ResString", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LineNumber(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_LineNumber", ())?;
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
    pub fn New_String_Exception_i32_i32_String5(
        message: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
        lineNumber: i32,
        linePosition: i32,
        sourceUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (message, innerException, lineNumber, linePosition, sourceUri),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray6(
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (res, args))?;
        Ok(__cordl_object)
    }
    pub fn New_String_String7(
        res: *mut crate::System::String,
        arg: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (res, arg))?;
        Ok(__cordl_object)
    }
    pub fn New_String_String_String8(
        res: *mut crate::System::String,
        arg: *mut crate::System::String,
        sourceUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (res, arg, sourceUri))?;
        Ok(__cordl_object)
    }
    pub fn New_String_String_IXmlLineInfo9(
        res: *mut crate::System::String,
        arg: *mut crate::System::String,
        lineInfo: *mut crate::System::Xml::IXmlLineInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (res, arg, lineInfo))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray_IXmlLineInfo10(
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        lineInfo: *mut crate::System::Xml::IXmlLineInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (res, args, lineInfo))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray_IXmlLineInfo_String11(
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        lineInfo: *mut crate::System::Xml::IXmlLineInfo,
        sourceUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (res, args, lineInfo, sourceUri))?;
        Ok(__cordl_object)
    }
    pub fn New_String_String_i32_i32_12(
        res: *mut crate::System::String,
        arg: *mut crate::System::String,
        lineNumber: i32,
        linePosition: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (res, arg, lineNumber, linePosition))?;
        Ok(__cordl_object)
    }
    pub fn New_String_String_i32_i32_String13(
        res: *mut crate::System::String,
        arg: *mut crate::System::String,
        lineNumber: i32,
        linePosition: i32,
        sourceUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (res, arg, lineNumber, linePosition, sourceUri))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray_i32_i32_14(
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        lineNumber: i32,
        linePosition: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (res, args, lineNumber, linePosition))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray_i32_i32_String15(
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        lineNumber: i32,
        linePosition: i32,
        sourceUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (res, args, lineNumber, linePosition, sourceUri))?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray_Exception_i32_i32_16(
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        innerException: *mut crate::System::Exception,
        lineNumber: i32,
        linePosition: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (res, args, innerException, lineNumber, linePosition),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_String_Il2CppArray_Exception_i32_i32_String17(
        res: *mut crate::System::String,
        args: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
        innerException: *mut crate::System::Exception,
        lineNumber: i32,
        linePosition: i32,
        sourceUri: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (res, args, innerException, lineNumber, linePosition, sourceUri),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Xml+XmlException")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XmlException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
