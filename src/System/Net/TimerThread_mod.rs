#[cfg(feature = "System+Net+TimerThread+TimerNode+TimerState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimerNode_TimerThread_TimerState {
    #[default]
    Cancelled = 2i32,
    Fired = 1i32,
    Ready = 0i32,
    Sentinel = 3i32,
}
#[cfg(feature = "System+Net+TimerThread+TimerNode+TimerState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::TimerNode_TimerThread_TimerState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "TimerThread/TimerNode/TimerState";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "System+Net+TimerThread+TimerNode+TimerState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Net::TimerNode_TimerThread_TimerState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Net+TimerThread+TimerNode+TimerState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Net::TimerNode_TimerThread_TimerState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "System+Net+TimerThread+TimerNode+TimerState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Net::TimerNode_TimerThread_TimerState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "System+Net+TimerThread+TimerNode+TimerState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Net::TimerNode_TimerThread_TimerState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "System+Net+TimerThread")]
#[repr(C)]
#[derive(Debug)]
pub struct TimerThread {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+TimerThread")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::TimerThread {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "TimerThread";
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
#[cfg(feature = "System+Net+TimerThread")]
impl std::ops::Deref for crate::System::Net::TimerThread {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread")]
impl std::ops::DerefMut for crate::System::Net::TimerThread {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread")]
impl crate::System::Net::TimerThread {
    #[cfg(feature = "System+Net+TimerThread+Callback")]
    pub type Callback = crate::System::Net::TimerThread_Callback;
    #[cfg(feature = "System+Net+TimerThread+InfiniteTimer")]
    pub type InfiniteTimer = crate::System::Net::TimerThread_InfiniteTimer;
    #[cfg(feature = "System+Net+TimerThread+InfiniteTimerQueue")]
    pub type InfiniteTimerQueue = crate::System::Net::TimerThread_InfiniteTimerQueue;
    #[cfg(feature = "System+Net+TimerThread+Queue")]
    pub type Queue = crate::System::Net::TimerThread_Queue;
    #[cfg(feature = "System+Net+TimerThread+Timer")]
    pub type Timer = crate::System::Net::TimerThread_Timer;
    #[cfg(feature = "System+Net+TimerThread+TimerNode")]
    pub type TimerNode = crate::System::Net::TimerThread_TimerNode;
    #[cfg(feature = "System+Net+TimerThread+TimerQueue")]
    pub type TimerQueue = crate::System::Net::TimerThread_TimerQueue;
    pub fn CreateQueue(
        durationMilliseconds: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::TimerThread_Queue>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::TimerThread_Queue,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateQueue", (durationMilliseconds))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreateQueue(
        durationMilliseconds: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::TimerThread_Queue>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::TimerThread_Queue,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOrCreateQueue", (durationMilliseconds))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTickBetween(
        start: i32,
        end: i32,
        comparand: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsTickBetween", (start, end, comparand))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDomainUnload(
        sender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        e: quest_hook::libil2cpp::Gc<crate::System::EventArgs>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnDomainUnload", (sender, e))?;
        Ok(__cordl_ret.into())
    }
    pub fn Prod() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Prod", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StopTimerThread() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("StopTimerThread", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ThreadProc() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ThreadProc", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+TimerThread")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::TimerThread {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+TimerThread+Callback")]
#[repr(C)]
#[derive(Debug)]
pub struct TimerThread_Callback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "System+Net+TimerThread+Callback")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::TimerThread_Callback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "TimerThread/Callback";
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
#[cfg(feature = "System+Net+TimerThread+Callback")]
impl std::ops::Deref for crate::System::Net::TimerThread_Callback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+Callback")]
impl std::ops::DerefMut for crate::System::Net::TimerThread_Callback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+Callback")]
impl crate::System::Net::TimerThread_Callback {
    pub fn Invoke(
        &mut self,
        timer: quest_hook::libil2cpp::Gc<crate::System::Net::TimerThread_Timer>,
        timeNoticed: i32,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (timer, timeNoticed, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+TimerThread+Callback")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::TimerThread_Callback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+TimerThread+InfiniteTimer")]
#[repr(C)]
#[derive(Debug)]
pub struct TimerThread_InfiniteTimer {
    __cordl_parent: crate::System::Net::TimerThread_Timer,
    pub cancelled: i32,
}
#[cfg(feature = "System+Net+TimerThread+InfiniteTimer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::TimerThread_InfiniteTimer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "TimerThread/InfiniteTimer";
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
#[cfg(feature = "System+Net+TimerThread+InfiniteTimer")]
impl std::ops::Deref for crate::System::Net::TimerThread_InfiniteTimer {
    type Target = crate::System::Net::TimerThread_Timer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+InfiniteTimer")]
impl std::ops::DerefMut for crate::System::Net::TimerThread_InfiniteTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+InfiniteTimer")]
impl crate::System::Net::TimerThread_InfiniteTimer {
    pub fn Cancel(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Cancel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasExpired(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasExpired", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+TimerThread+InfiniteTimer")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::TimerThread_InfiniteTimer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+TimerThread+InfiniteTimerQueue")]
#[repr(C)]
#[derive(Debug)]
pub struct TimerThread_InfiniteTimerQueue {
    __cordl_parent: crate::System::Net::TimerThread_Queue,
}
#[cfg(feature = "System+Net+TimerThread+InfiniteTimerQueue")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::TimerThread_InfiniteTimerQueue {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "TimerThread/InfiniteTimerQueue";
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
#[cfg(feature = "System+Net+TimerThread+InfiniteTimerQueue")]
impl std::ops::Deref for crate::System::Net::TimerThread_InfiniteTimerQueue {
    type Target = crate::System::Net::TimerThread_Queue;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+InfiniteTimerQueue")]
impl std::ops::DerefMut for crate::System::Net::TimerThread_InfiniteTimerQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+InfiniteTimerQueue")]
impl crate::System::Net::TimerThread_InfiniteTimerQueue {
    pub fn CreateTimer(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Net::TimerThread_Callback>,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::TimerThread_Timer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::TimerThread_Timer,
        > = __cordl_object.invoke("CreateTimer", (callback, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+TimerThread+InfiniteTimerQueue")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::TimerThread_InfiniteTimerQueue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+TimerThread+Queue")]
#[repr(C)]
#[derive(Debug)]
pub struct TimerThread_Queue {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_DurationMilliseconds: i32,
}
#[cfg(feature = "System+Net+TimerThread+Queue")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::TimerThread_Queue {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "TimerThread/Queue";
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
#[cfg(feature = "System+Net+TimerThread+Queue")]
impl std::ops::Deref for crate::System::Net::TimerThread_Queue {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+Queue")]
impl std::ops::DerefMut for crate::System::Net::TimerThread_Queue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+Queue")]
impl crate::System::Net::TimerThread_Queue {
    pub fn CreateTimer(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Net::TimerThread_Callback>,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::TimerThread_Timer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::TimerThread_Timer,
        > = __cordl_object.invoke("CreateTimer", (callback, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        durationMilliseconds: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (durationMilliseconds))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        durationMilliseconds: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (durationMilliseconds))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Duration(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Duration", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+TimerThread+Queue")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::TimerThread_Queue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+TimerThread+Timer")]
#[repr(C)]
#[derive(Debug)]
pub struct TimerThread_Timer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_StartTimeMilliseconds: i32,
    pub m_DurationMilliseconds: i32,
}
#[cfg(feature = "System+Net+TimerThread+Timer")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::TimerThread_Timer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "TimerThread/Timer";
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
#[cfg(feature = "System+Net+TimerThread+Timer")]
impl std::ops::Deref for crate::System::Net::TimerThread_Timer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+Timer")]
impl std::ops::DerefMut for crate::System::Net::TimerThread_Timer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+Timer")]
impl crate::System::Net::TimerThread_Timer {
    pub fn Cancel(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Cancel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        durationMilliseconds: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (durationMilliseconds))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        durationMilliseconds: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (durationMilliseconds))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Expiration(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Expiration", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasExpired(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasExpired", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_StartTime(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_StartTime", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+TimerThread+Timer")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::TimerThread_Timer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+TimerThread+Timer")]
impl AsRef<crate::System::IDisposable> for crate::System::Net::TimerThread_Timer {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+TimerThread+Timer")]
impl AsMut<crate::System::IDisposable> for crate::System::Net::TimerThread_Timer {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Net+TimerThread+TimerNode")]
#[repr(C)]
#[derive(Debug)]
pub struct TimerThread_TimerNode {
    __cordl_parent: crate::System::Net::TimerThread_Timer,
    pub m_TimerState: crate::System::Net::TimerNode_TimerThread_TimerState,
    pub m_Callback: quest_hook::libil2cpp::Gc<crate::System::Net::TimerThread_Callback>,
    pub m_Context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_QueueLock: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub next: quest_hook::libil2cpp::Gc<crate::System::Net::TimerThread_TimerNode>,
    pub prev: quest_hook::libil2cpp::Gc<crate::System::Net::TimerThread_TimerNode>,
}
#[cfg(feature = "System+Net+TimerThread+TimerNode")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::TimerThread_TimerNode {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "TimerThread/TimerNode";
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
#[cfg(feature = "System+Net+TimerThread+TimerNode")]
impl std::ops::Deref for crate::System::Net::TimerThread_TimerNode {
    type Target = crate::System::Net::TimerThread_Timer;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+TimerNode")]
impl std::ops::DerefMut for crate::System::Net::TimerThread_TimerNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+TimerNode")]
impl crate::System::Net::TimerThread_TimerNode {
    #[cfg(feature = "System+Net+TimerThread+TimerNode+TimerState")]
    pub type TimerState = crate::System::Net::TimerNode_TimerThread_TimerState;
    pub fn Cancel(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Cancel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Fire(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Fire", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_1() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_TimerThread_Callback_Il2CppObject_i32_Il2CppObject0(
        callback: quest_hook::libil2cpp::Gc<crate::System::Net::TimerThread_Callback>,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        durationMilliseconds: i32,
        queueLock: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (callback, context, durationMilliseconds, queueLock))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_TimerThread_Callback_Il2CppObject_i32_Il2CppObject0(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Net::TimerThread_Callback>,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        durationMilliseconds: i32,
        queueLock: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (callback, context, durationMilliseconds, queueLock))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HasExpired(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasExpired", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Next(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::TimerThread_TimerNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::TimerThread_TimerNode,
        > = __cordl_object.invoke("get_Next", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Prev(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::TimerThread_TimerNode>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::TimerThread_TimerNode,
        > = __cordl_object.invoke("get_Prev", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Next(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Net::TimerThread_TimerNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Next", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Prev(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Net::TimerThread_TimerNode>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Prev", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+TimerThread+TimerNode")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::TimerThread_TimerNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+TimerThread+TimerQueue")]
#[repr(C)]
#[derive(Debug)]
pub struct TimerThread_TimerQueue {
    __cordl_parent: crate::System::Net::TimerThread_Queue,
    pub m_ThisHandle: crate::System::IntPtr,
    pub m_Timers: quest_hook::libil2cpp::Gc<crate::System::Net::TimerThread_TimerNode>,
}
#[cfg(feature = "System+Net+TimerThread+TimerQueue")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::TimerThread_TimerQueue {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "TimerThread/TimerQueue";
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
#[cfg(feature = "System+Net+TimerThread+TimerQueue")]
impl std::ops::Deref for crate::System::Net::TimerThread_TimerQueue {
    type Target = crate::System::Net::TimerThread_Queue;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+TimerQueue")]
impl std::ops::DerefMut for crate::System::Net::TimerThread_TimerQueue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+TimerThread+TimerQueue")]
impl crate::System::Net::TimerThread_TimerQueue {
    pub fn CreateTimer(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<crate::System::Net::TimerThread_Callback>,
        context: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::TimerThread_Timer>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::TimerThread_Timer,
        > = __cordl_object.invoke("CreateTimer", (callback, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn Fire(
        &mut self,
        nextExpiration: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Fire", (nextExpiration))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        durationMilliseconds: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (durationMilliseconds))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        durationMilliseconds: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (durationMilliseconds))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+TimerThread+TimerQueue")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::TimerThread_TimerQueue {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
