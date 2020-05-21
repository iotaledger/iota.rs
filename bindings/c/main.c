#include <stdint.h>
#include <stdio.h>
#include "include/iota.h"

int main() {
    iota_init("https://nodes.comnet.thetangle.org");

    /*
     * Generate new address & send transfers
     */
    uint8_t err;
    seed_t *seed = iota_seed_new();
    address_t *address = iota_address_new();
    transfers_t *transfers = iota_transfers_new();
    bundle_t *bundle = iota_bundle_new();
    iota_get_new_address(seed, 10, address);
    iota_transfers_add(transfers, address, 0);
    iota_send_transfers(seed, transfers, 10, bundle);
    iota_bundle_free(bundle);

    /*
     * Get node info
     */
    printf("\nFollowing node information is retrieved from iota.rs:\n");
    // This is the IRI API call get_nod_info.
    get_node_info_t *node_info = iota_get_node_info(&err);
    // We only define a few fields in the response struct. But this should give a glance how to use it.
    printf("Node name: %s\n", node_info->app_name);
    printf("Node version: %s\n", node_info->app_version);
    printf("Last milestone index: %d\n", node_info->latest_milestone_index);
    printf("err: %d\n", err);
    return 0;
}