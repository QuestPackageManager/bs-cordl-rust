#[cfg(feature = "System+Net+Http+HttpMessageInvoker")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpMessageInvoker {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub handler: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpMessageHandler>,
    pub disposeHandler: bool,
}
#[cfg(feature = "System+Net+Http+HttpMessageInvoker")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::Http::HttpMessageInvoker {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.Http";
    const CLASS_NAME: &'static str = "HttpMessageInvoker";
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
#[cfg(feature = "System+Net+Http+HttpMessageInvoker")]
impl std::ops::Deref for crate::System::Net::Http::HttpMessageInvoker {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+HttpMessageInvoker")]
impl std::ops::DerefMut for crate::System::Net::Http::HttpMessageInvoker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Http+HttpMessageInvoker")]
impl crate::System::Net::Http::HttpMessageInvoker {
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        handler: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpMessageHandler>,
        disposeHandler: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (handler, disposeHandler))?;
        Ok(__cordl_object.into())
    }
    pub fn SendAsync(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpRequestMessage>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpResponseMessage>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpResponseMessage>,
            >,
        > = __cordl_object.invoke("SendAsync", (request, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        handler: quest_hook::libil2cpp::Gc<crate::System::Net::Http::HttpMessageHandler>,
        disposeHandler: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (handler, disposeHandler))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+Http+HttpMessageInvoker")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Http::HttpMessageInvoker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+Http+HttpMessageInvoker")]
impl AsRef<crate::System::IDisposable> for crate::System::Net::Http::HttpMessageInvoker {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+Http+HttpMessageInvoker")]
impl AsMut<crate::System::IDisposable> for crate::System::Net::Http::HttpMessageInvoker {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
