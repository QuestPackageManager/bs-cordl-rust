#[cfg(feature = "System+Xml+XPath+XPathException")]
#[repr(C)]
#[derive(Debug)]
pub struct XPathException {
    __cordl_parent: crate::System::SystemException,
    pub res: *mut quest_hook::libil2cpp::Il2CppString,
    pub args: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
    pub message: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+Xml+XPath+XPathException")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XPath::XPathException =>
    "System.Xml.XPath"."XPathException"
);
#[cfg(feature = "System+Xml+XPath+XPathException")]
impl std::ops::Deref for crate::System::Xml::XPath::XPathException {
    type Target = crate::System::SystemException;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XPath+XPathException")]
impl std::ops::DerefMut for crate::System::Xml::XPath::XPathException {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+XPath+XPathException")]
impl crate::System::Xml::XPath::XPathException {
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
    pub fn New_1() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppString_Exception2(
        message: *mut quest_hook::libil2cpp::Il2CppString,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (message, innerException))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppString_Il2CppArray3(
        res: *mut quest_hook::libil2cpp::Il2CppString,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (res, args))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppString_Il2CppArray_Exception4(
        res: *mut quest_hook::libil2cpp::Il2CppString,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        inner: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (res, args, inner))?;
        Ok(__cordl_object)
    }
    pub fn New_SerializationInfo_StreamingContext0(
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object)
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
    pub fn _ctor_Il2CppString_Exception2(
        &mut self,
        message: *mut quest_hook::libil2cpp::Il2CppString,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (message, innerException))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppString_Il2CppArray3(
        &mut self,
        res: *mut quest_hook::libil2cpp::Il2CppString,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (res, args))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppString_Il2CppArray_Exception4(
        &mut self,
        res: *mut quest_hook::libil2cpp::Il2CppString,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
        inner: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (res, args, inner))?;
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
    pub fn get_Message(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Message", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Xml+XPath+XPathException")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::XPath::XPathException {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
