# lab5

实际上认为自己完成得并不算太好。

`BANKER_ALGO` 作为银行家算法的全局实例，映射一组 KV 键值对来从一个 `pid` 映射到一个进程下的银行家算法。`BankerAlgorithm` 下的 `available` 是对整个进程的，而 `task_state` 则是对某个线程的资源管理。

```rust
pub struct BankerAlgorithm {
    /// Available map, (Resource) = available number
    available: BTreeMap<ResourceIdentifier, NumberOfResources>,
    /// (Task, Resource) = Resource {allocation, need}
    task_state: BTreeMap<TaskIdentifier, BTreeMap<ResourceIdentifier, TaskResourceState>>,
}
```