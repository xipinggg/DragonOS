use ida::IdAllocator;

int_like!(KernelHandle, usize);

/// # socket的句柄管理组件
/// 它在smoltcp的SocketHandle上封装了一层，增加更多的功能。
/// 比如，在socket被关闭时，自动释放socket的资源，通知系统的其他组件。
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum GlobalSocketHandle {
    Smoltcp(smoltcp::iface::SocketHandle),
    Kernel(KernelHandle),
}

static KERNEL_HANDLE_IDA: IdAllocator = IdAllocator::new(0, usize::MAX);

impl GlobalSocketHandle {
    pub fn new_smoltcp_handle(handle: smoltcp::iface::SocketHandle) -> Self {
        return Self::Smoltcp(handle);
    }

    pub fn new_kernel_handle() -> Self {
        return Self::Kernel(KernelHandle::new(KERNEL_HANDLE_IDA.alloc().unwrap()));
    }

    pub fn smoltcp_handle(&self) -> Option<smoltcp::iface::SocketHandle> {
        if let Self::Smoltcp(sh) = *self {
            return Some(sh);
        }
        None
    }

    pub fn kernel_handle(&self) -> Option<KernelHandle> {
        if let Self::Kernel(kh) = *self {
            return Some(kh);
        }
        None
    }
}

impl From<smoltcp::iface::SocketHandle> for GlobalSocketHandle {
    fn from(handle: smoltcp::iface::SocketHandle) -> Self {
        return Self::new_smoltcp_handle(handle);
    }
}