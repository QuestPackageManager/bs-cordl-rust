#[cfg(feature = "System+Threading+StackCrawlMark")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StackCrawlMark {
    #[default]
    LookForMe = 0i32,
    LookForMyCaller = 1i32,
    LookForMyCallersCaller = 2i32,
    LookForThread = 3i32,
}
#[cfg(feature = "System+Threading+StackCrawlMark")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::StackCrawlMark =>
    "System.Threading"."StackCrawlMark"
);
