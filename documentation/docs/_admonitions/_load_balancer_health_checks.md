:::note Load Balancers & Health Checks

If you use node load balancers then the health check may be useless as the follow-up API calls may be served by a
different node behind the load balancer that may not have been checked. You should be aware of this fact and trust that the
load balancer participates only with nodes that are in healthy state. The `iota.rs` library additionally supports
a management of internal node pool, so you can mimic a load-balancer-like behavior using this feature locally.

:::