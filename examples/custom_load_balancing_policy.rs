use anyhow::Result;
use rand::Rng;
use rand::rng;
use scylla::client::execution_profile::ExecutionProfile;
use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use scylla::cluster::ClusterState;
use scylla::cluster::NodeRef;
use scylla::policies::load_balancing::{LoadBalancingPolicy, RoutingInfo};
use scylla::routing::Shard;
use std::{env, sync::Arc};

/// Example load balancing policy that prefers nodes from favorite datacenter
/// This is, of course, very naive, as it is completely non token-aware.
/// For more realistic implementation, see [`DefaultPolicy`](scylla::policies::load_balancing::DefaultPolicy).
#[derive(Debug)]
struct CustomLoadBalancingPolicy {
    fav_datacenter_name: String,
}

fn with_random_shard(node: NodeRef) -> (NodeRef, Option<Shard>) {
    let nr_shards = node
        .sharder()
        .map(|sharder| sharder.nr_shards.get())
        .unwrap_or(1);
    (node, Some(rng().random_range(0..nr_shards) as Shard))
}

impl LoadBalancingPolicy for CustomLoadBalancingPolicy {
    fn pick<'a>(
        &'a self,
        _info: &'a RoutingInfo,
        cluster: &'a ClusterState,
    ) -> Option<(NodeRef<'a>, Option<Shard>)> {
        self.fallback(_info, cluster).next()
    }

    fn fallback<'a>(
        &'a self,
        _info: &'a RoutingInfo,
        cluster: &'a ClusterState,
    ) -> scylla::policies::load_balancing::FallbackPlan<'a> {
        let fav_dc_nodes = cluster
            .replica_locator()
            .unique_nodes_in_datacenter_ring(&self.fav_datacenter_name);

        match fav_dc_nodes {
            Some(nodes) => Box::new(nodes.iter().map(with_random_shard)),
            // If there is no dc with provided name, fallback to other datacenters
            None => Box::new(cluster.get_nodes_info().iter().map(with_random_shard)),
        }
    }

    fn name(&self) -> String {
        "CustomPolicy".to_string()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let uri = env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());

    let custom_load_balancing = CustomLoadBalancingPolicy {
        fav_datacenter_name: "PL".to_string(),
    };

    let profile = ExecutionProfile::builder()
        .load_balancing_policy(Arc::new(custom_load_balancing))
        .build();

    let _session: Session = SessionBuilder::new()
        .known_node(uri)
        .default_execution_profile_handle(profile.into_handle())
        .build()
        .await?;

    Ok(())
}
