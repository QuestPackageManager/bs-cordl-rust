#[cfg(feature = "Newtonsoft+Json+Utilities+AsyncUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct AsyncUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+AsyncUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::AsyncUtils =>
    "Newtonsoft.Json.Utilities"."AsyncUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+AsyncUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::AsyncUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+AsyncUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::AsyncUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+AsyncUtils")]
impl crate::Newtonsoft::Json::Utilities::AsyncUtils {
    pub fn CancelIfRequestedAsync_CancellationToken0(
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CancelIfRequestedAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn CancelIfRequestedAsync_CancellationToken1<T>(
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CancelIfRequestedAsync", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromCanceled_CancellationToken0(
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromCanceled", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromCanceled_CancellationToken1<T>(
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromCanceled", (cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsCompletedSuccessfully(
        task: quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsCompletedSuccessfully", (task))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadAsync(
        reader: quest_hook::libil2cpp::Gc<crate::System::IO::TextReader>,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        index: i32,
        count: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<i32>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<i32>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadAsync", (reader, buffer, index, count, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToAsync(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task_1<bool>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<bool>,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("ToAsync", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteAsync_Il2CppArray_i32_i32_CancellationToken2(
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        start: i32,
        count: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteAsync", (writer, value, start, count, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteAsync_Il2CppString_CancellationToken1(
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteAsync", (writer, value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteAsync__cordl_char_CancellationToken0(
        writer: quest_hook::libil2cpp::Gc<crate::System::IO::TextWriter>,
        value: char,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Threading::Tasks::Task>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WriteAsync", (writer, value, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+AsyncUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::AsyncUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
